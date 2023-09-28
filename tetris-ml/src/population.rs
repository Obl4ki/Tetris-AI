use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    entities: Vec<Entity>,
}

impl Population {
    #[must_use]
    pub fn new(n_entities: usize) -> Self {
        let entities = (0..n_entities).map(|_| Entity::new()).collect();
        Self { entities }
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
        todo!();
    }
}
