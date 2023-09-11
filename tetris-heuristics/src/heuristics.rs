use itertools::Itertools;
use tetris_core::{entities::Coord, prelude::Game};

type HeuristicScore = usize;

fn get_cols_max_heights(state: &Game) -> [usize; 10] {
    let mut highest_blocks_x_axis = [0; 10];

    for (coord, _block) in state.board.iter_blocks() {
        if highest_blocks_x_axis[coord.x] > coord.y {
            highest_blocks_x_axis[coord.x] = coord.y;
        }
    }
    highest_blocks_x_axis
}

/// Measures the amount of holes present on board.
/// Holes are defined as cells with no blocks that have some block above them.
#[must_use]
pub fn holes_present(state: &Game) -> HeuristicScore {
    let mut score = 0;
    let highest_blocks_x_axis = get_cols_max_heights(state);

    for (x, highest_y) in highest_blocks_x_axis.into_iter().enumerate() {
        score += (highest_y..=0)
            .filter(|y| state.board.get(Coord::new(x, *y)).is_none())
            .count();
    }

    score
}

#[must_use]
pub fn highest_block(state: &Game) -> HeuristicScore {
    state
        .board
        .iter_blocks()
        .map(|(coord, _)| coord.y)
        .max()
        .unwrap_or(0)
        + 1
}

#[must_use]
pub fn bumpyness(state: &Game) -> HeuristicScore {
    let highest_blocks_x_axis = get_cols_max_heights(state);

    let mut score = 0;
    for (prev, next) in highest_blocks_x_axis.into_iter().tuple_windows::<(_, _)>() {
        score += prev.abs_diff(next);
    }

    score
}

#[cfg(test)]
mod tests {
    use tetris_core::{entities::Coord, game_builder::GameBuilder};

    use crate::highest_block;
    use tetris_core::entities::PieceType as PT;

    #[test]
    fn test_highest_block_single_i() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(0, 0))
            .add_piece(PT::I, Coord::new(0, 1))
            .add_piece(PT::I, Coord::new(0, 2))
            .add_piece(PT::I, Coord::new(0, 3))
            .build();

        let res = highest_block(&game);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_highest_block_irregular_dots() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(3, 1))
            .add_piece(PT::I, Coord::new(9, 5))
            .add_piece(PT::I, Coord::new(7, 0))
            .add_piece(PT::I, Coord::new(4, 4))
            .build();

        let res = highest_block(&game);
        assert_eq!(res, 6);
    }
}
