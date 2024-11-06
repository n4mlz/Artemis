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
const POPULATION_SIZE: usize = 50;
// adjust the accuracy of the evaluation
const MATCH_COUNT: usize = 4;
// adjust selection pressure
const SELECTION_SIZE: usize = 5;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Population {
    pub generation: u32,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Member {
    pub evaluator: Evaluator,
    pub score: Option<Score>,
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
        if let Ok(mut file) = File::open(path) {
            let mut json = String::new();
            let _ = file.read_to_string(&mut json);
            serde_json::from_str(&json).unwrap_or_else(|_| Self::generate())
        } else {
            Self::generate()
        }
    }

    pub fn save(&self, path: &str) {
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
                if self.members[j].score.is_none() {
                    self.members[j].score = Some(Score::new());
                }

                let p1 = Bot::new(self.members[i].evaluator);
                let p2 = Bot::new(self.members[j].evaluator);

                let win = do_battle(&p1, &p2, false);
                self.members[i].score.as_mut().unwrap().update(win);
                self.members[j].score.as_mut().unwrap().update(!win);
            }

            debug_optimizer(self.generation, i);
        }
    }

    fn select(&self) -> (&Member, &Member) {
        let mut rng = thread_rng();
        let group = self.members.choose_multiple(&mut rng, SELECTION_SIZE);
        group
            .sorted_by(|a, b| {
                let a_score = a.score.as_ref().unwrap();
                let b_score = b.score.as_ref().unwrap();
                b_score.cmp(a_score)
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
                b_score.cmp(a_score)
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

    pub fn optimize(&mut self) -> Self {
        self.evaluate();
        self.crossover()
    }
}

fn debug_optimizer(generation: u32, member: usize) {
    println!("{}", termion::cursor::Show);
    println!("generation: {}", generation);
    println!(
        "member: {} / {} ({:4.1} %)",
        member,
        POPULATION_SIZE,
        member as f64 / POPULATION_SIZE as f64 * 100.0
    );
    println!("{}{}", termion::cursor::Up(4), termion::cursor::Hide);
}
