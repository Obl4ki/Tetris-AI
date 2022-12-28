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
}

impl Piece {
    pub fn iter_blocks(&self) -> impl Iterator<Item = Coord> + '_ {
        self.blocks
            .iter()
            .map(|(x, y)| (x + self.anchor_point.0, y + self.anchor_point.1))
    }
}

pub fn get_o(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::OShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    }
}

pub fn get_i(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
    }
}

pub fn get_l(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (0, 1), (0, 2), (1, 0)],
    }
}


pub fn get_j(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        anchor_point: (x as i32, y as i32),
        blocks: vec![(0, 0), (0, 1), (0, 2), (-1, 0)],
    }
}