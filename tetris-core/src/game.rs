use super::{core_types, direction::Direction};
use crate::collision::Collision;
use crate::core_types::BlockType;
use crate::piece::Piece;
use itertools::Itertools;

/// Main game struct, used to instantiate the game.
#[derive(Debug)]
pub struct Game {
    pub board: Vec<Vec<Option<BlockType>>>,
    pub falling_piece: Piece,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let board = vec![vec![None; height]; width];
        Self {
            board: board.clone(),
            falling_piece: Piece::new(core_types::BlockType::Z, &board).unwrap(),
            width,
            height,
        }
    }

    /// Iterate through every block in grid
    pub fn iter_board(&self) -> impl Iterator<Item = (usize, usize, BlockType)> + '_ {
        (0..self.width)
            .cartesian_product(0..self.height)
            .filter_map(|(x, y)| {
                let val = self.board[x][y];
                val.map(|block_type| (x, y, block_type))
            })
    }

    pub fn iter_piece_blocks(&self) -> impl Iterator<Item = (i32, i32, BlockType)> + '_ {
        self.falling_piece
            .iter_blocks(self)
            .map(move |(x, y)| (x, y, self.falling_piece.block_type))
    }

    /// Check if after the move in the specified direction there will
    /// be any collision.
    pub fn get_collision_after_move(&self, dir: Direction) -> Collision {
        let (dx, dy) = dir.into();

        for (x, y, _) in self.iter_piece_blocks() {
            let xx = x + dx;
            let yy = y + dy;

            if xx < 0 {
                return Collision::LeftBorder;
            }

            if xx as usize >= self.width {
                return Collision::RightBorder;
            }

            if yy < 0 {
                return Collision::BottomBorder;
            }

            let row = self.board.get((x + dx) as usize).unwrap();

            let target_block = row.get((y + dy) as usize).unwrap();

            if let Some(_block) = target_block {
                return Collision::Block;
            }
        }

        Collision::None
    }

    pub fn go_left(&mut self) {
        if self.get_collision_after_move(Direction::Left) == Collision::None {
            self.falling_piece.anchor_point.0 -= 1;
        }
    }

    pub fn go_right(&mut self) {
        if self.get_collision_after_move(Direction::Right) == Collision::None {
            self.falling_piece.anchor_point.0 += 1;
        }
    }

    pub fn go_down(&mut self) {
        if self.get_collision_after_move(Direction::Down) == Collision::None {
            self.falling_piece.anchor_point.1 -= 1;
        }
    }

    pub fn hard_drop(&mut self) {
        while self.get_collision_after_move(Direction::Down) == Collision::None {
            self.falling_piece.anchor_point.1 -= 1;
        }
    }

    pub fn rotate_cw(&mut self) {
        // TODO test for Piece out of border
        // TODO offset out of border
        self.falling_piece.rotate();
    }

    pub fn rotate_ccw(&mut self) {
        self.falling_piece.rotate_ccw();
    }
}
