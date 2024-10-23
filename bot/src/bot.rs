use crate::*;
use tetris::State;

pub struct Bot {
    pub evaluator: Evaluator,
}

impl Bot {
    pub fn decide_next_state(&self, state: State) -> Option<State> {
        let mut root = Node::new(&self.evaluator, state);

        // TODO: manage by time measurement
        for _ in 0..100 {
            root.search();
        }

        match root.best_child() {
            Some(best_child) => Some(best_child.state.clone()),
            None => None,
        }
    }
}
