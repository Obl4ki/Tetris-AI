use itertools::Itertools;

use crate::entities::{Coord, PieceType};

#[derive(Clone, Debug)]
pub struct Board<const W: usize, const H: usize> {
    grid: [[Option<PieceType>; H]; W],
}

impl<const W: usize, const H: usize> Default for Board<W, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const W: usize, const H: usize> Board<W, H> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            grid: [[None; H]; W],
        }
    }

    pub fn set(&mut self, block_type: Option<PieceType>, loc: Coord<usize>) {
        self.grid[loc.x][loc.y] = block_type;
    }

    #[must_use]
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
