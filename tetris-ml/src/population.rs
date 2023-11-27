use std::{cmp::Ordering, ops, sync::Arc};

use anyhow::Result;
use rand::{
    distributions::WeightedIndex, prelude::Distribution, seq::SliceRandom, thread_rng, Rng,
};

use rayon::prelude::*;
use tetris_core::prelude::*;

use crate::entity::Entity;
use crate::model_config::Config;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone)]
pub struct Population {
    pub entities: Vec<Entity>,
    n_entities: usize,
    mutation_rate: f64,
    max_drops: Option<usize>,
}

impl Population {
    /// # Errors
    ///
    /// This function will return an error if validation of [`Config`] fails.
    /// See [`Config::validate`] for more details.
    pub fn new(config: &Config) -> Result<Self> {
        config.validate()?;

        let heuristics_ref = Arc::new(config.heuristics_used.clone());

        let entities: Vec<Entity> = (0..config.n_entities)
            .map(|_| Entity::new(Arc::clone(&heuristics_ref)))
            .collect();

        Ok(Self {
            entities,
            mutation_rate: config.mutation_rate,
            max_drops: config.max_drops,
            n_entities: config.n_entities,
        })
    }

    #[must_use]
    pub fn advance(&self) -> Self {
        self.clone()
            .restart_games()
            .finish_all_games()
            .crossover()
            .selection()
            .mutation(-1.0..1.0)
    }

    #[must_use]
    pub fn finish_all_games(self) -> Self {
        let completed_population = self
            .entities
            .into_par_iter()
            .map(|entity| {
                entity.play_for_n_turns_or_lose(self.max_drops)
            })
            .progress_with_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                )
                .unwrap(),
            )
            .collect::<Vec<Entity>>();

        Self {
            entities: completed_population,
            ..self
        }
    }

    #[must_use]
    pub const fn fitness(entity: &Entity) -> f64 {
        entity.game.score.score as f64
    }

    #[must_use]
    // Rulette selection
    pub fn selection(self) -> Self {
        let raw_probs: Vec<f64> = self.entities.iter().map(Self::fitness).collect();

        let norm_min = *raw_probs
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap();

        let norm_probs = raw_probs
            .into_iter()
            .map(|x| x - norm_min)
            .collect::<Vec<f64>>();

        let dist = WeightedIndex::new(norm_probs).unwrap();

        let rng = thread_rng();

        let new_population = dist
            .sample_iter(rng)
            .take(self.n_entities)
            .map(|idx| self.entities[idx].clone())
            .collect();

        Self {
            entities: new_population,
            ..self
        }
    }

    #[must_use]
    pub fn crossover(self) -> Self {
        let offsprings = self
            .entities
            .par_chunks(2)
            .flat_map(|entities| {
                let heuristics = &entities[0].heuristics;

                let w1 = entities[0].weights.iter();
                let w2 = entities[1].weights.iter();
                let entity1_w = w1.zip(w2).map(|(w1, w2)| 1.5 * w1 - 0.5 * w2).collect();

                let w1 = entities[0].weights.iter();
                let w2 = entities[1].weights.iter();
                let entity2_w = w1.zip(w2).map(|(w1, w2)| -0.5 * w1 + 1.5 * w2).collect();

                let w1 = entities[0].weights.iter();
                let w2 = entities[1].weights.iter();
                let entity3_w = w1.zip(w2).map(|(w1, w2)| 0.5 * w1 + 0.5 * w2).collect();

                vec![
                    Entity::from_weights(entity1_w, heuristics).unwrap(),
                    Entity::from_weights(entity2_w, heuristics).unwrap(),
                    Entity::from_weights(entity3_w, heuristics).unwrap(),
                ]
            })
            .collect::<Vec<_>>();

        let new_population = self.entities.into_iter().chain(offsprings).collect();

        Self {
            entities: new_population,
            ..self
        }
    }

    #[must_use]
    pub fn mutation(self, weights_sampling_interval: ops::Range<f32>) -> Self {
        let mut rng = thread_rng();

        let new_population = self
            .entities
            .into_iter()
            .map(|mut entity| {
                if rng.gen_bool(self.mutation_rate) {
                    if let Some(random_weight) = entity.weights.choose_mut(&mut rng) {
                        *random_weight += rng.gen_range(weights_sampling_interval.clone());
                    }
                }
                entity
            })
            .collect();

        Self {
            entities: new_population,
            ..self
        }
    }

    #[must_use]
    pub fn restart_games(mut self) -> Self {
        self.entities
            .par_iter_mut()
            .for_each(|x: &mut Entity| x.game = Game::new());
        self
    }

    #[must_use]
    pub fn sorted_by_performance(&self) -> Vec<&Entity> {
        let mut entity_refs = self.entities.iter().collect::<Vec<_>>();

        entity_refs.sort_unstable_by(|x, y| Self::fitness(y).total_cmp(&Self::fitness(x)));
        entity_refs
    }

    #[must_use]
    pub fn get_best_entity(&self) -> &Entity {
        self.sorted_by_performance()
            .first()
            .expect("Population cannot be empty.")
    }

    #[must_use]
    pub fn mean_fitness(&self) -> f64 {
        self.entities.iter().map(Self::fitness).sum::<f64>() / self.entities.len() as f64
    }

    #[must_use]
    pub fn lowest_fitness(&self) -> f64 {
        let worst = self
            .entities
            .iter()
            .min_by(|x, y| Self::fitness(x).total_cmp(&Self::fitness(y)))
            .unwrap();
        Self::fitness(worst)
    }

    #[must_use]
    pub fn biggest_fitness(&self) -> f64 {
        let best = self
            .entities
            .iter()
            .max_by(|x, y| Self::fitness(x).total_cmp(&Self::fitness(y)))
            .unwrap();
        Self::fitness(best)
    }

    #[must_use]
    pub fn median_fitness(&self) -> f64 {
        let take_n = if self.entities.len() & 1 == 0 { 1 } else { 2 };
        self.entities
            .iter()
            .skip(self.entities.len() / 2)
            .take(take_n)
            .map(Self::fitness)
            .sum::<f64>()
            / take_n as f64
    }
}
