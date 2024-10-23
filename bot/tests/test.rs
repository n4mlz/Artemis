#[test]
fn search_halting() {
    let state = tetris::State::new_random_state();

    let bot = bot::Bot {
        evaluator: bot::Evaluator {},
    };

    let next_state = bot.get_move(state.clone()).unwrap();

    assert!(next_state != state);
}

// this is not a test, but for checking the display
#[test]
fn bot_play() {
    let mut current_state = tetris::State::new_random_state();

    let bot = bot::Bot {
        evaluator: bot::Evaluator {},
    };

    loop {
        println!("{}", termion::clear::All);
        println!("{}", current_state);

        if let Some(next_state) = bot.get_move(current_state.clone()) {
            current_state = next_state.clone();
            if current_state.next_pieces.len() < 8 {
                current_state.extend_next_pieces();
            }
        } else {
            break;
        }
    }
}
