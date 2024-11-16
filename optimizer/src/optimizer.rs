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
    sync::{Arc, Mutex},
};

// adjust the diversity of the population
const POPULATION_SIZE: usize = 500;
// adjust the accuracy of the evaluation
const MATCH_COUNT: usize = 8;
// adjust selection pressure
const SELECTION_SIZE: usize = 30;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Population {
    pub generation: u32,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Member {
    pub evaluator: Evaluator,
    pub score: Score,
}

impl Population {
    fn generate() -> Self {
        let mut members = vec![];
        for _ in 0..POPULATION_SIZE {
            members.push(Member {
                evaluator: Evaluator::generate(),
                score: Score::new(),
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
        let members: Arc<Vec<Mutex<Member>>> =
            Arc::new(self.members.clone().into_iter().map(Mutex::new).collect());

        let generation = self.generation;
        let count = Arc::new(Mutex::new(0));

        {
            let thread_num = num_cpus::get();
            let pool = ThreadPool::new(thread_num);

            for i in 0..POPULATION_SIZE {
                let members = Arc::clone(&members);
                let count = Arc::clone(&count);

                pool.execute(move || {
                    let mut rng = thread_rng();
                    let opponents_index =
                        (0..POPULATION_SIZE).choose_multiple(&mut rng, MATCH_COUNT);

                    for j in opponents_index {
                        {
                            let p1 = Bot::new(members[i].lock().unwrap().evaluator);
                            let p2 = Bot::new(members[j].lock().unwrap().evaluator);

                            let (p1, p2) = do_battle(&p1, &p2, false);

                            members[i]
                                .lock()
                                .unwrap()
                                .score
                                .update(p1.attack, p1.time, p1.win);
                            members[j]
                                .lock()
                                .unwrap()
                                .score
                                .update(p2.attack, p2.time, p2.win);
                        }
                    }

                    let mut count = count.lock().unwrap();
                    *count += 1;
                    debug_optimizer(generation, *count);
                });
            }
        }

        self.members = members.iter().map(|m| m.lock().unwrap().clone()).collect();
    }

    fn select(&self) -> (&Member, &Member) {
        let mut rng = thread_rng();
        let group = self.members.choose_multiple(&mut rng, SELECTION_SIZE);
        group
            .sorted_by(|a, b| b.score.cmp(&a.score))
            .take(2)
            .next_tuple()
            .unwrap()
    }

    fn crossover(&self) -> Self {
        // elite
        let mut new_members: Vec<_> = self
            .members
            .iter()
            .sorted_by(|a, b| b.score.cmp(&a.score))
            .take(2)
            .cloned()
            .collect();

        // crossover
        while new_members.len() < POPULATION_SIZE {
            let (parent1, parent2) = self.select();
            let evaluator = Evaluator::crossover(&parent1.evaluator, &parent2.evaluator);
            new_members.push(Member {
                evaluator,
                score: Score::new(),
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
        "member: {:4} / {} ({:5.1} %)",
        member,
        POPULATION_SIZE,
        member as f64 / POPULATION_SIZE as f64 * 100.0
    );
    println!("{}{}", termion::cursor::Up(4), termion::cursor::Hide);
}
