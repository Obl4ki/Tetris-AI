use crate::board::Board;
use crate::entities::{Collision, Coord, Direction, PieceType, Rotation};
use crate::piece::Piece;
use crate::srs::get_offset_table;

/// Main game struct, used to instantiate the game.
#[derive(Debug)]
pub struct Game {
    pub board: Board<10, 20>,
    pub piece: Piece,
    pub width: i32,
    pub height: i32,
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
    pub fn get_collision_after_move(&self, dir: Direction) -> Collision {
        let dir = Coord::from(dir);

        for mut block_pos in self.piece.iter_blocks() {
            block_pos.x += dir.x;
            block_pos.y += dir.y;

            if block_pos.x < 0 {
                return Collision::LeftBorder;
            }

            if block_pos.x >= self.width {
                return Collision::RightBorder;
            }

            if block_pos.y < 0 {
                return Collision::BottomBorder;
            }

            let target_block = self.board.get(block_pos);

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
        let original_piece = self.piece.clone();
        self.piece.rotate(rotation);
        dbg!(self.piece.rotation_idx);
        for srs_case in get_offset_table(self.piece.block_type) {
            let offset = srs_case[self.piece.rotation_idx];
            let mut kicked_piece = self.piece.clone();

            kicked_piece.anchor_point += offset;
            if self.get_collision_after_move(Direction::None) == Collision::None {
                self.piece = kicked_piece;
                return;
            }
        }

        self.piece = original_piece;
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
