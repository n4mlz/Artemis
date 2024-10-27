use rand::{seq::SliceRandom, thread_rng};
use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
};

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

// this is not a test, but for checking the display
#[test]
fn random_play() {
    let mut rng = thread_rng();
    let mut current_state = tetris::State::new_random_state();

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
    }
}

// this is not a test, but for checking garbage
// place 3 pieces and receive 5 garbage
#[test]
fn receive_garbage() {
    let mut rng = thread_rng();
    let mut current_state = tetris::State::new_random_state();

    for _ in 0..3 {
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
    }

    current_state.receive_garbage(5);

    println!("{}", termion::clear::All);
    println!("{}", current_state);
}

// this is not a test, but for checking the display
#[test]
fn display_two_states() {
    let mut rng = thread_rng();
    let mut p1 = tetris::State::new_random_state();
    let mut p2 = tetris::State::new_random_state();

    loop {
        println!("{}", termion::clear::All);
        println!("{}", tetris::PairState(p1.clone(), p2.clone()));

        let legal_actions = p1.legal_actions();
        if let Some(next_state) = legal_actions.choose(&mut rng) {
            p1 = next_state.clone();
            if p1.next_pieces.len() < 8 {
                p1.extend_next_pieces();
            }
        } else {
            break;
        }

        let legal_actions = p2.legal_actions();
        if let Some(next_state) = legal_actions.choose(&mut rng) {
            p2 = next_state.clone();
            if p2.next_pieces.len() < 8 {
                p2.extend_next_pieces();
            }
        } else {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
