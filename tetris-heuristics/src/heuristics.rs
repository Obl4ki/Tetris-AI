#![allow(unused)]
use tetris_core::prelude::Game;

pub fn holes_present(state: &Game) -> f32 {
    let mut score = 0.;
    todo!();
}

pub fn highest_block(state: &Game) -> f32 {
    let highest = 0.;
    state
        .board
        .iter_blocks()
        .map(|(coord, _)| coord.y)
        .max()
        .unwrap_or(0) as f32
}

#[cfg(test)]
mod tests {
    use tetris_core::{entities::Coord, game_builder::GameBuilder, prelude::PieceType};

    use crate::highest_block;

    #[test]
    fn test_highest_block_single_i() {
        let game = GameBuilder::new()
            .add_piece(PieceType::I, Coord::new(0, 0))
            .add_piece(PieceType::I, Coord::new(0, 1))
            .add_piece(PieceType::I, Coord::new(0, 2))
            .add_piece(PieceType::I, Coord::new(0, 3))
            .build();

        let res = highest_block(&game);
        assert_eq!(res, 3.0);
    }

    #[test]
    fn test_highest_block_irregular_dots() {
        let game = GameBuilder::new()
            .add_piece(PieceType::I, Coord::new(3, 1))
            .add_piece(PieceType::I, Coord::new(9, 5))
            .add_piece(PieceType::I, Coord::new(7, 0))
            .add_piece(PieceType::I, Coord::new(4, 4))
            .build();

        let res = highest_block(&game);
        assert_eq!(res, 5.0);
    }
}
