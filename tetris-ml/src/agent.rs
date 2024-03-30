use anyhow::{bail, Result};
use rand::distributions::{Distribution, Uniform};
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

use tetris_core::prelude::*;
use tetris_heuristics::prelude::*;

use crate::BranchingMode;

#[derive(Debug, Clone)]
pub struct Agent {
    pub game: Game,
    pub weights: Vec<f32>,
    pub heuristics: Arc<Vec<Heuristic>>,
}

impl Agent {
    #[must_use]
    pub fn new(heuristics: Arc<Vec<Heuristic>>) -> Self {
        let rng = rand::thread_rng();
        let dist = Uniform::from(-1.0..1.0);
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
    pub const fn fitness(&self) -> f64 {
        self.game.score.score as f64
    }

    pub fn make_a_move(&mut self, branching_mode: BranchingMode) {
        if let Some(next_state) = self.next_best_state(branching_mode) {
            self.game = next_state;
        }
    }

    #[must_use]
    pub fn play_for_n_turns_or_lose(
        self,
        n_turns: Option<usize>,
        branching_mode: BranchingMode,
    ) -> Self {
        let mut entity = self;
        for _ in 0..n_turns.unwrap_or(usize::MAX) {
            entity.make_a_move(branching_mode);
            if entity.game.is_lost() {
                break;
            }
        }
        entity
    }

    #[must_use]
    pub fn play_until_lost(self, branching_mode: BranchingMode) -> Self {
        self.play_for_n_turns_or_lose(None, branching_mode)
    }

    #[must_use]
    pub fn next_best_state(&self, branching_mode: BranchingMode) -> Option<Game> {
        let next_state = match branching_mode {
            BranchingMode::Current => Self::get_all_possible_next_game_states(self.game.clone())
                .into_iter()
                .min_by(|a, b| {
                    self.forward_with_board(&a.board)
                        .total_cmp(&self.forward_with_board(&b.board))
                })?,

            BranchingMode::CurrentAndNext => {
                Self::get_all_possible_next_game_states(self.game.clone())
                    .into_iter()
                    .map(|game| vec![game])
                    .flat_map(|path: Vec<Game>| {
                        Self::get_all_possible_next_game_states(path[0].clone())
                            .into_iter()
                            .map(move |next| vec![path[0].clone(), next])
                    })
                    .min_by(|path1, path2| {
                        self.forward_with_board(&path1.last().unwrap().board)
                            .total_cmp(&self.forward_with_board(&path2.last().unwrap().board))
                    })?
                    .first()?
                    .clone()
            }
        };

        Some(next_state)
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
    pub fn get_all_possible_next_game_states(mut game: Game) -> Vec<Game> {
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
