use serde::{Deserialize, Serialize};
use tetris_core::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    entities: Vec<Game>,
}

impl Population {
    #[must_use]
    pub fn initialization(&self) -> Self {
        todo!();
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
