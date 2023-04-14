use crate::core_types::{BlockType, Coord};

pub fn get_offset_table(block_type: BlockType) -> Vec<Vec<Coord>> {
    match block_type {
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
    }
}
