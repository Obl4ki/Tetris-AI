use std::{cmp::Ordering, ops};

use rand::{
    distributions::WeightedIndex, prelude::Distribution, seq::SliceRandom, thread_rng, Rng,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tetris_core::prelude::*;

use crate::entity::Entity;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub entities: Vec<Entity>,
    crossover_rate: f64,
    mutation_rate: f64,
    n_weights: usize,
    max_drops: Option<usize>,
}

impl Population {
    #[must_use]
    pub fn new(
        n_entities: usize,
        crossover_rate: f64,
        mutation_rate: f64,
        max_drops: Option<usize>,
    ) -> Self {
        let entities: Vec<Entity> = (0..n_entities).map(|_| Entity::new()).collect();
        Self {
            n_weights: entities[0].weights.len(),
            entities,
            crossover_rate,
            mutation_rate,
            max_drops,
        }
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
    pub fn fitness(&self, entity: &Entity) -> f64 {
        let dropped = entity.game.score.dropped_pieces;
        let cleared = entity.game.score.cleared_rows;
        if dropped == 0 {
            return 0.;
        }

        let mut score = (cleared / dropped + 1) as f64;

        // apply penalty for not reaching the goal of n drops
        if let Some(max_drops) = self.max_drops {
            score /= (max_drops / dropped) as f64;
        }

        score
    }

    #[must_use]
    // Rulette selection
    #[allow(clippy::missing_panics_doc)]
    pub fn selection(self) -> Self {
        let raw_probs: Vec<f64> = self
            .entities
            .iter()
            .map(|entity| self.fitness(entity))
            .collect();

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

                        Entity {
                            game: Game::new(),
                            weights: new_weights,
                        }
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

        entity_refs.sort_unstable_by(|x, y| self.fitness(y).total_cmp(&self.fitness(x)));
        entity_refs
    }
}
