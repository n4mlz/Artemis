const MAX_TIME: u32 = 1000000;

// returns true if p1 wins, false if p2 wins
pub fn do_battle(p1: &bot::Bot, p2: &bot::Bot) -> bool {
    let mut p1_state = tetris::State::new_random_state();
    let mut p2_state = tetris::State::new_random_state();

    let mut p1_time = 0;
    let mut p2_time = 0;

    // garbage that a player has not yet received
    // ex: if p1 sends 2 garbage, p2_garbage will be 2 until p2 receives it
    let mut p1_garbage = 0;
    let mut p2_garbage = 0;

    loop {
        if p1_time >= MAX_TIME && p2_time >= MAX_TIME {
            return p1_state.board.collumn_heights.iter().max().unwrap()
                <= p2_state.board.collumn_heights.iter().max().unwrap();
        }

        if p1_time <= p2_time {
            if p1_garbage > 0 {
                p2_state.receive_garbage(p1_garbage);
                p1_garbage = 0;
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
            p2_garbage += p1_state.last_action.clone().unwrap().garbage_sent;
        } else {
            if p2_garbage > 0 {
                p1_state.receive_garbage(p2_garbage);
                p2_garbage = 0;
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
            p1_garbage += p2_state.last_action.clone().unwrap().garbage_sent;
        }
    }
}
