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
