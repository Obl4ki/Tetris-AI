use itertools::Itertools;

use crate::entities::{BlockType, Coord};

#[derive(Clone, Debug)]
pub struct Board<const W: usize, const H: usize> {
    grid: [[Option<BlockType>; H]; W],
}

impl<const W: usize, const H: usize> Default for Board<W, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const W: usize, const H: usize> Board<W, H> {
    pub fn new() -> Self {
        Self {
            grid: [[None; H]; W],
        }
    }

    pub fn set(&mut self, block_type: Option<BlockType>, loc: Coord<usize>) {
        self.grid[loc.x][loc.y] = block_type;
    }

    pub fn get(&self, loc: Coord<usize>) -> Option<BlockType> {
        self.grid[loc.x][loc.y]
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = (Coord<usize>, BlockType)> + '_ {
        (0..W).cartesian_product(0..H).filter_map(|(x, y)| {
            let val = self.grid[x][y];
            val.map(|block_type| (Coord::new(x, y), block_type))
        })
    }
}
