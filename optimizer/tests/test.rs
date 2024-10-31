use itertools::Itertools;
use optimizer::*;

#[test]
fn replay_population() {
    let path = "../population.json";

    let population = Population::load_or_generate(path);

    // TODO: fix
    if population.generation == 0 {
        return;
    }

    let (p1, p2) = population
        .members
        .iter()
        .sorted_by(|a, b| {
            let a_score = a.score.as_ref().unwrap();
            let b_score = b.score.as_ref().unwrap();
            b_score.cmp(a_score)
        })
        .take(2)
        .next_tuple()
        .unwrap();

    let p1 = bot::Bot::new(p1.evaluator);
    let p2 = bot::Bot::new(p2.evaluator);

    optimizer::do_battle(&p1, &p2, true);
}
