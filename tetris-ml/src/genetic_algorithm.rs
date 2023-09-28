use crate::population::Population;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    populations: Vec<Population>,
}

impl Model {
    #[must_use]
    pub fn new(n_entities: usize) -> Self {
        Self {
            populations: vec![Population::new(n_entities)],
        }
    }
}
