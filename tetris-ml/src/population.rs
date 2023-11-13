use std::cmp::Ordering;

use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tetris_core::prelude::Piece;
use tetris_heuristics::heuristics::HeuristicScore;

use crate::entity::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub entities: Vec<Entity>,
    crossover_rate: f32,
    mutation_rate: f32,
}

impl Population {
    #[must_use]
    pub fn new(n_entities: usize, crossover_rate: f32, mutation_rate: f32) -> Self {
        let entities = (0..n_entities).map(|_| Entity::new()).collect();
        Self {
            entities,
            crossover_rate,
            mutation_rate,
        }
    }

    #[must_use]
    pub fn advance(&self) -> Self {
        self.clone().selection().crossover().mutation()
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub const fn fitness(entity: &Entity) -> f64 {
        (entity.game.score.dropped_pieces) as f64
    }

    #[must_use]
    // Rulette selection
    #[allow(clippy::missing_panics_doc)]
    pub fn selection(self) -> Self {
        let completed_population = self
            .entities
            .into_par_iter()
            .map(|mut entity| {
                while let Some(next_entity) = entity.next_best_state(Piece::random()) {
                    entity = next_entity;
                }
                println!("Entity done with score: {:?}", entity.game.score);
                entity
            })
            .collect::<Vec<Entity>>();

        let raw_probs: Vec<f64> = completed_population.iter().map(Self::fitness).collect();

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
            .take(completed_population.len() / 2)
            .map(|idx| completed_population[idx].clone())
            .collect();

        Self {
            entities: new_population,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
        }
    }

    #[must_use]
    pub fn crossover(self) -> Self {
        self
    }

    #[must_use]
    pub fn mutation(self) -> Self {
        self
    }
}
