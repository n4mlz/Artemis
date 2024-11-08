mod battle;
mod mutate;
mod optimizer;
mod score;
mod threadpool;

pub use battle::*;
pub use mutate::*;
pub use optimizer::*;
pub use score::*;
pub use threadpool::*;

fn main() {
    let path = "population.json";

    let mut population = Population::load_or_generate(path);

    loop {
        let new_population = population.optimize();
        population.save(path);
        population = new_population;
    }
}
