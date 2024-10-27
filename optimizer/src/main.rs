mod battle;
mod mutate;
mod optimizer;
mod score;

pub use battle::*;
pub use mutate::*;
pub use optimizer::*;
pub use score::*;

fn main() {
    let path = "population.json";

    let mut population = Population::load_or_generate(path);

    loop {
        population.optimize();
        population.save(path);
    }
}
