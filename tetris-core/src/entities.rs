use anyhow::Result;
use num::Integer;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Coord<I: Integer> {
    pub x: I,
    pub y: I,
}

impl<I: Integer> Coord<I> {
    pub const fn new(x: I, y: I) -> Self {
        Self { x, y }
    }
}

impl<I: Integer> From<(I, I)> for Coord<I> {
    fn from((x, y): (I, I)) -> Self {
        Self::new(x, y)
    }
}

impl TryFrom<Coord<i32>> for Coord<usize> {
    type Error = anyhow::Error;

    fn try_from(value: Coord<i32>) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;
        Ok(Self::new(x, y))
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
impl From<Coord<usize>> for Coord<i32> {
    fn from(value: Coord<usize>) -> Self {
        Self::new(value.x as i32, value.y as i32)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType {
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
            Direction::Left => Self::new(-1, 0),
            Direction::Right => Self::new(1, 0),
            Direction::Down => Self::new(0, -1),
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
