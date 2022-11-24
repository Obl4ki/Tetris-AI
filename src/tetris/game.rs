use crate::tetris::blocks::BlockType;
use crate::tetris::piece::Piece;
use crate::tetris::piece::{get_i, get_o};

struct GameBuilder {
    data: Vec<Vec<BlockType>>,
    width: usize,
    height: usize,
    falling_piece: Option<Piece>,
}

impl GameBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![BlockType::None; height]; width],
            width,
            height,
            falling_piece: None,
        }
    }

    pub fn add_blocks(mut self, blocks: Vec<(usize, usize)>, block_type: BlockType) -> Self {
        for (x, y) in blocks {
            self.data[x][y] = block_type;
        }

        self
    }

    pub fn set_falling_piece(mut self, piece: Piece) -> Self {
        self.falling_piece = Some(piece);
        self
    }

    pub fn compile(mut self) -> Game {
        let falling_piece = self.falling_piece.unwrap_or_else(get_random_falling_piece);
        Game {
            board: GameData { data: self.data },
            falling_piece,
            width: self.width,
            height: self.height,
        }
    }
}

fn get_random_falling_piece() -> Piece {
    // TODO implement this properly
    get_i(4, 18)
}

#[derive(Debug)]
pub struct Game {
    pub board: GameData,
    pub falling_piece: Piece,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: GameData::new(width, height),
            falling_piece: get_i(4, 16),
            width,
            height,
        }
    }

    /// Piece-border and Piece-Piece collision checker for SRS algorithm.
    pub fn is_colliding(&self) -> bool {
        // check for board borders and collisions
        for (x, y) in self.falling_piece.iter_blocks() {
            let target_block = self.board.data.get(x).and_then(|x| x.get(y));
            if target_block.is_none() {
                // off the grid
                return true;
            }

            let is_colliding = match target_block {
                Some(block) => *block != BlockType::None,
                None => false,
            };

            if is_colliding {
                return true;
            };
        }

        false
    }

    pub fn go_left(&mut self) {
        todo!()
    }
    pub fn go_right(&mut self) {
        todo!()
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

#[derive(Debug)]
pub struct GameData {
    pub data: Vec<Vec<BlockType>>,
}

impl GameData {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![BlockType::None; height]; width],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_block_to_block() {
        let game = GameBuilder::new(10, 20)
            .add_blocks(vec![(0, 0), (0, 1), (0, 2)], BlockType::SShape)
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                offset: (0, 0),
                blocks: vec![(0, 2), (1, 2), (2, 2)],
            })
            .compile();

        assert!(game.is_colliding())
    }

    #[test]
    fn test_collision_block_to_border() {
        let game = GameBuilder::new(10, 20)
            .set_falling_piece(Piece {
                block_type: BlockType::OShape,
                offset: (7, 0),
                blocks: vec![(0, 0), (1, 0), (2, 0)],
            })
            .compile();
        // on the right edge, should not collide yet
        assert!(!game.is_colliding());

        // move to the right by any value
        for i in 1..5 {
            let game = GameBuilder::new(10, 20)
                .set_falling_piece(Piece {
                    block_type: BlockType::OShape,
                    offset: (7 + i, 0),
                    blocks: vec![(0, 0), (1, 0), (2, 0)],
                })
                .compile();

            // bang
            assert!(game.is_colliding())
        }
    }
}
