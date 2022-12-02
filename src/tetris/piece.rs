use super::{blocks::BlockType, game::GameData};

type Coord = (usize, usize);

fn get_random_falling_piece() -> Piece {
    // TODO implement this properly
    get_i(4, 18)
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub block_type: BlockType,
    pub offset: Coord,
    pub(crate) blocks: Vec<Coord>,
}

impl Piece {
    pub fn iter_blocks(&self) -> impl Iterator<Item = Coord> + '_ {
        self.blocks
            .iter()
            .map(|(x, y)| (x + self.offset.0, y + self.offset.1))
    }
}

pub fn get_o(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::OShape,
        offset: (x, y),
        blocks: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    }
}

pub fn get_i(x: usize, y: usize) -> Piece {
    Piece {
        block_type: BlockType::IShape,
        offset: (x, y),
        blocks: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
    }
}
