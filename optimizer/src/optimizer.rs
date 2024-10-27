use crate::*;
use bot::{Bot, Evaluator};
use itertools::Itertools;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

// adjust the diversity of the population
const POPULATION_SIZE: usize = 100;
// adjust the accuracy of the evaluation
const MATCH_COUNT: usize = 30;
// adjust selection pressure
const SELECTION_SIZE: usize = 10;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Population {
    generation: u32,
    members: Vec<Member>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct Member {
    evaluator: Evaluator,
    // numerator and denominator
    score: Option<Score>,
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

    pub fn load_or_generate(path: &str) -> Self {
        // TODO: make sure it works correctly
        if let Ok(mut file) = File::open(path) {
            let mut json = String::new();
            let _ = file.read_to_string(&mut json);
            serde_json::from_str(&json).unwrap_or_else(|_| Self::generate())
        } else {
            Self::generate()
        }
    }

    pub fn save(&self, path: &str) {
        // TODO: make sure it works correctly
        let json = serde_json::to_string(self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    fn evaluate(&mut self) {
        for i in 0..POPULATION_SIZE {
            if self.members[i].score.is_none() {
                self.members[i].score = Some(Score::new());
            }

            let mut rng = thread_rng();
            let opponents_index = (0..POPULATION_SIZE).choose_multiple(&mut rng, MATCH_COUNT);

            for j in opponents_index {
                let p1 = Bot::new(self.members[i].evaluator);
                let p2 = Bot::new(self.members[j].evaluator);

                let win = do_battle(&p1, &p2);
                self.members[i].score.as_mut().unwrap().update(win);
                self.members[j].score.as_mut().unwrap().update(!win);
            }
        }
    }

    fn select(&self) -> (&Member, &Member) {
        // TODO: make sure it works correctly
        let mut rng = thread_rng();
        let group = self.members.choose_multiple(&mut rng, SELECTION_SIZE);
        group
            .sorted_by(|a, b| {
                let a_score = a.score.as_ref().unwrap();
                let b_score = b.score.as_ref().unwrap();
                a_score.cmp(b_score)
            })
            .take(2)
            .next_tuple()
            .unwrap()
    }

    fn crossover(&self) -> Self {
        // elite
        let mut new_members: Vec<_> = self
            .members
            .iter()
            .sorted_by(|a, b| {
                let a_score = a.score.as_ref().unwrap();
                let b_score = b.score.as_ref().unwrap();
                a_score.cmp(b_score)
            })
            .take(2)
            .cloned()
            .collect();

        // crossover
        while new_members.len() < POPULATION_SIZE {
            let (parent1, parent2) = self.select();
            let evaluator = Evaluator::crossover(&parent1.evaluator, &parent2.evaluator);
            new_members.push(Member {
                evaluator,
                score: None,
            });
        }

        Population {
            generation: self.generation + 1,
            members: new_members,
        }
    }

    pub fn optimize(&mut self) {
        self.evaluate();
        *self = self.crossover();
    }
}
