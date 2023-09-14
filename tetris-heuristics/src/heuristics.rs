use itertools::Itertools;
use tetris_core::{entities::Coord, prelude::Game};

type HeuristicScore = usize;

/// Helper method to get height of each individual column in the tetris board.
#[must_use]
fn get_cols_max_heights(state: &Game) -> [usize; 10] {
    let mut highest_blocks_x_axis = [0; 10];

    for (coord, _block) in state.board.iter_blocks() {
        if coord.y > highest_blocks_x_axis[coord.x] {
            highest_blocks_x_axis[coord.x] = coord.y;
        }
    }
    highest_blocks_x_axis
}

/// Measures the amount of holes present on board.
/// Holes are defined as cells with no blocks that have some block above them.
/// Distance to top block can be greater than one.
#[must_use]
pub fn holes_present(state: &Game) -> HeuristicScore {
    let mut score = 0;
    let highest_blocks_x_axis = get_cols_max_heights(state);

    for (x, highest_y) in highest_blocks_x_axis.into_iter().enumerate() {
        score += (0..highest_y)
            .rev()
            .filter(|y| state.board.get(Coord::new(x as i32, *y as i32)).is_none())
            .count();
    }

    score
}

/// Measures the height of the highest block in the entire tetris board.
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

/// Measures the "bumpyness" of the columns in the grid.
/// This means that difference in heights of each individual next and previous columns will be summed.
#[must_use]
pub fn bumpyness(state: &Game) -> HeuristicScore {
    let highest_blocks_x_axis = get_cols_max_heights(state);

    let mut score = 0;
    for (prev, next) in highest_blocks_x_axis.into_iter().tuple_windows::<(_, _)>() {
        score += prev.abs_diff(next);
    }

    score
}

/// Maximum minus minumum height of all the columns.
#[must_use]
pub fn relative_diff(state: &Game) -> HeuristicScore {
    let heights = dbg!(get_cols_max_heights(state));
    let max = heights.iter().max().copied().unwrap_or_default();
    let min = heights.iter().min().copied().unwrap_or_default();
    max - min
}

#[cfg(test)]
mod tests {
    use tetris_core::{entities::Coord, game_builder::GameBuilder};

    use crate::{
        heuristics::{bumpyness, relative_diff},
        highest_block, holes_present,
    };
    use tetris_core::entities::PieceType as PT;

    use super::get_cols_max_heights;

    #[test]
    fn test_get_cols_max_heights() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(0, 0))
            .add_piece(PT::I, Coord::new(1, 1))
            .add_piece(PT::I, Coord::new(2, 5))
            .add_piece(PT::I, Coord::new(5, 9))
            .build();
        let heights = get_cols_max_heights(&game);

        assert_eq!(heights, [0, 1, 5, 0, 0, 9, 0, 0, 0, 0]);
    }

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

    #[test]
    fn test_bumpyness() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(0, 0))
            .add_piece(PT::I, Coord::new(1, 2))
            .add_piece(PT::I, Coord::new(2, 2))
            .add_piece(PT::I, Coord::new(3, 3))
            .add_piece(PT::I, Coord::new(4, 4))
            .add_piece(PT::I, Coord::new(5, 1))
            .add_piece(PT::I, Coord::new(6, 4))
            .add_piece(PT::I, Coord::new(7, 2))
            .add_piece(PT::I, Coord::new(8, 2))
            .add_piece(PT::I, Coord::new(9, 2))
            .build();
        assert_eq!(bumpyness(&game), 12);
    }

    #[test]
    fn test_relative_diff() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(0, 1))
            .add_piece(PT::I, Coord::new(3, 5))
            .add_piece(PT::I, Coord::new(2, 7))
            .build();

        let res = relative_diff(&game);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_holes_present() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(0, 1))
            .add_piece(PT::I, Coord::new(1, 0))
            .add_piece(PT::I, Coord::new(3, 3))
            .build();

        let res = holes_present(&game);
        assert_eq!(res, 4);
    }
}
