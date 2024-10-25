use crate::*;
use bot::Evaluator;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

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

    fn load_or_generate(path: &str) -> Self {
        // TODO: make sure it works correctly
        if let Ok(mut file) = File::open(path) {
            let mut json = String::new();
            let _ = file.read_to_string(&mut json);
            serde_json::from_str(&json).unwrap_or_else(|_| Self::generate())
        } else {
            Self::generate()
        }
    }

    fn save(&self, path: &str) {
        // TODO: make sure it works correctly
        let json = serde_json::to_string(self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
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
