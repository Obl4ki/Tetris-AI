use itertools::Itertools;
use tetris_core::prelude::*;

pub type HeuristicScore = f32;
pub type Heuristic = fn(&Board) -> HeuristicScore;

/// Helper method to get height of each individual column in the tetris board.
#[must_use]
fn get_cols_max_heights(state: &Board) -> [usize; 10] {
    let mut highest_blocks_x_axis = [0; 10];

    for (coord, _block) in state.iter_blocks() {
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
pub fn holes_present(state: &Board) -> HeuristicScore {
    let mut score = 0.;
    let highest_blocks_x_axis = get_cols_max_heights(state);

    for (x, highest_y) in highest_blocks_x_axis.into_iter().enumerate() {
        score += (0..highest_y)
            .rev()
            .filter(|y| state.get(Coord::new(x as i32, *y as i32)).is_none())
            .count() as f32;
    }

    score
}

/// Measures the height of the highest block in the entire tetris board.
#[must_use]
pub fn highest_block(state: &Board) -> HeuristicScore {
    state
        .iter_blocks()
        .map(|(coord, _)| coord.y)
        .max()
        .unwrap_or(0) as f32
        + 1.
}

/// Measures the "bumpyness" of the columns in the grid.
/// This means that difference in heights of each individual next and previous columns will be summed.
#[must_use]
pub fn bumpyness(state: &Board) -> HeuristicScore {
    let highest_blocks_x_axis = get_cols_max_heights(state);

    let mut score = 0.;
    for (prev, next) in highest_blocks_x_axis.into_iter().tuple_windows::<(_, _)>() {
        score += prev.abs_diff(next) as f32;
    }

    score
}

/// Maximum minus minumum height of all the columns.
#[must_use]
pub fn relative_diff(state: &Board) -> HeuristicScore {
    let heights = get_cols_max_heights(state);
    let max = heights.iter().max().copied().unwrap_or_default() as f32;
    let min = heights.iter().min().copied().unwrap_or_default() as f32;
    max - min
}

#[must_use]
pub fn clear_potential(state: &Board) -> HeuristicScore {
    let mut game = Game::new();
    game.board = *state;
    game.piece = Piece::new(PieceType::I);
    game.piece.anchor_point = Coord::new(0, game.piece.anchor_point.y);

    let mut maximum_clears = 0;

    for _ in 0..game.width {
        let mut game_cpy = game.clone();
        game_cpy.hard_drop();

        maximum_clears = maximum_clears.max(game_cpy.score.cleared_rows - game.score.cleared_rows);
        if maximum_clears == 4 {
            break;
        }

        game.piece.anchor_point.x += 1;
    }

    maximum_clears as f32
}

#[must_use]
pub fn distance_mean_from_4(state: &Board) -> HeuristicScore {
    let heights = get_cols_max_heights(state);

    let most_common_height = heights
        .into_iter()
        .counts()
        .drain()
        .max_by_key(|(_, count)| *count);

    match most_common_height {
        Some((height, _)) if height >= 4 => 2usize.pow((height - 4) as u32) as HeuristicScore,
        Some((height, _)) => (4 - height) as HeuristicScore,
        None => 0.,
    }
}

#[cfg(test)]
mod tests {

    use tetris_core::{entities::Coord, game_builder::GameBuilder};

    use tetris_core::entities::PieceType as PT;

    use super::{
        bumpyness, clear_potential, get_cols_max_heights, highest_block, holes_present,
        relative_diff,
    };

    #[test]
    fn test_get_cols_max_heights() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(0, 0))
            .add_piece(PT::I, Coord::new(1, 1))
            .add_piece(PT::I, Coord::new(2, 5))
            .add_piece(PT::I, Coord::new(5, 9))
            .build();
        let heights = get_cols_max_heights(&game.board);

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

        let res = highest_block(&game.board);
        assert!((res - 4.).abs() < f32::EPSILON);
    }

    #[test]
    fn test_highest_block_irregular_dots() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(3, 1))
            .add_piece(PT::I, Coord::new(9, 5))
            .add_piece(PT::I, Coord::new(7, 0))
            .add_piece(PT::I, Coord::new(4, 4))
            .build();

        let res = highest_block(&game.board);
        assert!((res - 6.).abs() < f32::EPSILON);
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
        let res = bumpyness(&game.board);
        assert!((res - 12.).abs() < f32::EPSILON);
    }

    #[test]
    fn test_relative_diff() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(0, 1))
            .add_piece(PT::I, Coord::new(3, 5))
            .add_piece(PT::I, Coord::new(2, 7))
            .build();

        let res = relative_diff(&game.board);
        assert!((res - 7.).abs() < f32::EPSILON);
    }

    #[test]
    fn test_holes_present() {
        let game = GameBuilder::new()
            .add_piece(PT::I, Coord::new(0, 1))
            .add_piece(PT::I, Coord::new(1, 0))
            .add_piece(PT::I, Coord::new(3, 3))
            .build();

        let res = holes_present(&game.board);
        assert!((res - 4.).abs() < f32::EPSILON);
    }

    #[test]
    fn test_clear_potential_on_edges() {
        // well on left
        let mut gb = GameBuilder::new();
        for x in 1..10 {
            gb = gb.add_piece(PT::I, Coord::new(x, 0));
        }
        let game = gb.build();

        let res = clear_potential(&game.board);
        assert!(((res - 1.).abs() < f32::EPSILON));

        // well on right
        let mut gb = GameBuilder::new();
        for x in 0..9 {
            gb = gb.add_piece(PT::I, Coord::new(x, 0));
        }
        let game = gb.build();

        let res = clear_potential(&game.board);
        assert!(((res - 1.).abs() < f32::EPSILON));

        // well on right height 2
        let mut gb = GameBuilder::new();
        for x in 0..9 {
            gb = gb.add_piece(PT::I, Coord::new(x, 0));
            gb = gb.add_piece(PT::I, Coord::new(x, 1));
        }
        let game = gb.build();

        let res = clear_potential(&game.board);
        assert!(((res - 2.).abs() < f32::EPSILON));
    }
}
