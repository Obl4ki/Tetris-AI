#[cfg(test)]
mod tests {
    use tetris_core::prelude::{Game, PieceType};
    use tetris_macros::gen_game;

    #[test]
    fn test_gen_board_full_representation() {
        let game: Game = gen_game!(
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x x x x x x x x x x|
            |x o o x x x x x x x|
            |i o o i i i i x x x|
        );

        println!("{game:?}");
        assert_eq!(game.board.get((0, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((1, 0).into()), Some(PieceType::O));
        assert_eq!(game.board.get((2, 0).into()), Some(PieceType::O));
        assert_eq!(game.board.get((3, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((4, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((5, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((6, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((1, 1).into()), Some(PieceType::O));
        assert_eq!(game.board.get((2, 1).into()), Some(PieceType::O));
        assert_eq!(game.board.get((0, 1).into()), None);
    }

    #[test]
    fn test_gen_board_short_representation() {
        let game: Game = gen_game!(
            |x o o x x x x x x x|
            |i o o i i i i x x x|
        );

        println!("{game:?}");
        assert_eq!(game.board.get((0, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((1, 0).into()), Some(PieceType::O));
        assert_eq!(game.board.get((2, 0).into()), Some(PieceType::O));
        assert_eq!(game.board.get((3, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((4, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((5, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((6, 0).into()), Some(PieceType::I));
        assert_eq!(game.board.get((1, 1).into()), Some(PieceType::O));
        assert_eq!(game.board.get((2, 1).into()), Some(PieceType::O));
        assert_eq!(game.board.get((0, 1).into()), None);
    }
}
