#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BranchingMode {
    #[default]
    Current,
    CurrentAndNext,
}
