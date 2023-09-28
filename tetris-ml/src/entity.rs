use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use tetris_core::prelude::Game;
use tetris_heuristics::{
    heuristics::{bumpyness, relative_diff, HeuristicScore},
    highest_block, holes_present,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref HEURISTICS: Vec<fn(&Game) -> HeuristicScore> =
        vec![holes_present, highest_block, bumpyness, relative_diff];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub game: Game,
    pub weights: Vec<f32>,
}

impl Entity {
    #[must_use]
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        let dist = Uniform::from(0.0..1.0);
        let n_weights = HEURISTICS.len();
        Self {
            game: Game::new(),
            weights: dist.sample_iter(rng).take(n_weights).collect(),
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}
