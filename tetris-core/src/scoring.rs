use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize,
)]
pub struct Score {
    pub cleared_rows: usize,
    pub score: usize,
    pub dropped_pieces: usize,
}

impl Score {
    pub fn on_lines_clear(&mut self, n_cleans: usize) {
        self.cleared_rows += n_cleans;
        self.score += match n_cleans {
            4 => 800,
            3 => 500,
            2 => 300,
            1 => 100,
            _ => 0,
        }
    }

    pub fn on_drop(&mut self) {
        self.dropped_pieces += 1;
    }
}
