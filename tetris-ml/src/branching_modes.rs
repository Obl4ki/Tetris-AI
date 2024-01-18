#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BranchingMode {
    #[default]
    Current,
    CurrentAndNext,
}

impl BranchingMode {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Current => Self::CurrentAndNext,
            Self::CurrentAndNext => Self::Current,
        }
    }
}
