use crate::population::Population;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    populations: Vec<Population>,
}

impl Model {
    #[must_use]
    pub fn new(n_entities: usize, crossover_rate: f32, mutation_rate: f32) -> Self {
        Self {
            populations: vec![Population::new(n_entities, crossover_rate, mutation_rate)],
        }
    }
}
