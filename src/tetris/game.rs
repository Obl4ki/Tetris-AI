use super::core_types;
use crate::tetris::core_types::BlockType;
use crate::tetris::piece::Piece;
use anyhow::{Context, Result};
use itertools::Itertools;

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
            falling_piece: Piece::new(core_types::BlockType::I, &board).unwrap(),
            width,
            height,
        }
    }

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
            .iter_blocks(&self.board)
            .map(move |(x, y)| (x, y, self.falling_piece.block_type))
    }

    /// Piece-Piece collision checker for SRS algorithm.
    pub fn is_colliding(&self) -> Result<bool> {
        for (x, y) in self.falling_piece.iter_blocks(&self.board) {
            let row = self
                .board
                .get(x as usize)
                .context(format!("X value of ({} {}) is off the grid", &x, &y))?;

            let target_block = row
                .get(y as usize)
                .context(format!("Block with ({} {}) is off the grid", &x, &y))?;

            if let Some(_block) = target_block {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn go_left(&mut self) {
        if self
            .falling_piece
            .iter_blocks(&self.board)
            .any(|(x, _)| x == 0)
        {
            return;
        }

        self.falling_piece.anchor_point.0 -= 1;

        if self.is_colliding().unwrap() {
            self.falling_piece.anchor_point.0 += 1;
        }
    }
    pub fn go_right(&mut self) {
        if self
            .falling_piece
            .iter_blocks(&self.board)
            .any(|(x, _)| x == self.width as i32 - 1)
        {
            return;
        }

        self.falling_piece.anchor_point.0 += 1;

        if self.is_colliding().unwrap() {
            self.falling_piece.anchor_point.0 -= 1;
        }
    }
    pub fn fall_by_one(&mut self) {
        if self
            .falling_piece
            .iter_blocks(&self.board)
            .any(|(_, y)| y == 0)
        {
            return;
        }

        self.falling_piece.anchor_point.1 -= 1;

        if self.is_colliding().unwrap() {
            self.falling_piece.anchor_point.1 += 1;
        }
    }

    pub fn drop(&mut self) {
        todo!()
    }

    pub fn rotate_cw(&mut self) {
        self.falling_piece.rotate();
    }

    pub fn rotate_ccw(&mut self) {
        self.falling_piece.rotate_ccw();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const test_width: usize = 10;
    const test_height: usize = 20;

    #[test]
    fn test_move_left_not_obstructed() {
        let mut game = Game::new(test_width, test_height);

        let positions_before = game.falling_piece.iter_blocks(&game.board).collect_vec();
        game.go_left();
        let positions_after = game.falling_piece.iter_blocks(&game.board).collect_vec();

        for (pos1, pos2) in itertools::zip_eq(positions_before, positions_after) {
            assert_ne!(
                pos1, pos2,
                "Position {pos1:?} should not be equal to {pos2:?}"
            );
        }
    }

    // #[test]
    // fn test_move_left_grid_margin_obstructed() {
    //     let mut game = Game::new(test_width, test_height);
    // }

    // #[test]
    // fn test_collision_block_to_block() {
    //     // one block overlap between falling piece's block and board block
    //     let game = GameBuilder::new(10, 20)
    //         .add_blocks(vec![(0, 0), (0, 1), (0, 2)], BlockType::SShape)
    //         .set_falling_piece(Piece {
    //             block_type: BlockType::OShape,
    //             anchor_point: (0, 0),
    //             blocks: vec![(0, 2), (1, 2), (2, 2)],
    //             rotation_idx: 0,
    //         })
    //         .compile();

    //     assert!(game.is_colliding().unwrap());
    // }

    // #[test]
    // fn test_move_left_block_collision() {
    //     let mut game = GameBuilder::new(10, 20)
    //         .add_blocks(
    //             vec![(0, 0), (1, 0), (2, 0), (1, 0), (1, 1), (2, 0)],
    //             BlockType::IShape,
    //         )
    //         .set_falling_piece(Piece {
    //             block_type: BlockType::OShape,
    //             anchor_point: (4, 0),
    //             blocks: vec![(0, 0), (0, 1)],

    //             rotation_idx: 0,
    //         })
    //         .compile();

    //     // it should move left as usual (there is free space)
    //     let piece_positions = game.falling_piece.clone();
    //     game.go_left();

    //     for (pos1, pos2) in piece_positions
    //         .iter_blocks()
    //         .zip(game.falling_piece.iter_blocks())
    //     {
    //         assert_ne!(pos1, pos2);
    //     }

    //     let piece_positions = game.falling_piece.clone();
    //     game.go_left();

    //     // only one pair of blocks colided but there should be no change after the move
    //     for (pos1, pos2) in piece_positions
    //         .iter_blocks()
    //         .zip(game.falling_piece.iter_blocks())
    //     {
    //         assert_eq!(pos1, pos2);
    //     }
    // }

    // #[test]
    // fn test_move_right_block_collision() {
    //     let mut game = GameBuilder::new(10, 20)
    //         .add_blocks(
    //             vec![(9, 0), (9, 1), (9, 2), (8, 0), (8, 1), (7, 0)],
    //             BlockType::IShape,
    //         )
    //         .set_falling_piece(Piece {
    //             block_type: BlockType::OShape,
    //             anchor_point: (6, 1),
    //             blocks: vec![(0, 0), (0, 1)],

    //             rotation_idx: 0,
    //         })
    //         .compile();

    //     // it should move left as usual (there is free space)
    //     let piece_positions = game.falling_piece.clone();
    //     game.go_right();

    //     for (pos1, pos2) in piece_positions
    //         .iter_blocks()
    //         .zip(game.falling_piece.iter_blocks())
    //     {
    //         assert_ne!(pos1, pos2);
    //     }

    //     let piece_positions = game.falling_piece.clone();
    //     game.go_right();

    //     // only one pair of blocks colided but there should be no change after the move
    //     for (pos1, pos2) in piece_positions
    //         .iter_blocks()
    //         .zip(game.falling_piece.iter_blocks())
    //     {
    //         assert_eq!(pos1, pos2);
    //     }
    // }

    // #[test]
    // fn test_left_board_margin_collision() {
    //     let mut game = GameBuilder::new(10, 20)
    //         .set_falling_piece(Piece {
    //             block_type: BlockType::OShape,
    //             anchor_point: (0, 0),
    //             blocks: vec![(0, 0), (0, 1)],
    //             rotation_idx: 0,
    //         })
    //         .compile();

    //     let piece_positions = game.falling_piece.clone();
    //     game.go_left();

    //     for (pos1, pos2) in piece_positions
    //         .iter_blocks()
    //         .zip(game.falling_piece.iter_blocks())
    //     {
    //         assert_eq!(pos1, pos2);
    //     }
    // }

    // #[test]
    // fn test_right_board_margin_collision() {
    //     let mut game = GameBuilder::new(10, 20)
    //         .set_falling_piece(Piece {
    //             block_type: BlockType::OShape,
    //             anchor_point: (9, 0),
    //             blocks: vec![(0, 0), (0, 1)],
    //             rotation_idx: 0,
    //         })
    //         .compile();

    //     let piece_positions = game.falling_piece.clone();
    //     game.go_right();

    //     for (pos1, pos2) in piece_positions
    //         .iter_blocks()
    //         .zip(game.falling_piece.iter_blocks())
    //     {
    //         assert_eq!(pos1, pos2);
    //     }
    // }

    // #[test]
    // fn test_fall_by_one() {
    //     let mut game = GameBuilder::new(10, 20)
    //         .set_falling_piece(Piece {
    //             block_type: BlockType::OShape,
    //             anchor_point: (0, 4),
    //             blocks: vec![(0, 0), (0, 1)],
    //             rotation_idx: 0,
    //         })
    //         .compile();

    //     for _ in 0..4 {
    //         let piece_positions = game.falling_piece.clone();
    //         game.fall_by_one();

    //         for (pos1, pos2) in piece_positions
    //             .iter_blocks()
    //             .zip(game.falling_piece.iter_blocks())
    //         {
    //             assert_ne!(pos1, pos2);
    //         }
    //     }

    //     let piece_positions = game.falling_piece.clone();
    //     game.fall_by_one();

    //     for (pos1, pos2) in piece_positions
    //         .iter_blocks()
    //         .zip(game.falling_piece.iter_blocks())
    //     {
    //         assert_eq!(pos1, pos2);
    //     }
    // }

    // #[test]
    // fn test_fall_by_one_block_collision() {
    //     let mut game = GameBuilder::new(10, 20)
    //         .set_falling_piece(Piece {
    //             block_type: BlockType::OShape,
    //             anchor_point: (1, 4),
    //             blocks: vec![(0, 0), (0, 1)],
    //             rotation_idx: 0,
    //         })
    //         .add_blocks(
    //             vec![(0, 0), (0, 1), (1, 0), (2, 0), (2, 1), (2, 2)],
    //             BlockType::IShape,
    //         )
    //         .compile();

    //     for _ in 0..3 {
    //         let piece_positions = game.falling_piece.clone();
    //         game.fall_by_one();

    //         for (pos1, pos2) in piece_positions
    //             .iter_blocks()
    //             .zip(game.falling_piece.iter_blocks())
    //         {
    //             assert_ne!(pos1, pos2);
    //         }
    //     }

    //     let piece_positions = game.falling_piece.clone();
    //     game.fall_by_one();

    //     for (pos1, pos2) in piece_positions
    //         .iter_blocks()
    //         .zip(game.falling_piece.iter_blocks())
    //     {
    //         assert_eq!(pos1, pos2);
    //     }
    // }
}
