use num::Integer;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Coord<I: Integer> {
    pub x: I,
    pub y: I,
}

impl<I: Integer> Coord<I> {
    pub fn new(x: I, y: I) -> Self {
        Self { x, y }
    }
}

impl<I: Integer> From<(I, I)> for Coord<I> {
    fn from((x, y): (I, I)) -> Self {
        Self::new(x, y)
    }
}

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

/// Respresents every direction the piece can be moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl From<Direction> for Coord<i32> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Left => Coord::new(-1, 0),
            Direction::Right => Coord::new(1, 0),
            Direction::Down => Coord::new(0, -1),
        }
    }
}

/// Every type of piece collision there can be. Useful for differentiating
/// between piece-out-of-grid event, or for collision for blocks.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Collision {
    None,
    LeftBorder,
    RightBorder,
    BottomBorder,
    Block,
}
