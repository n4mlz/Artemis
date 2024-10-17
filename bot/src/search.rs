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
    fn ucb(parent_n: u32) -> evaluate::Score {
        // TODO: implement
        0
    }

    fn search(&mut self) {
        // TODO: implement
    }

    fn expand(&mut self) {
        // TODO: implement
    }
}
