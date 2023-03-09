use super::game::Game;
use super::BlockType;
use crate::tetris::core_types;
use crate::tetris::srs;
use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Piece {
    pub block_type: core_types::BlockType,
    pub anchor_point: core_types::Coord,
    pub blocks: Vec<core_types::Coord>,
    pub rotation_idx: usize,
}


impl Piece {
    pub fn new(block_type: core_types::BlockType, board_visitor: &core_types::Board) -> Option<Self> {
        let anchor_point = Self::get_anchor_point(board_visitor);
        Some(Self {
            block_type,
            anchor_point,
            blocks: get_blocks(block_type),
            rotation_idx: 0,
        })
    }

    fn get_anchor_point(board_visitor: &core_types::Board) -> core_types::Coord {
        (4, 16)
    }

    pub fn iter_blocks<'p, 'b: 'p>(
        &'p self,
        board_visitor: &'b core_types::Board,
    ) -> impl Iterator<Item = core_types::Coord> + '_ {
        let offset = self.get_first_valid_offset(board_visitor).unwrap();
        self.get_blocks_with_offset(offset)
    }

    fn get_blocks_with_offset(
        &self,
        offset: (i32, i32),
    ) -> impl Iterator<Item = core_types::Coord> + '_ {
        self.blocks.iter().map(move |(x, y)| {
            (
                *x + self.anchor_point.0 + offset.0,
                *y + self.anchor_point.1 + offset.1,
            )
        })
    }

    pub fn get_first_valid_offset(&self, board_visitor: &core_types::Board) -> Option<core_types::Coord> {
        let offset_table: Vec<Vec<core_types::Coord>> = srs::get_offset_table(self.block_type);

        for offsets_for_rotations in offset_table {
            let offset = offsets_for_rotations[self.rotation_idx];

            let mut obstructed = false;
            // let blocks_with_offset =
            for (x, y) in self.get_blocks_with_offset(offset) {
                if let Some(row) = board_visitor.get(x as usize) {
                    if let Some(cell) = row.get(y as usize) {
                        if cell.is_some() {
                            obstructed = true;
                            break;
                        }
                    }
                }
            }

            if !obstructed {
                return Some(offset);
            }
        }

        None
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
            if self.rotation_idx == 3 {
                self.rotation_idx = 0;
            } else {
                self.rotation_idx += 1;
            }
        } else if self.rotation_idx == 0 {
            self.rotation_idx = 3;
        } else {
            self.rotation_idx -= 1;
        }
        dbg!(self.rotation_idx);
    }
}

fn get_blocks(block_type: super::BlockType) -> Vec<(i32, i32)> {
    match block_type {
        super::BlockType::I => vec![(0, -1), (0, 0), (0, 1), (0, 2)],
        super::BlockType::O => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        super::BlockType::T => vec![(0, 0), (-1, 0), (1, 0), (0, 1)],
        super::BlockType::S => vec![(0, 0), (-1, 0), (0, 1), (1, 1)],
        super::BlockType::Z => vec![(0, 0), (0, 1), (-1, 1), (1, 0)],
        super::BlockType::J => vec![(0, 0), (0, 1), (0, 2), (-1, 0)],
        super::BlockType::L => vec![(0, 0), (0, 1), (0, 2), (1, 0)],
    }
}

// #[cfg(test)]
// mod tests {
//     use itertools::Itertools;

//     use crate::tetris::builders::GameBuilder;

//     use super::*;

//     #[test]
//     fn test_o_rotation_compensation() {
//         let mut o_piece = get_o(4, 16);
//         let game = GameBuilder::new(10, 20).compile();
//         for _ in 0..4 {
//             let old_piece = o_piece.clone();
//             o_piece.rotate(&game);

//             for (pos1, pos2) in old_piece
//                 .iter_blocks()
//                 .sorted()
//                 .zip(o_piece.iter_blocks().sorted())
//             {
//                 assert_eq!(pos1, pos2);
//             }
//         }
//     }
// }
