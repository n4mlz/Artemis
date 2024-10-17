use crate::*;
use rand::Rng;
use tetris::State;

pub type Reward = i32;
pub type Value = i32;

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
