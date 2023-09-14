use crate::entities::{Coord, PieceType};

#[must_use]
pub fn get_offset_table(block_type: PieceType) -> Vec<Vec<Coord<i32>>> {
    let table = match block_type {
        PieceType::I => vec![
            vec![(0, 0), (-1, 0), (-1, 1), (0, 1)],
            vec![(-1, 0), (0, 0), (1, 1), (0, 1)],
            vec![(2, 0), (0, 0), (-2, 1), (0, 1)],
            vec![(-1, 0), (0, 1), (1, 0), (0, -1)],
            vec![(2, 0), (0, -2), (-2, 0), (0, 2)],
        ],
        PieceType::O => vec![vec![(-1, 0), (0, 1), (1, 0), (0, -1)]],
        PieceType::T | PieceType::S | PieceType::Z | PieceType::J | PieceType::L => vec![
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
