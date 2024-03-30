use std::ops::RangeInclusive;
use std::{cmp::Ordering, sync::Arc};

use anyhow::Result;
use rand::{
    distributions::WeightedIndex, prelude::Distribution, seq::SliceRandom, thread_rng, Rng,
};

use rayon::prelude::*;
use tetris_core::prelude::*;

use crate::model_config::Config;
use crate::{agent::Agent, BranchingMode};
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone)]
pub struct Population {
    pub entities: Vec<Agent>,
    n_entities: usize,
    mutation_rate: f64,
    max_drops: Option<usize>,
    evaluator: fn(&Self),
}

impl Population {
    /// # Errors
    ///
    /// This function will return an error if validation of [`Config`] fails.
    /// See [`Config::validate`] for more details.
    pub fn new(config: &mut Config, evaluator: fn(&Self)) -> Result<Self> {
        config.validate()?;

        let heuristics_ref = Arc::new(std::mem::take(&mut config.heuristics_used));

        let entities: Vec<Agent> = (0..config.n_entities)
            .map(|_| Agent::new(Arc::clone(&heuristics_ref)))
            .collect();

        Ok(Self {
            entities,
            mutation_rate: config.mutation_rate,
            max_drops: config.max_drops,
            n_entities: config.n_entities,
            evaluator,
        })
    }

    #[must_use]
    pub fn advance_population(&self) -> Self {
        const WEIGHT_RANGE: RangeInclusive<f32> = -1.0..=1.0;
        self.clone()
            .evaluate()
            .selection()
            .crossover()
            .mutation(WEIGHT_RANGE)
    }

    #[must_use]
    pub fn finish_all_games(self) -> Self {
        let completed_population = self
            .entities
            .into_par_iter()
            .map(|entity| entity.play_for_n_turns_or_lose(self.max_drops, BranchingMode::default()))
            .progress_with_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                )
                .unwrap(),
            )
            .collect::<Vec<Agent>>();

        let finalized_population = Self {
            entities: completed_population,
            ..self
        };

        (self.evaluator)(&finalized_population);

        finalized_population
    }

    #[must_use]
    pub const fn fitness(entity: &Agent) -> f64 {
        entity.game.score.score as f64
    }

    fn evaluate(self) -> Self {
        self.restart_games().finish_all_games()
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
        let cross_method = |w1: f32, w2: f32| -> f32 {
            let alpha: f32 = rand::thread_rng().gen_range(0.0..1.0);

            alpha.mul_add(w1, (1.0 - alpha) * w2)
        };

        let offsprings = self
            .entities
            .into_par_iter()
            .chunks(2)
            .flat_map(|entities| {
                let first = &entities[0];
                let second = &entities[1];

                vec![
                    Agent::from_weights(
                        first
                            .weights
                            .iter()
                            .zip(second.weights.iter())
                            .map(|(&first_w, &second_w)| cross_method(first_w, second_w))
                            .collect(),
                        &first.heuristics,
                    )
                    .unwrap(),
                    Agent::from_weights(
                        first
                            .weights
                            .iter()
                            .zip(second.weights.iter())
                            .map(|(&first_w, &second_w)| cross_method(second_w, first_w))
                            .collect(),
                        &second.heuristics,
                    )
                    .unwrap(),
                ]
            })
            .collect::<Vec<Agent>>();

        Self {
            entities: offsprings,
            ..self
        }
    }

    #[must_use]
    pub fn mutation(self, weights_sampling_interval: RangeInclusive<f32>) -> Self {
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
            .for_each(|x: &mut Agent| x.game = Game::new());
        self
    }

    #[must_use]
    pub fn sorted_by_performance(&self) -> Vec<&Agent> {
        let mut entity_refs = self.entities.iter().collect::<Vec<_>>();

        entity_refs.sort_unstable_by(|x, y| Self::fitness(y).total_cmp(&Self::fitness(x)));
        entity_refs
    }

    #[must_use]
    pub fn get_best_entity(&self) -> &Agent {
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
