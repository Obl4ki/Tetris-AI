use crate::board::Board;
use crate::entities::{Coord, PieceType};
use crate::game::Game;

#[derive(Debug, Default, Clone)]
pub struct GameBuilder {
    board: Board<10, 20>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    pub fn add_piece(mut self, piece: PieceType, coord: Coord<usize>) -> Self {
        self.board.set(Some(piece), coord);
        self
    }

    pub fn build(self) -> Game {
        let mut game = Game::new();
        game.board = self.board;

        game
    }
}
