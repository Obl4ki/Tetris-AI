use crate::board::Board;
use crate::entities::{Collision, Coord, Direction, PieceType, Rotation};
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            piece: Piece::new(PieceType::O),
            width: 10,
            height: 20,
        }
    }

    pub fn reload_piece(&mut self) {
        self.piece = rand::random();
    }

    /// Check if after the move in the specified direction there will
    /// be any collision.
    #[must_use]
    // tetris board is only 10x20 and proper checks are made, so no numerical errors
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn get_collision_after_move(&self, dir: Direction) -> Collision {
        let dir: Coord<i32> = dir.into();

        for pos in self.piece.iter_blocks() {
            let xx = pos.x + dir.x;
            let yy = pos.y + dir.y;

            if xx < 0 {
                return Collision::LeftBorder;
            }

            if xx >= self.width as i32 {
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

        self.set_piece_blocks_into_board();
        self.reload_piece();
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left => self.piece.rotate(),
            Rotation::Right => self.piece.rotate_ccw(),
        }
    }

    fn set_piece_blocks_into_board(&mut self) {
        for piece_coords in self.piece.iter_blocks() {
            self.board.set(
                Some(self.piece.block_type),
                piece_coords
                    .try_into()
                    .expect("Every piece block should be inside the board."),
            );
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
