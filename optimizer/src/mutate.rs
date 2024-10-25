use bot::Evaluator;

pub trait Gene {
    fn generate() -> Self;
    fn crossover(parent1: &Self, parent2: &Self) -> Self;
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
