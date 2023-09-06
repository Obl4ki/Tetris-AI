use crate::entities::{BlockType, Coord};

pub fn get_offset_table(block_type: BlockType) -> Vec<Vec<Coord<i32>>> {
    let table = match block_type {
        BlockType::I => vec![
            vec![(0, 0), (-1, 0), (-1, 1), (0, 1)],
            vec![(-1, 0), (0, 0), (1, 1), (0, 1)],
            vec![(2, 0), (0, 0), (-2, 1), (0, 1)],
            vec![(-1, 0), (0, 1), (1, 0), (0, -1)],
            vec![(2, 0), (0, -2), (-2, 0), (0, 2)],
        ],
        BlockType::O => vec![vec![(0, 0), (0, -1), (-1, -1), (-1, 0)]],
        BlockType::T | BlockType::S | BlockType::Z | BlockType::J | BlockType::L => vec![
            vec![(0, 0), (0, 0), (0, 0), (0, 0)],
            vec![(0, 0), (1, 0), (0, 0), (-1, 0)],
            vec![(0, 0), (1, -1), (0, 0), (-1, -1)],
            vec![(0, 0), (0, 2), (0, 0), (0, 2)],
            vec![(0, 0), (1, 2), (0, 0), (-1, 2)],
        ],
    };

    table
        .into_iter()
        .map(|r| r.into_iter().map(Coord::from).collect())
        .collect()
}
