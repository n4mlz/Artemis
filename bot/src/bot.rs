use std::time::Instant;

use crate::*;
use tetris::State;

pub struct Bot {
    pub evaluator: Evaluator,
}

impl Bot {
    pub fn new(evaluator: Evaluator) -> Self {
        Self { evaluator }
    }

    pub fn get_move_for_count(&self, state: State, count: u32) -> Option<State> {
        let mut root = Node::new(&self.evaluator, state);

        for _ in 0..count {
            root.search();
        }

        root.best_child().map(|best_child| best_child.state.clone())
    }

    pub fn get_move_for_time(&self, state: State, time: u32) -> Option<State> {
        let mut root = Node::new(&self.evaluator, state);

        let start = Instant::now();
        let duration = std::time::Duration::from_millis(time as u64);

        while start.elapsed() < duration {
            root.search();
        }

        root.best_child().map(|best_child| best_child.state.clone())
    }
}
