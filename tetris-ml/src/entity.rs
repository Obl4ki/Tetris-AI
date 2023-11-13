use std::collections::{HashSet, VecDeque};

use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use tetris_core::prelude::{Board, Game, Piece, Rotation};
use tetris_heuristics::{
    heuristics::{bumpyness, relative_diff, HeuristicScore},
    highest_block, holes_present,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref HEURISTICS: Vec<fn(&Board) -> HeuristicScore> =
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
        let dist = Uniform::from(0.0..10.0);
        let n_weights = HEURISTICS.len();
        Self {
            game: Game::new(),
            weights: dist.sample_iter(rng).take(n_weights).collect(),
        }
    }

    // allow panic and dont require documenting - panics only when weights are NaN, which is never the case.
    #[allow(clippy::missing_panics_doc)]
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

            piece_positions_visited.insert(popped_game.piece.clone());
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
            .zip(HEURISTICS.iter())
            .map(|(weight, h)| h(board) * weight)
            .sum()
    }
}

#[allow(clippy::cast_possible_truncation)]
fn lower_piece_before_branching(game: &mut Game) {
    let lowest_piece_block = game.piece.iter_blocks().map(|pos| pos.y).min().unwrap();
    let highest_grid_block = highest_block(&game.board) as i32;
    let dist_to_lower = lowest_piece_block - highest_grid_block;
    for _ in 0..dist_to_lower {
        game.go_down();
    }
}
