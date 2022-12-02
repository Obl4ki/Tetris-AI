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
                .get(x)
                .context(format!("X value of {} is off the grid", &x))?;

            let target_block = row
                .get(y)
                .context(format!("Block with ({} {}) is off the grid", &x, &y))?;

            let is_colliding = target_block == &BlockType::None;

            if is_colliding {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn go_left(&mut self) {
        if self.falling_piece.anchor_point.0 == 0 {
            return;
        }

        self.falling_piece.anchor_point.0 -= 1;
    }
    pub fn go_right(&mut self) {
        for (_, x) in self.falling_piece.iter_blocks() {
            if x == self.width - 1 {
                return;
            }
        }

        self.falling_piece.anchor_point.0 += 1;
    }
    pub fn fall_by_one(&mut self) {
        todo!()
    }

    pub fn drop(&mut self) {
        todo!()
    }

    pub fn rotate_cw(&mut self) {}

    pub fn rotate_ccw(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetris::builders::GameBuilder;

    #[test]
    fn test_collision_block_to_block() {
        let game = GameBuilder::new(10, 20)
            .add_blocks(vec![(0, 0), (0, 1), (0, 2)], BlockType::SShape)
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (0, 0),
                blocks: vec![(0, 2), (1, 2), (2, 2)],
            })
            .compile();

        assert!(game.is_colliding().unwrap());
    }

    #[test]
    fn test_collision_block_to_border() {
        let game = GameBuilder::new(10, 20)
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (7, 0),
                blocks: vec![(0, 0), (1, 0), (2, 0)],
            })
            .compile();
        // on the right edge, should not collide yet
        assert!(!game.is_colliding().unwrap());

        // move to the right by any value
        for i in 1..5 {
            let game = GameBuilder::new(10, 20)
                .set_falling_piece(Piece {
                    block_type: BlockType::OShape,
                    anchor_point: (7 + i, 0),
                    blocks: vec![(0, 0), (1, 0), (2, 0)],
                })
                .compile();

            // bang
            assert!(game.is_colliding().unwrap())
        }
    }

    #[test]
    fn test_move_left() {
        let mut game = GameBuilder::new(10, 20)
            .add_blocks(
                vec![(0, 0), (1, 0), (2, 0), (1, 0), (1, 1), (2, 0)],
                BlockType::IShape,
            )
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                anchor_point: (7, 0),
                blocks: vec![(4, 0), (4, 1)],
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
}
