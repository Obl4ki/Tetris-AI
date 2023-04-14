use crate::{core_types, game};

pub trait Piece {
    fn new(anchor_point: core_types::Coord, game_visitor: &game::Game) -> Self;

    fn iter_blocks<T>(&self) -> T
    where
        T: Iterator<Item = core_types::BlockType>;

    fn rotate(&mut self, game_visitor: &game::Game);

    fn _get_blocks() -> Vec<core_types::Coord>;
}
