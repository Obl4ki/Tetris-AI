use super::{core_types, game::Game, piece::Piece, BlockType};

#[derive(Debug, Default, Clone, Copy)]
pub struct GameBuilder {
    width: usize,
    height: usize,
    block_type: Option<core_types::BlockType>,
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
            falling_piece: Piece::new(self.block_type.unwrap_or(BlockType::O), &board).unwrap(),
            width: self.width,
            height: self.height,
        }
    }
}
