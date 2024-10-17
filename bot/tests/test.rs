use rand::{seq::SliceRandom, thread_rng};
use std::{collections::VecDeque, thread::sleep};
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

    let next_state = bot.search(state.clone()).unwrap();

    assert!(next_state != state);
}
