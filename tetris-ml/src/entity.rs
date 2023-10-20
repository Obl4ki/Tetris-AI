use std::collections::{HashSet, VecDeque};

use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use tetris_core::prelude::{Coord, Game, Piece, Rotation};
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

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
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

    const ACTIONS: [&dyn Fn(&mut Game); 6] = [
        &Game::go_down,
        &Game::go_left,
        &Game::go_right,
        &Game::hard_drop,
        &|game| game.rotate(Rotation::Counterclockwise),
        &|game| game.rotate(Rotation::Clockwise),
    ];

    /// Implementation of an algorithm to discover and collect all possible game states after 1 piece drop.
    /// At first, the piece is dropped one by one until it is exactly 1 level above the highest block in the grid.
    /// Then we check every option with DFS algorithm using branching by every possible move.
    /// Next game states' boards are unique.
    ///
    /// Use hashset to delete pieces that were previously branched out to avoid repetition.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_all_possible_next_game_states(&self) -> Vec<Game> {
        let mut game = self.game.clone();
        let max_height = highest_block(&game);
        let mut next_states = vec![];
        while game
            .piece
            .iter_blocks()
            .any(|pos| pos.y <= max_height as i32)
        {
            game.go_down();
        }

        let mut games_stack = VecDeque::from([game]);
        let mut piece_positions_visited: HashSet<Piece> = HashSet::new();
        while let Some(popped_game) = games_stack.pop_front() {
            if popped_game.piece_recently_dropped {
                next_states.push(popped_game);
                continue;
            }

            if piece_positions_visited.contains(&popped_game.piece) {
                continue;
            }

            for action in Self::ACTIONS {
                let mut branched_game = popped_game.clone();
                action(&mut branched_game);

                games_stack.push_front(branched_game);
            }

            piece_positions_visited.insert(popped_game.piece.clone());
        }

        next_states
    }

    #[must_use]
    pub fn calculate_weighted_heuristic(&self) -> HeuristicScore {
        self.weights
            .iter()
            .zip(HEURISTICS.iter())
            .map(|(weight, h)| h(&self.game) * weight)
            .sum()
    }
}
