pub enum Direction {
    Left,
    Right,
    Down,
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
        }
    }
}
