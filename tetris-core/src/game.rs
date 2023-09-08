use crate::board::Board;
use crate::entities::{PieceType, Collision, Coord, Direction};
use crate::piece::Piece;

/// Main game struct, used to instantiate the game.
#[derive(Debug)]
pub struct Game {
    pub board: Board<10, 20>,
    pub piece: Piece,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            piece: Piece::new(PieceType::O).unwrap(),
            width: 10,
            height: 20,
        }
    }

    pub fn reload_piece(&mut self) {
        self.piece = Piece::new(PieceType::I).unwrap()
    }

    /// Check if after the move in the specified direction there will
    /// be any collision.
    pub fn get_collision_after_move(&self, dir: Direction) -> Collision {
        let dir: Coord<i32> = dir.into();

        for pos in self.piece.iter_blocks() {
            let xx = pos.x + dir.x;
            let yy = pos.y + dir.y;

            if xx < 0 {
                return Collision::LeftBorder;
            }

            if xx as usize >= self.width {
                return Collision::RightBorder;
            }

            if yy < 0 {
                return Collision::BottomBorder;
            }

            let target_block = self.board.get((xx as usize, yy as usize).into());

            if let Some(_block) = target_block {
                return Collision::Block;
            }
        }

        Collision::None
    }

    pub fn go_left(&mut self) {
        if self.get_collision_after_move(Direction::Left) == Collision::None {
            self.piece.anchor_point.x -= 1;
        }
    }

    pub fn go_right(&mut self) {
        if self.get_collision_after_move(Direction::Right) == Collision::None {
            self.piece.anchor_point.x += 1;
        }
    }

    pub fn go_down(&mut self) {
        if self.get_collision_after_move(Direction::Down) == Collision::None {
            self.piece.anchor_point.y -= 1;
        } else {
            self.set_piece_blocks_into_board();
            self.reload_piece();
        }
    }

    pub fn hard_drop(&mut self) {
        while self.get_collision_after_move(Direction::Down) == Collision::None {
            self.piece.anchor_point.y -= 1;
        }
        // map the blocks from piece into the board
        self.set_piece_blocks_into_board();
        self.reload_piece();
    }

    pub fn rotate_cw(&mut self) {
        self.piece.rotate();
    }

    pub fn rotate_ccw(&mut self) {
        self.piece.rotate_ccw();
    }

    fn set_piece_blocks_into_board(&mut self) {
        for piece_coords in self.piece.iter_blocks() {
            self.board.set(
                Some(self.piece.block_type),
                piece_coords.try_into().unwrap(),
            );
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
