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
    crossover_rate: f64,
    mutation_rate: f64,
    n_weights: usize,
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
            n_weights: entities[0].weights.len(),
            entities,
            crossover_rate: config.crossover_rate,
            mutation_rate: config.mutation_rate,
            max_drops: config.max_drops,
        })
    }

    #[must_use]
    pub fn advance(&self) -> Self {
        self.clone()
            .restart_games()
            .finish_all_games()
            .selection()
            .crossover()
            .mutation(0.0..10.0)
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn finish_all_games(self) -> Self {
        let completed_population = self
            .entities
            .into_par_iter()
            .map(|mut entity| {
                for _ in 0..self.max_drops.unwrap_or(usize::MAX) {
                    if let Some(next_entity) = entity.next_best_state(Piece::random()) {
                        entity = next_entity;
                    } else {
                        break;
                    };
                }
                entity
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
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            n_weights: self.n_weights,
            max_drops: self.max_drops,
        }
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub const fn fitness(entity: &Entity) -> f64 {
        entity.game.score.score as f64
    }

    #[must_use]
    // Rulette selection
    #[allow(clippy::missing_panics_doc)]
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
            .take(self.entities.len() / 2)
            .map(|idx| self.entities[idx].clone())
            .collect();

        Self {
            entities: new_population,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            n_weights: self.n_weights,
            max_drops: self.max_drops,
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn crossover(self) -> Self {
        let offsprings = self
            .entities
            .par_chunks(2)
            .flat_map(|entities| {
                (0..2)
                    .map(|_| {
                        let split_idx = rand::thread_rng().gen_range(0..self.n_weights);
                        let new_weights: Vec<f32> = entities[0]
                            .weights
                            .iter()
                            .take(split_idx)
                            .chain(entities[1].weights.iter().skip(split_idx))
                            .copied()
                            .collect();

                        Entity::from_weights(new_weights, &entities[0].heuristics).unwrap()
                    })
                    .collect::<Vec<Entity>>()
            })
            .collect::<Vec<_>>();

        let new_population = self.entities.into_iter().chain(offsprings).collect();

        Self {
            entities: new_population,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            n_weights: self.n_weights,
            max_drops: self.max_drops,
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
                        *random_weight = rng.gen_range(weights_sampling_interval.clone());
                    }
                }
                entity
            })
            .collect();

        Self {
            entities: new_population,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            n_weights: self.n_weights,
            max_drops: self.max_drops,
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
    #[allow(clippy::missing_panics_doc)]
    pub fn get_best_entity(&self) -> &Entity {
        self.sorted_by_performance()
            .first()
            .expect("Population cannot be empty.")
    }
}
