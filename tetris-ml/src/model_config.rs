use anyhow::{bail, Result};
use tetris_heuristics::Heuristic;

#[derive(Debug, Clone)]
pub struct Config {
    pub n_entities: usize,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
    pub max_drops: Option<usize>,
    pub heuristics_used: Vec<Heuristic>,
}

impl Config {
    /// Validate the state of this [`Config`].
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - `n_entities` is 0
    /// - `n_entities` is an odd number (because current implementation doesn't work with odd number of entities)
    /// - `heuristics_used` used are empty
    pub fn validate(&self) -> Result<()> {
        if self.n_entities == 0 {
            bail!("N entities cannot be 0.")
        }
        if self.n_entities & 1 != 0 {
            bail!("N entities must be even for GA to work in a stable way.")
        }
        if self.heuristics_used.is_empty() {
            bail!("Heuristics cannot be empty!")
        }

        Ok(())
    }
}
