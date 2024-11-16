use itertools::Itertools;
use optimizer::*;

const EVALUATOR_REPEAT: u32 = 10;

#[test]
fn replay_population() {
    let path = "../population.json";

    let population = Population::load_or_generate(path);

    let (p1, p2) = population
        .members
        .iter()
        .sorted_by(|a, b| b.score.cmp(&a.score))
        .take(2)
        .next_tuple()
        .unwrap();

    let p1 = bot::Bot::new(p1.evaluator);
    let p2 = bot::Bot::new(p2.evaluator);

    optimizer::do_battle(&p1, &p2, true);
}

#[test]
fn replay_bot() {
    let path = "../population.json";

    let population = Population::load_or_generate(path);

    let bot = population.members.iter().max_by_key(|m| &m.score).unwrap();
    let bot = bot::Bot::new(bot.evaluator);

    let mut current_state = tetris::State::new_random_state();

    loop {
        println!("{}", termion::clear::All);
        println!("{}", current_state);
        bot::debug_evaluation(&current_state);

        if let Some(next_state) = bot.get_move_for_repeat(current_state.clone(), EVALUATOR_REPEAT) {
            current_state = next_state.clone();
            if current_state.next_pieces.len() < 8 {
                current_state.extend_next_pieces();
            }
        } else {
            break;
        }
    }
}
