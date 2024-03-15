use crate::board::Board;
use crate::entities::{Collision, Coord, Direction, Rotation};
use crate::piece::Piece;
use crate::scoring::Score;
use crate::srs::get_offset_table;

/// Main game struct, used to instantiate the game.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Game {
    pub board: Board,
    pub piece: Piece,
    pub next_piece: Piece,
    pub width: i32,
    pub height: i32,
    pub score: Score,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            piece: rand::random(),
            next_piece: rand::random(),
            width: 10,
            height: 20,
            score: Score::default(),
        }
    }

    pub fn reload_piece(&mut self) {
        self.piece = self.next_piece;
        self.next_piece = rand::random();
    }

    /// Check if after the move in the specified direction there will
    /// be any collision.
    #[must_use]
    pub fn get_collision_after_move(&self, dir: Direction, piece: &Piece) -> Collision {
        let dir = Coord::from(dir);

        for mut block_pos in piece.iter_blocks() {
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
        if self.get_collision_after_move(Direction::Left, &self.piece) == Collision::None {
            self.piece.anchor_point.x -= 1;
        }
    }

    pub fn go_right(&mut self) {
        if self.get_collision_after_move(Direction::Right, &self.piece) == Collision::None {
            self.piece.anchor_point.x += 1;
        }
    }

    pub fn go_down(&mut self) {
        if self.get_collision_after_move(Direction::Down, &self.piece) == Collision::None {
            self.piece.anchor_point.y -= 1;
        } else {
            self.on_drop();
        }
    }

    pub fn hard_drop(&mut self) {
        while self.get_collision_after_move(Direction::Down, &self.piece) == Collision::None {
            self.piece.anchor_point.y -= 1;
        }

        self.on_drop();
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let original_piece = self.piece;

        let old_rot_idx = self.piece.rotation_idx;

        self.piece.rotate(rotation);

        let new_rot_idx = self.piece.rotation_idx;
        // dla każdego z 5 przesunięć
        for srs_case in get_offset_table(self.piece.block_type) {
            // oblicznie alternatywnej pozycji z tablicy
            let offset = srs_case[new_rot_idx] - srs_case[old_rot_idx];

            let mut kicked_piece = self.piece;
            // próba przesunięcia skopiowanego tetrimina
            kicked_piece.anchor_point -= offset;

            // jeżeli tetrimino nie koliduje z żadnymi blokami, to znaleziono alternatywną pozycję
            if self.get_collision_after_move(Direction::None, &kicked_piece) == Collision::None {
                self.piece = kicked_piece;
                return;
            }
        }

        // wszystkie 5 pozycji nie przeszły testu na kolidowanie z klockami na planszy, więc nie obracamy
        self.piece = original_piece;
    }

    #[must_use]
    pub fn is_lost(&self) -> bool {
        for x in 0..self.width {
            if self.board.get(Coord::new(x, self.height)).is_some() {
                return true;
            }
        }

        false
    }

    fn on_drop(&mut self) {
        self.set_piece_blocks_into_board();
        let n_cleans = self.board.delete_full_lines(
            self.piece
                .iter_blocks()
                .map(|Coord { x: _, y }| y)
                .collect(),
        );

        self.score.on_lines_clear(n_cleans);
        self.score.on_drop();

        self.reload_piece();
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
