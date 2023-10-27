use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};
use serde::{Deserialize, Serialize};
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
    // allow panic and dont require documenting - panics only when weights are NaN, which is never the case.
    #[allow(clippy::missing_panics_doc)]
    // Rulette selection
    pub fn selection(&self) -> Self {
        let weights: Vec<HeuristicScore> = self.entities.iter().map(Entity::forward).collect();

        let weights_min = *weights
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let positive_weights = weights
            .into_iter()
            .map(|x| x - weights_min)
            .collect::<Vec<f32>>();

        dbg!(&positive_weights);
        dbg!(&weights_min);
        let dist = WeightedIndex::new(positive_weights).unwrap();

        let rng = thread_rng();

        let new_population = dist
            .sample_iter(rng)
            .take(self.entities.len() / 2) // for now take 50% of the population
            .map(|idx| self.entities[idx].clone())
            .collect();

        Self {
            entities: new_population,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
        }
    }

    #[must_use]
    pub fn crossover(&self) -> Self {
        todo!();
    }

    #[must_use]
    pub fn mutation(&self) -> Self {
        todo!();
    }
}
