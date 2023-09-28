#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Score {
    cleared_rows: usize,
    score: usize,
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
}
