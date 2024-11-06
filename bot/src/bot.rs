use crate::*;
use tetris::State;

pub struct Bot {
    pub evaluator: Evaluator,
}

impl Bot {
    pub fn new(evaluator: Evaluator) -> Self {
        Self { evaluator }
    }

    pub fn get_move(&self, state: State) -> Option<State> {
        let mut root = Node::new(&self.evaluator, state);

        // TODO: manage by time measurement
        // TODO: make it possible to change the amount of repetition
        // for _ in 0..100 {
        root.search();
        // }

        match root.best_child() {
            Some(best_child) => Some(best_child.state.clone()),
            None => None,
        }
    }
}
