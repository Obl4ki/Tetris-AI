use crate::population::Population;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    populations: Vec<Population>,
}
