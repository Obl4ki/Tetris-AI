use std::fmt::Display;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::entities::{Coord, PieceType};
use colored::customcolors::CustomColor;
use colored::Colorize;

const W: usize = 10;
const H: usize = 24;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Board {
    grid: [[Option<PieceType>; H]; W],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            grid: [[None; H]; W],
        }
    }

    #[inline]
    pub fn set(&mut self, block_type: Option<PieceType>, loc: Coord<usize>) {
        self.grid[loc.x][loc.y] = block_type;
    }

    #[must_use]
    #[inline]
    pub fn get(&self, loc: Coord<i32>) -> Option<PieceType> {
        let col = self.grid.get(loc.x as usize)?;
        let element = col.get(loc.y as usize)?;
        *element
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = (Coord<usize>, PieceType)> + '_ {
        (0..W).cartesian_product(0..H).filter_map(|(x, y)| {
            let val = self.grid[x][y];
            val.map(|block_type| (Coord::new(x, y), block_type))
        })
    }

    pub fn delete_full_lines(&mut self, ys: Vec<i32>) -> usize {
        let mut lines_to_delete = vec![];
        for y in ys {
            if self.is_whole_line_occupied(y) {
                lines_to_delete.push(y);
            }
        }

        lines_to_delete.sort_unstable();

        for y in lines_to_delete.iter().rev() {
            self.delete_line_and_shift_upper_lines_down(*y as usize);
        }

        lines_to_delete.len()
    }

    #[must_use]
    fn is_whole_line_occupied(&self, y: i32) -> bool {
        (0..W)
            .map(|x| self.get(Coord::new(x as i32, y)))
            .all(|cell| cell.is_some())
    }

    fn delete_line_and_shift_upper_lines_down(&mut self, y: usize) {
        for upper_y in y + 1..H {
            for x in 0..W {
                let upper_block = self.get((x as i32, upper_y as i32).into());
                self.set(upper_block, (x, upper_y - 1).into());
            }
        }

        for x in 0..W {
            self.set(None, (x, H - 1).into());
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..20).rev() {
            write!(f, "[")?;

            for x in 0..10 {
                let cell = self.get(Coord::new(x, y));
                let cell_str = cell.map_or("   ".black(), |block| match block {
                    PieceType::I => " I ".cyan(),
                    PieceType::O => " O ".yellow(),
                    PieceType::T => " T ".purple(),
                    PieceType::S => " S ".green(),
                    PieceType::Z => " Z ".red(),
                    PieceType::J => " J ".blue(),
                    PieceType::L => " L ".custom_color(CustomColor {
                        r: 255,
                        g: 165,
                        b: 0,
                    }),
                });

                write!(f, "{cell_str}")?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}
