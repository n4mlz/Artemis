use crate::*;
use tetris::State;

// parameter to balance exploration and exploitation
// a larger value increases randomness in selections
const C: u32 = 100;

// discount factor for future rewards
const GAMMA: f64 = 0.9;

// update rate for value
const UPDATE_RATE: f64 = 0.1;

pub struct Node<'a> {
    pub state: State,
    evaluator: &'a Evaluator,
    reward: Reward,
    value: Value,
    pub n: u32,
    children: Vec<Node<'a>>,
    max_children_score: Option<Score>,
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
            max_children_score: None,
        }
    }

    fn ucb(&self, parent_n: u32) -> Score {
        let log_parent_n = 32 - parent_n.leading_zeros();

        // TODO: make it a lightweight calculation
        self.reward + self.value + (((C * log_parent_n) as f64 / self.n as f64).sqrt()) as i32
    }

    pub fn best_child(&self) -> Option<&Node<'a>> {
        self.children
            .iter()
            .max_by_key(|child| child.reward + child.value)
    }

    pub fn best_ucb_child(&mut self) -> Option<&mut Node<'a>> {
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
                max_children_score: None,
            });
            self.max_children_score = Some(
                self.max_children_score
                    .unwrap_or(i32::MIN)
                    .max(reward + value),
            );
        }
    }

    pub fn search(&mut self) -> Score {
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
            let best_child = self.best_ucb_child().unwrap();
            best_child.search()
        };

        self.max_children_score = Some(
            self.max_children_score
                .unwrap_or(i32::MIN)
                .max(updated_child_score),
        );
        self.value = ((1.0 - UPDATE_RATE) * (self.value as f64)
            + UPDATE_RATE * (GAMMA * self.max_children_score.unwrap() as f64))
            as Value;
        self.n += 1;
        self.reward + self.value
    }
}
