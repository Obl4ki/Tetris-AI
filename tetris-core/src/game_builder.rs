use crate::game::Game;
use crate::piece::Piece;
use crate::entities::BlockType;

#[derive(Debug, Default, Clone, Copy)]
pub struct GameBuilder {
    width: usize,
    height: usize,
    block_type: Option<BlockType>,
}

impl GameBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            block_type: None,
        }
    }
    pub fn build(self) -> Game {
        let board = vec![vec![None; self.height]; self.width];
        Game {
            board: board.clone(),
            falling_piece: Piece::new(self.block_type.unwrap_or(BlockType::O)).unwrap(),
            width: self.width,
            height: self.height,
        }
    }
}
