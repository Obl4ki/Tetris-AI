use crate::tetris::blocks::BlockType;
use crate::tetris::piece::Piece;
use crate::tetris::piece::{get_i, get_o};
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Game {
    pub board: Vec<Vec<BlockType>>,
    pub falling_piece: Piece,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: vec![vec![BlockType::None; height]; width],
            falling_piece: get_i(4, 16),
            width,
            height,
        }
    }

    /// Piece-Piece collision checker for SRS algorithm.
    pub fn is_colliding(&self) -> Result<bool> {
        for (x, y) in self.falling_piece.iter_blocks() {
            let row = self
                .board
                .get(x as usize)
                .context(format!("X value of ({} {}) is off the grid", &x, &y))?;

            let target_block = row
                .get(y as usize)
                .context(format!("Block with ({} {}) is off the grid", &x, &y))?;

            let is_colliding = target_block != &BlockType::None;

            if is_colliding {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn go_left(&mut self) {
        if self.falling_piece.iter_blocks().any(|(x, _)| x == 0) {
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
            .iter_blocks()
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
        if self.falling_piece.iter_blocks().any(|(_, y)| y == 0) {
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
    use std::collections::VecDeque;

    use super::*;
    use crate::tetris::builders::GameBuilder;

    #[test]
    fn test_collision_block_to_block() {
        // one block overlap between falling piece's block and board block
        let game = GameBuilder::new(10, 20)
            .add_blocks(vec![(0, 0), (0, 1), (0, 2)], BlockType::SShape)
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (0, 0),
                blocks: vec![(0, 2), (1, 2), (2, 2)],
                rotation_offset_queue: vec![(0, 0)],
                rotation_offset: 0,
            })
            .compile();

        assert!(game.is_colliding().unwrap());
    }

    #[test]
    fn test_move_left_block_collision() {
        let mut game = GameBuilder::new(10, 20)
            .add_blocks(
                vec![(0, 0), (1, 0), (2, 0), (1, 0), (1, 1), (2, 0)],
                BlockType::IShape,
            )
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (4, 0),
                blocks: vec![(0, 0), (0, 1)],
                rotation_offset_queue: vec![(0, 0)],
                rotation_offset: 0,
            })
            .compile();

        // it should move left as usual (there is free space)
        let piece_positions = game.falling_piece.clone();
        game.go_left();

        for (pos1, pos2) in piece_positions
            .iter_blocks()
            .zip(game.falling_piece.iter_blocks())
        {
            assert_ne!(pos1, pos2);
        }

        let piece_positions = game.falling_piece.clone();
        game.go_left();

        // only one pair of blocks colided but there should be no change after the move
        for (pos1, pos2) in piece_positions
            .iter_blocks()
            .zip(game.falling_piece.iter_blocks())
        {
            assert_eq!(pos1, pos2);
        }
    }

    #[test]
    fn test_move_right_block_collision() {
        let mut game = GameBuilder::new(10, 20)
            .add_blocks(
                vec![(9, 0), (9, 1), (9, 2), (8, 0), (8, 1), (7, 0)],
                BlockType::IShape,
            )
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (6, 1),
                blocks: vec![(0, 0), (0, 1)],
                rotation_offset_queue: vec![(0, 0)],
                rotation_offset: 0,
            })
            .compile();

        // it should move left as usual (there is free space)
        let piece_positions = game.falling_piece.clone();
        game.go_right();

        for (pos1, pos2) in piece_positions
            .iter_blocks()
            .zip(game.falling_piece.iter_blocks())
        {
            assert_ne!(pos1, pos2);
        }

        let piece_positions = game.falling_piece.clone();
        game.go_right();

        // only one pair of blocks colided but there should be no change after the move
        for (pos1, pos2) in piece_positions
            .iter_blocks()
            .zip(game.falling_piece.iter_blocks())
        {
            assert_eq!(pos1, pos2);
        }
    }

    #[test]
    fn test_left_board_margin_collision() {
        let mut game = GameBuilder::new(10, 20)
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (0, 0),
                blocks: vec![(0, 0), (0, 1)],
                rotation_offset_queue: vec![(0, 0)],
                rotation_offset: 0,
            })
            .compile();

        let piece_positions = game.falling_piece.clone();
        game.go_left();

        for (pos1, pos2) in piece_positions
            .iter_blocks()
            .zip(game.falling_piece.iter_blocks())
        {
            assert_eq!(pos1, pos2);
        }
    }

    #[test]
    fn test_right_board_margin_collision() {
        let mut game = GameBuilder::new(10, 20)
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (9, 0),
                blocks: vec![(0, 0), (0, 1)],
                rotation_offset_queue: vec![(0, 0)],
                rotation_offset: 0,
            })
            .compile();

        let piece_positions = game.falling_piece.clone();
        game.go_right();

        for (pos1, pos2) in piece_positions
            .iter_blocks()
            .zip(game.falling_piece.iter_blocks())
        {
            assert_eq!(pos1, pos2);
        }
    }

    #[test]
    fn test_fall_by_one() {
        let mut game = GameBuilder::new(10, 20)
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (0, 4),
                blocks: vec![(0, 0), (0, 1)],
                rotation_offset_queue: vec![(0, 0)],
                rotation_offset: 0,
            })
            .compile();

        for _ in 0..4 {
            let piece_positions = game.falling_piece.clone();
            game.fall_by_one();

            for (pos1, pos2) in piece_positions
                .iter_blocks()
                .zip(game.falling_piece.iter_blocks())
            {
                assert_ne!(pos1, pos2);
            }
        }

        let piece_positions = game.falling_piece.clone();
        game.fall_by_one();

        for (pos1, pos2) in piece_positions
            .iter_blocks()
            .zip(game.falling_piece.iter_blocks())
        {
            assert_eq!(pos1, pos2);
        }
    }

    #[test]
    fn test_fall_by_one_block_collision() {
        let mut game = GameBuilder::new(10, 20)
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (1, 4),
                blocks: vec![(0, 0), (0, 1)],
                rotation_offset_queue: vec![(0, 0)],
                rotation_offset: 0,
            })
            .add_blocks(
                vec![(0, 0), (0, 1), (1, 0), (2, 0), (2, 1), (2, 2)],
                BlockType::IShape,
            )
            .compile();

        for _ in 0..3 {
            let piece_positions = game.falling_piece.clone();
            game.fall_by_one();

            for (pos1, pos2) in piece_positions
                .iter_blocks()
                .zip(game.falling_piece.iter_blocks())
            {
                assert_ne!(pos1, pos2);
            }
        }

        let piece_positions = game.falling_piece.clone();
        game.fall_by_one();

        for (pos1, pos2) in piece_positions
            .iter_blocks()
            .zip(game.falling_piece.iter_blocks())
        {
            assert_eq!(pos1, pos2);
        }
    }
}
