use std::collections::VecDeque;

use super::blocks::BlockType;

type Coord = (i32, i32);

fn get_random_falling_piece() -> Piece {
    // TODO implement this properly
    get_i(4, 18)
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub block_type: BlockType,
    pub anchor_point: Coord,
    pub(crate) blocks: Vec<Coord>,
    pub(crate) rotation_offset_queue: Vec<Coord>,
    pub rotation_offset: usize,
}

impl Piece {
    pub fn iter_blocks(&self) -> impl Iterator<Item = Coord> + '_ {
        self.blocks.iter().map(|(x, y)| {
            (
                x + self.anchor_point.0 + self.rotation_offset_queue[self.rotation_offset].0,
                y + self.anchor_point.1 + self.rotation_offset_queue[self.rotation_offset].1,
            )
        })
    }

    pub fn rotate(&mut self) {
        self._rotate(true)
    }

    pub fn rotate_ccw(&mut self) {
        self._rotate(false)
    }

    fn _rotate(&mut self, clockwise: bool) {
        let r = if clockwise {
            vec![vec![0, -1], vec![1, 0]]
        } else {
            vec![vec![0, 1], vec![-1, 0]]
        };

        for piece in &mut self.blocks {
            let new_x_pos = r[0][0] * piece.0 + r[1][0] * piece.1;
            let new_y_pos = r[0][1] * piece.0 + r[1][1] * piece.1;
            piece.0 = new_x_pos;
            piece.1 = new_y_pos;
        }

        if clockwise {
            if self.rotation_offset == 3 {
                self.rotation_offset = 0;
            } else {
                self.rotation_offset += 1;
            }
        } else if self.rotation_offset == 0 {
            self.rotation_offset = 3;
        } else {
            self.rotation_offset -= 1;
        }
    }
}

pub fn get_o(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::OShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        rotation_offset_queue: vec![(0, 0), (0, 1), (1, 1), (1, 0)],
        rotation_offset: 0,
    }
}

pub fn get_i(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, -1), (0, 0), (0, 1), (0, 2)],
        rotation_offset_queue: vec![(0, 0), (0, 1), (1, 1), (1, 0)],
        rotation_offset: 0,
    }
}

pub fn get_l(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (0, 1), (0, 2), (1, 0)],
        rotation_offset_queue: vec![(0, 0), (0, 0), (0, 0), (0, 0)],
        rotation_offset: 0,
    }
}

pub fn get_j(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (0, 1), (0, 2), (-1, 0)],
        rotation_offset_queue: vec![(0, 0), (0, 0), (0, 0), (0, 0)],
        rotation_offset: 0,
    }
}

pub fn get_z(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (0, 1), (-1, 1), (1, 0)],
        rotation_offset_queue: vec![(0, 0), (0, 0), (0, 0), (0, 0)],
        rotation_offset: 0,
    }
}

pub fn get_s(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (-1, 0), (0, 1), (1, 1)],
        rotation_offset_queue: vec![(0, 0), (0, 0), (0, 0), (0, 0)],
        rotation_offset: 0,
    }
}

pub fn get_t(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (-1, 0), (1, 0), (0, 1)],
        rotation_offset_queue: vec![(0, 0), (0, 0), (0, 0), (0, 0)],
        rotation_offset: 0,
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_o_rotation_compensation() {
        let mut o_piece = get_o(4, 16);

        for i in 0..4 {
            let old_piece = o_piece.clone();
            o_piece.rotate();

            for (pos1, pos2) in old_piece
                .iter_blocks()
                .sorted()
                .zip(o_piece.iter_blocks().sorted())
            {
                assert_eq!(pos1, pos2);
            }
        }
    }
}
