use serde::{Deserialize, Serialize};

// the attenuation rate of attack when losing
const LOSE_RATE: f64 = 0.5;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Score {
    time: u32,
    attack: u32,
}

impl Score {
    pub fn new() -> Self {
        Score { time: 0, attack: 0 }
    }

    pub fn update(&mut self, attack: u32, time: u32, win: bool) {
        self.time += time;
        self.attack += if win {
            attack
        } else {
            (attack as f64 * LOSE_RATE) as u32
        };
    }

    fn attack_per_time(&self) -> f64 {
        if self.time == 0 {
            0.0
        } else {
            self.attack as f64 / self.time as f64
        }
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.attack_per_time()
            .partial_cmp(&other.attack_per_time())
            .unwrap()
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
