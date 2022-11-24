use crate::tetris::blocks::BlockType;
use crate::tetris::piece::Piece;
use crate::tetris::piece::{get_i, get_o};

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

    pub fn is_colliding(&self) -> bool {
        todo!()
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
