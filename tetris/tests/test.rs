use crate::tetris::FieldCells;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;
use strum::IntoEnumIterator;
extern crate tetris;

// this is not a test, but for checking the display
#[test]
fn it_works() {
    let mut initial_next_pieces: Vec<_> = tetris::Piece::iter().collect();
    let mut rng = thread_rng();
    initial_next_pieces.shuffle(&mut rng);
    let mut initial_next_pieces = VecDeque::from(initial_next_pieces);

    let initial_state = tetris::State {
        board: tetris::Board::new(),
        current_piece: Some(initial_next_pieces.pop_front().unwrap()),
        hold_piece: None,
        next_pieces: initial_next_pieces,
        b2b: false,
        last_action: None,
    };

    print!("{}", initial_state);
}
