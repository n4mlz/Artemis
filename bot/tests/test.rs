const EVALUATOR_REPEAT: u32 = 10;
const EVALUATOR_TIME: u32 = 1000;

#[test]
fn search_halting() {
    let state = tetris::State::new_random_state();

    let bot = bot::Bot {
        evaluator: bot::Evaluator::default(),
    };

    let next_state = bot
        .get_move_for_count(state.clone(), EVALUATOR_REPEAT)
        .unwrap();

    assert!(next_state != state);
}

// this is not a test, but for checking the display
#[test]
fn bot_play_for_count() {
    let mut current_state = tetris::State::new_random_state();

    let bot = bot::Bot {
        evaluator: bot::Evaluator::default(),
    };

    loop {
        println!("{}", termion::clear::All);
        println!("{}", current_state);
        bot::debug_evaluation(&current_state);

        if let Some(next_state) = bot.get_move_for_count(current_state.clone(), EVALUATOR_REPEAT) {
            current_state = next_state.clone();
            if current_state.next_pieces.len() < 8 {
                current_state.extend_next_pieces();
            }
        } else {
            break;
        }
    }
}

// this is not a test, but for checking the display
#[test]
fn bot_play_for_time() {
    let mut current_state = tetris::State::new_random_state();

    let bot = bot::Bot {
        evaluator: bot::Evaluator::default(),
    };

    loop {
        println!("{}", termion::clear::All);
        println!("{}", current_state);
        bot::debug_evaluation(&current_state);

        if let Some(next_state) = bot.get_move_for_time(current_state.clone(), EVALUATOR_TIME) {
            current_state = next_state.clone();
            if current_state.next_pieces.len() < 8 {
                current_state.extend_next_pieces();
            }
        } else {
            break;
        }
    }
}
