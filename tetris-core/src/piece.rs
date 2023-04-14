use super::game;
use crate::core_types::{BlockType, Board, Coord};
use crate::srs;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Piece {
    pub block_type: BlockType,
    pub anchor_point: Coord,
    pub blocks: Vec<Coord>,
    pub rotation_idx: usize,
}

impl Piece {
    pub fn new(block_type: BlockType, board_visitor: &Board) -> Option<Self> {
        let anchor_point = Self::get_anchor_point(board_visitor);
        Some(Self {
            block_type,
            anchor_point,
            blocks: get_blocks(block_type),
            rotation_idx: 0,
        })
    }

    fn get_anchor_point(board_visitor: &Board) -> Coord {
        //TODO
        (4, 16)
    }

    pub fn iter_blocks<'p, 'b: 'p>(
        &'p self,
        board_visitor: &'b game::Game,
    ) -> impl Iterator<Item = Coord> + '_ {
        let offset = self.get_first_valid_offset(board_visitor).unwrap();
        self.get_blocks_with_offset(offset)
    }

    fn get_blocks_with_offset(&self, offset: (i32, i32)) -> impl Iterator<Item = Coord> + '_ {
        self.blocks.iter().map(move |(x, y)| {
            (
                *x + self.anchor_point.0 - offset.0,
                *y + self.anchor_point.1 - offset.1,
            )
        })
    }

    pub fn get_first_valid_offset(&self, game: &game::Game) -> Option<Coord> {
        let offset_table: Vec<Vec<Coord>> = srs::get_offset_table(self.block_type);

        for offsets_for_rotations in offset_table {
            let offset = offsets_for_rotations[self.rotation_idx];

            let blocks = self.get_blocks_with_offset(offset).collect_vec();

            let any_out_of_grid = blocks
                .iter()
                .any(|(x, y)| x < &0 || x >= &(game.width as i32) || y >= &(game.height as i32));

            if any_out_of_grid {
                continue;
            }

            let collides_with_block = blocks.iter().any(|(x, y)| {
                let row = game.board.get((*x) as usize).unwrap();
                let cell = row.get((*y) as usize).unwrap();
                cell.is_some()
            });

            if collides_with_block {
                continue;
            }
            println!("{:?}", blocks.iter().collect_vec());
            return Some(offset);
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
    }
}

fn get_blocks(block_type: BlockType) -> Vec<(i32, i32)> {
    match block_type {
        BlockType::I => vec![(0, -1), (0, 0), (0, 1), (0, 2)],
        BlockType::O => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        BlockType::T => vec![(0, 0), (-1, 0), (1, 0), (0, 1)],
        BlockType::S => vec![(0, 0), (-1, 0), (0, 1), (1, 1)],
        BlockType::Z => vec![(0, 0), (0, 1), (-1, 1), (1, 0)],
        BlockType::J => vec![(0, 0), (0, 1), (0, 2), (-1, 0)],
        BlockType::L => vec![(0, 0), (0, 1), (0, 2), (1, 0)],
    }
}
