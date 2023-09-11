use crate::entities::{Coord, PieceType};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Clone)]
pub struct Piece {
    pub block_type: PieceType,
    pub anchor_point: Coord<usize>,
    pub blocks: Vec<Coord<i32>>,
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

/// Every block is represented as a Coordinate relative to the anchor point.
fn _get_blocks(block_type: PieceType) -> Vec<Coord<i32>> {
    match block_type {
        PieceType::I => vec![(0, -1), (0, 0), (0, 1), (0, 2)],
        PieceType::O => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        PieceType::T => vec![(0, 0), (-1, 0), (1, 0), (0, 1)],
        PieceType::S => vec![(0, 0), (-1, 0), (0, 1), (1, 1)],
        PieceType::Z => vec![(0, 0), (0, 1), (-1, 1), (1, 0)],
        PieceType::J => vec![(0, 0), (0, 1), (0, 2), (-1, 0)],
        PieceType::L => vec![(0, 0), (0, 1), (0, 2), (1, 0)],
    }
    .into_iter()
    .map(Coord::from)
    .collect()
}

impl Piece {
    #[must_use]
    pub fn new(block_type: PieceType) -> Self {
        let anchor_point = Coord::new(4, 16);

        Self {
            block_type,
            anchor_point,
            blocks: _get_blocks(block_type),
            rotation_idx: 0,
        }
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = Coord<i32>> + '_ {
        self.blocks.iter().map(move |Coord { x, y }| {
            let mut c: Coord<i32> = self.anchor_point.into();
            c.x += *x;
            c.y += *y;
            c
        })
    }

    pub fn rotate(&mut self) {
        self._rotate(true);
    }

    pub fn rotate_ccw(&mut self) {
        self._rotate(false);
    }

    const CLOCKWISE_ROT: [[i32; 2]; 2] = [[0, -1], [1, 0]];
    const COUNTER_CLOCKWISE_ROT: [[i32; 2]; 2] = [[0, 1], [-1, 0]];

    fn _rotate(&mut self, clockwise: bool) {
        let r = if clockwise {
            Self::CLOCKWISE_ROT
        } else {
            Self::COUNTER_CLOCKWISE_ROT
        };

        for piece in &mut self.blocks {
            let x_pos = r[0][0] * piece.x + r[1][0] * piece.y;
            let y_pos = r[0][1] * piece.x + r[1][1] * piece.y;
            piece.x = x_pos;
            piece.y = y_pos;
        }

        match clockwise {
            true => {
                if self.rotation_idx == 3 {
                    self.rotation_idx = 0;
                } else {
                    self.rotation_idx += 1;
                }
            }
            false => {
                if self.rotation_idx == 0 {
                    self.rotation_idx = 3;
                } else {
                    self.rotation_idx -= 1;
                }
            }
        }
    }
}
