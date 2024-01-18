#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Score {
    pub cleared_rows: usize,
    pub score: usize,
    pub dropped_pieces: usize,
    pub fours: usize,
    pub threes: usize,
    pub twos: usize,
    pub ones: usize,
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
        };

        match n_cleans {
            4 => self.fours += 1,
            3 => self.threes += 1,
            2 => self.twos += 1,
            1 => self.ones += 1,
            _ => {}
        }
    }

    pub fn on_drop(&mut self) {
        self.dropped_pieces += 1;
    }
}
