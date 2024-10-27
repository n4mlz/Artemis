use bot::Evaluator;
use rand::{thread_rng, Rng};

pub trait Gene {
    fn generate() -> Self;
    fn crossover(parent1: &Self, parent2: &Self) -> Self;
}

impl Gene for i32 {
    fn generate() -> Self {
        thread_rng().gen_range(-999..=999)
    }

    fn crossover(v1: &Self, v2: &Self) -> i32 {
        let mut rng = thread_rng();
        let v = match rng.gen_range(0..100) {
            0..=41 => *v1,                  // 42%
            42..=83 => *v2,                 // 42%
            84..=98 => (v1 + v2) / 2,       // 15%
            _ => rng.gen_range(-999..=999), // 1%
        } + rng.gen_range(-10..=10);

        v.clamp(-999, 999)
    }
}

impl Gene for Evaluator {
    fn generate() -> Self {
        // TODO: implement
        Evaluator {}
    }

    fn crossover(parent1: &Self, parent2: &Self) -> Self {
        // TODO: implement
        Evaluator {}
    }
}
