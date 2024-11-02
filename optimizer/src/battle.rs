const MARGIN_TIME: u32 = 1000;
// gababe increase rate per 1000 time
const GABAGE_INCREASE: f64 = 1.1;

// returns true if p1 wins, false if p2 wins
// urrent implementation has the attack occur at the end of the turn with the attack (just before the next move begins)
// TODO: allow more accurate simulation of attack timing
pub fn do_battle(p1: &bot::Bot, p2: &bot::Bot, debug: bool) -> bool {
    let mut p1_state = tetris::State::new_random_state();
    let mut p2_state = tetris::State::new_random_state();

    let mut p1_time = 0;
    let mut p2_time = 0;

    let mut p1_attack = 0;
    let mut p2_attack = 0;

    // garbage that a player has not yet received
    // ex: if p1 sends 2 garbage, p2_garbage will be 2 until p2 receives it
    let mut p1_garbage = 0;
    let mut p2_garbage = 0;

    loop {
        if p1_time <= p2_time {
            let increase_rate = GABAGE_INCREASE
                .powf(((p1_time as i32 - MARGIN_TIME as i32).max(0) as f64) / 1000.0);
            p1_attack = (p1_attack as f64 * increase_rate).round() as u32;

            if p1_garbage as i32 - p1_attack as i32 > 0 {
                p1_state.receive_garbage(p1_garbage - p1_attack);
                p1_garbage = 0;
                p1_attack = 0;
            } else {
                p2_garbage += p1_attack - p1_garbage;
                p1_garbage = 0;
                p1_attack = 0;
            }

            if p1_state.is_dead() {
                return false;
            }

            if let Some(new_state) = p1.get_move(p1_state.clone()) {
                p1_state = new_state;
                if p1_state.next_pieces.len() < 8 {
                    p1_state.extend_next_pieces();
                }
            } else {
                return false;
            }

            p1_time += p1_state.last_action.clone().unwrap().time;
            p1_attack += p1_state.last_action.clone().unwrap().garbage_sent;
        } else {
            let increase_rate = GABAGE_INCREASE
                .powf(((p2_time as i32 - MARGIN_TIME as i32).max(0) as f64) / 1000.0);
            p2_attack = (p2_attack as f64 * increase_rate).round() as u32;

            if p2_garbage as i32 - p2_attack as i32 > 0 {
                p2_state.receive_garbage(p2_garbage - p2_attack);
                p2_garbage = 0;
                p2_attack = 0;
            } else {
                p1_garbage += p2_attack - p2_garbage;
                p2_garbage = 0;
                p2_attack = 0;
            }

            if p2_state.is_dead() {
                return true;
            }

            if let Some(new_state) = p2.get_move(p2_state.clone()) {
                p2_state = new_state;
                if p2_state.next_pieces.len() < 8 {
                    p2_state.extend_next_pieces();
                }
            } else {
                return true;
            }

            p2_time += p2_state.last_action.clone().unwrap().time;
            p2_attack += p2_state.last_action.clone().unwrap().garbage_sent;
        }

        if debug {
            println!("{}", termion::clear::All);
            println!("{}", tetris::PairState(p1_state.clone(), p2_state.clone()));
        }
    }
}
