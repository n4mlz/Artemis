use crate::*;
use tetris::State;

pub struct Node<'a> {
    evaluator: &'a Evaluator,
    state: State,
    reward: Reward,
    value: Value,
    n: u32,
    children: Vec<Node<'a>>,
}

impl Node<'_> {
    fn ucb(&self, parent_n: u32) -> Score {
        static C: u32 = 100;

        let log_parent_n = 32 - parent_n.leading_zeros();

        // TODO: make it a lightweight calculation
        self.reward + self.value + (((C * log_parent_n) as f64 / self.n as f64).sqrt()) as i32
    }

    fn expand(&mut self) {
        let legal_actions = self.state.legal_actions();
        for action in legal_actions {
            let (reward, value) = self.evaluator.evaluate(&action);
            self.children.push(Node {
                evaluator: self.evaluator,
                state: action,
                reward,
                value,
                n: 1,
                children: vec![],
            });
        }
    }

    fn search(&mut self) -> (Reward, Value) {
        // TODO: implement
        (0, 0)
    }
}
