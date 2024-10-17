use crate::*;
use tetris::State;

pub struct Node<'a> {
    pub state: State,
    evaluator: &'a Evaluator,
    reward: Reward,
    value: Value,
    n: u32,
    children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(evaluator: &'a Evaluator, state: State) -> Node<'a> {
        let (reward, value) = evaluator.evaluate(&state);

        Node {
            evaluator,
            state,
            reward,
            value,
            n: 1,
            children: vec![],
        }
    }

    fn ucb(&self, parent_n: u32) -> Score {
        static C: u32 = 100;

        let log_parent_n = 32 - parent_n.leading_zeros();

        // TODO: make it a lightweight calculation
        self.reward + self.value + (((C * log_parent_n) as f64 / self.n as f64).sqrt()) as i32
    }

    pub fn best_child(&mut self) -> Option<&mut Node<'a>> {
        self.children
            .iter_mut()
            .max_by_key(|child| child.ucb(self.n))
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

    pub fn search(&mut self) -> Score {
        static GAMMA: f64 = 0.95;

        if self.n > 1 && self.children.is_empty() {
            return self.reward + self.value;
        }

        let updated_child_score = if self.children.is_empty() {
            self.expand();
            self.children
                .iter()
                .map(|child| child.reward + child.value)
                .max()
                .unwrap_or(self.reward + self.value)
        } else {
            let best_child = self.best_child().unwrap();
            best_child.search()
        };

        self.value = (self.value + (GAMMA * updated_child_score as f64) as Value) / 2;
        self.n += 1;
        self.reward + self.value
    }
}
