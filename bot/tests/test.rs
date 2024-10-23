use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;
use strum::IntoEnumIterator;
use tetris::FieldCells;

#[test]
fn search_halting() {
    let mut next_pieces: Vec<_> = tetris::Piece::iter().collect();
    let mut rng = thread_rng();
    next_pieces.shuffle(&mut rng);
    let mut next_pieces = VecDeque::from(next_pieces);

    let state = tetris::State {
        board: tetris::Board::new(),
        current_piece: Some(next_pieces.pop_front().unwrap()),
        hold_piece: None,
        next_pieces,
        b2b: false,
        last_action: None,
    };

    let bot = bot::Bot {
        evaluator: bot::Evaluator {},
    };

    let next_state = bot.decide_next_state(state.clone()).unwrap();

    assert!(next_state != state);
}

// this is not a test, but for checking the display
#[test]
fn bot_play() {
    let mut next_pieces: Vec<_> = tetris::Piece::iter().collect();
    let mut rng = thread_rng();
    next_pieces.shuffle(&mut rng);
    let mut next_pieces = VecDeque::from(next_pieces);

    let mut current_state = tetris::State {
        board: tetris::Board::new(),
        current_piece: Some(next_pieces.pop_front().unwrap()),
        hold_piece: None,
        next_pieces,
        b2b: false,
        last_action: None,
    };

    let bot = bot::Bot {
        evaluator: bot::Evaluator {},
    };

    loop {
        println!("{}", termion::clear::All);
        println!("{}", current_state);

        if let Some(next_state) = bot.decide_next_state(current_state.clone()) {
            current_state = next_state.clone();
            if current_state.next_pieces.len() < 8 {
                current_state.extend_next_pieces();
            }
        } else {
            break;
        }
    }
}
