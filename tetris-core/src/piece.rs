use crate::entities::{PieceType, Coord};

#[derive(Debug, Clone)]
pub struct Piece {
    pub block_type: PieceType,
    pub anchor_point: Coord<usize>,
    pub blocks: Vec<Coord<i32>>,
    pub rotation_idx: usize,
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
    pub fn new(block_type: PieceType) -> Option<Self> {
        let anchor_point = Coord::new(4, 16);

        Some(Self {
            block_type,
            anchor_point,
            blocks: _get_blocks(block_type),
            rotation_idx: 0,
        })
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = Coord<i32>> + '_ {
        self.blocks.iter().map(move |Coord { x, y }| {
            Coord::new(
                *x + self.anchor_point.x as i32,
                *y + self.anchor_point.y as i32,
            )
        })
    }

    pub fn rotate(&mut self) {
        self._rotate(true)
    }

    pub fn rotate_ccw(&mut self) {
        self._rotate(false)
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
            let new_x_pos = r[0][0] * piece.x + r[1][0] * piece.y;
            let new_y_pos = r[0][1] * piece.x + r[1][1] * piece.y;
            piece.x = new_x_pos;
            piece.y = new_y_pos;
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
