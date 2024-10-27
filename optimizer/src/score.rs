use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Score {
    matches: u32,
    wins: u32,
}

impl Score {
    pub fn new() -> Self {
        Score {
            matches: 0,
            wins: 0,
        }
    }

    pub fn update(&mut self, win: bool) {
        self.matches += 1;
        if win {
            self.wins += 1;
        }
    }

    pub fn win_rate(&self) -> f64 {
        self.wins as f64 / self.matches as f64
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.win_rate().partial_cmp(&other.win_rate()).unwrap()
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
