pub type Coord = (i32, i32);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub type Board = Vec<Vec<Option<BlockType>>>;
