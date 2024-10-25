use rand::Rng;
use serde::{Deserialize, Serialize};
use tetris::State;

pub type Score = i32;
pub type Reward = Score;
pub type Value = Score;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Evaluator {
    // TODO: add some parameters
}

impl Evaluator {
    pub fn evaluate(&self, state: &State) -> (Reward, Value) {
        // TODO: implement
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..100), rng.gen_range(0..100))
    }
}
