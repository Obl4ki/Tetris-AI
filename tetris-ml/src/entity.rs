use std::collections::{HashSet, VecDeque};

use anyhow::{bail, Result};
use rand::distributions::{Distribution, Uniform};
use std::sync::Arc;
use tetris_core::prelude::*;
use tetris_heuristics::prelude::*;

#[derive(Debug, Clone)]
pub struct Entity {
    pub game: Game,
    pub weights: Vec<f32>,
    pub heuristics: Arc<Vec<Heuristic>>,
}

impl Entity {
    #[must_use]
    pub fn new(heuristics: Arc<Vec<Heuristic>>) -> Self {
        let rng = rand::thread_rng();
        let dist = Uniform::from(0.0..10.0);
        let n_weights = heuristics.len();
        Self {
            game: Game::new(),
            weights: dist.sample_iter(rng).take(n_weights).collect(),
            heuristics,
        }
    }

    /// # Errors
    ///
    /// This function will return an error if weights length don't match the number of heuristics passed.
    pub fn from_weights(weights: Vec<f32>, heuristics: &[Heuristic]) -> Result<Self> {
        if weights.len() != heuristics.len() {
            bail!(
                "Weights size doesn't match: passed: {}, expected: {}",
                weights.len(),
                heuristics.len()
            );
        }

        Ok(Self {
            game: Game::new(),
            weights,
            heuristics: Arc::new(heuristics.to_vec()),
        })
    }

    #[must_use]
    pub fn next_best_state(&self, piece: Piece) -> Option<Self> {
        self.get_all_possible_next_game_states()
            .into_iter()
            .min_by(|a, b| {
                self.forward_with_board(&a.board)
                    .total_cmp(&self.forward_with_board(&b.board))
            })
            .map(|mut best_game| {
                best_game.piece = piece;
                Self {
                    game: best_game,
                    weights: self.weights.clone(),
                    heuristics: self.heuristics.clone(),
                }
            })
    }

    const ACTIONS: [fn(&mut Game); 6] = [
        Game::hard_drop,
        Game::go_down,
        Game::go_left,
        Game::go_right,
        |game| game.rotate(Rotation::Counterclockwise),
        |game| game.rotate(Rotation::Clockwise),
    ];

    /// Implementation of an algorithm to discover and collect all possible game states after 1 piece drop.
    /// Check every option with DFS algorithm using branching by every possible move.
    /// Next game states' boards are unique.
    ///
    /// Use hashset to delete pieces that were previously branched out to avoid repetition.
    #[must_use]
    pub fn get_all_possible_next_game_states(&self) -> Vec<Game> {
        let mut game = self.game.clone();

        let n_dropped_pieces = game.score.dropped_pieces;

        lower_piece_before_branching(&mut game);

        let mut games_stack = VecDeque::from([game]);
        let mut next_states = HashSet::new();
        let mut piece_positions_visited: HashSet<Piece> = HashSet::new();

        while let Some(popped_game) = games_stack.pop_front() {
            if popped_game.is_lost() {
                continue;
            }

            if popped_game.score.dropped_pieces == n_dropped_pieces + 1 {
                next_states.insert(popped_game);
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

            piece_positions_visited.insert(popped_game.piece);
        }

        next_states.into_iter().collect()
    }

    #[must_use]
    pub fn forward(&self) -> HeuristicScore {
        self.forward_with_board(&self.game.board)
    }

    #[must_use]
    pub fn forward_with_board(&self, board: &Board) -> HeuristicScore {
        self.weights
            .iter()
            .zip(self.heuristics.iter())
            .map(|(weight, h)| h(board) * weight)
            .sum()
    }
}

fn lower_piece_before_branching(game: &mut Game) {
    let lowest_piece_block = game.piece.iter_blocks().map(|pos| pos.y).min().unwrap();
    let highest_grid_block = highest_block(&game.board) as i32;
    let dist_to_lower = lowest_piece_block - highest_grid_block;
    for _ in 0..dist_to_lower {
        game.go_down();
    }
}
