use serde::{Deserialize, Serialize};

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
    pub fn selection(&self) -> Self {
        todo!();
    }

    #[must_use]
    pub fn crossover(&self) -> Self {
        todo!();
    }

    #[must_use]
    pub fn mutation(&self) -> Self {
        todo!();
    }

    #[must_use]
    pub fn sort_by_score(&mut self) -> Self {
        // self.entities.iter().map(|entity| (entity, ))
        todo!();
    }
}
