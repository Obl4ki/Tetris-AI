use crate::entities::{Coord, PieceType, Rotation};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Piece {
    pub block_type: PieceType,
    pub anchor_point: Coord<i32>,
    pub block_positions: [Coord<i32>; 4],
    pub rotation_idx: usize,
}

impl Distribution<Piece> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Piece {
        match rng.gen_range(0..7) {
            0 => Piece::new(PieceType::I),
            1 => Piece::new(PieceType::O),
            2 => Piece::new(PieceType::T),
            3 => Piece::new(PieceType::S),
            4 => Piece::new(PieceType::Z),
            5 => Piece::new(PieceType::J),
            6 => Piece::new(PieceType::L),
            _ => unreachable!(),
        }
    }
}

#[must_use]
/// Every block is represented as a Coordinate relative to the anchor point.
pub const fn get_blocks(block_type: PieceType) -> [Coord<i32>; 4] {
    let c = Coord::new;
    match block_type {
        PieceType::I => [c(0, -2), c(0, -1), c(0, 0), c(0, 1)],
        PieceType::O => [c(0, 0), c(0, 1), c(1, 0), c(1, 1)],
        PieceType::T => [c(0, 0), c(-1, 0), c(1, 0), c(0, 1)],
        PieceType::S => [c(0, 0), c(-1, 0), c(0, 1), c(1, 1)],
        PieceType::Z => [c(0, 0), c(0, 1), c(-1, 1), c(1, 0)],
        PieceType::J => [c(0, 0), c(-1, 0), c(-1, 1), c(1, 0)],
        PieceType::L => [c(0, 0), c(-1, 0), c(1, 0), c(1, 1)],
    }
}

impl Piece {
    #[must_use]
    pub const fn new(block_type: PieceType) -> Self {
        let anchor_point = Coord::new(4, 21);

        Self {
            block_type,
            anchor_point,
            block_positions: get_blocks(block_type),
            rotation_idx: 0,
        }
    }

    #[must_use]
    pub fn random() -> Self {
        rand::random()
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = Coord<i32>> + '_ {
        self.block_positions
            .iter()
            .map(|coord| self.anchor_point + *coord)
    }

    const CLOCKWISE_ROT: [[i32; 2]; 2] = [[0, -1], [1, 0]];
    const COUNTER_CLOCKWISE_ROT: [[i32; 2]; 2] = [[0, 1], [-1, 0]];

    pub fn rotate(&mut self, clockwise: Rotation) {
        let r = match clockwise {
            Rotation::Clockwise => Self::CLOCKWISE_ROT,
            Rotation::Counterclockwise => Self::COUNTER_CLOCKWISE_ROT,
        };

        for piece in &mut self.block_positions {
            let x_pos = r[0][0] * piece.x + r[1][0] * piece.y;
            let y_pos = r[0][1] * piece.x + r[1][1] * piece.y;
            piece.x = x_pos;
            piece.y = y_pos;
        }

        match clockwise {
            Rotation::Clockwise => {
                if self.rotation_idx == 3 {
                    self.rotation_idx = 0;
                } else {
                    self.rotation_idx += 1;
                }
            }
            Rotation::Counterclockwise => {
                if self.rotation_idx == 0 {
                    self.rotation_idx = 3;
                } else {
                    self.rotation_idx -= 1;
                }
            }
        }
    }
}
