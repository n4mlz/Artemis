use rand::{seq::SliceRandom, thread_rng};
use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
    thread::sleep,
};
use strum::IntoEnumIterator;
use tetris::FieldCells;

// this is not a test, but for checking the display
#[test]
fn random_play() {
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

    loop {
        println!("{}", termion::clear::All);
        println!("{}", current_state);

        let legal_actions = current_state.legal_actions();
        if let Some(next_state) = legal_actions.choose(&mut rng) {
            current_state = next_state.clone();
            if current_state.next_pieces.len() < 8 {
                current_state.extend_next_pieces();
            }
        } else {
            break;
        }
        sleep(std::time::Duration::from_millis(50));
    }
}

#[test]
fn hash_of_movement_state() {
    let left_movement_state = tetris::MovementState::new_from_piece(
        tetris::Piece::I,
        Some(tetris::Piece::T),
        VecDeque::from(vec![tetris::Piece::L, tetris::Piece::J]),
    );

    let mut right_movement_state = left_movement_state.clone();

    let mut left_hasher = DefaultHasher::new();
    let mut right_hasher = DefaultHasher::new();

    left_movement_state.hash(&mut left_hasher);
    right_movement_state.hash(&mut right_hasher);

    assert_eq!(left_hasher.finish(), right_hasher.finish());

    right_movement_state
        .movements_history
        .push(tetris::PieceMovement::MoveRight);
    right_movement_state.time += 1;

    let mut right_hasher = DefaultHasher::new();
    right_movement_state.hash(&mut right_hasher);

    assert_eq!(left_hasher.finish(), right_hasher.finish());

    let right_movement_state = right_movement_state.hold();
    let mut right_hasher = DefaultHasher::new();
    right_movement_state.hash(&mut right_hasher);

    assert_ne!(left_hasher.finish(), right_hasher.finish());
}
