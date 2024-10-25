use crate::*;
use bot::Evaluator;
use serde::{Deserialize, Serialize};

const POPULATION_SIZE: usize = 100;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct Population {
    generation: u32,
    members: Vec<Member>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct Member {
    evaluator: Evaluator,
    score: Option<u32>,
}

impl Population {
    fn generate() -> Self {
        let mut members = vec![];
        for _ in 0..POPULATION_SIZE {
            members.push(Member {
                evaluator: Evaluator::generate(),
                score: None,
            });
        }

        Population {
            generation: 0,
            members,
        }
    }

    fn save(&self, path: &str) {
        // TODO: implement
    }

    fn load(path: &str) -> Self {
        // TODO: implement
        Self::generate()
    }

    fn evaluate(&mut self) {
        // TODO: implement
    }

    fn select(&self) -> (&Member, &Member) {
        // TODO: implement
        (&self.members[0], &self.members[1])
    }

    fn crossover(&self) -> Self {
        // TODO: implement
        Self::generate()
    }
}
