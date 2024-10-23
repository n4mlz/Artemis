use crate::*;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct State {
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub hold_piece: Option<Piece>,
    pub next_pieces: VecDeque<Piece>,
    pub b2b: bool, // as a state
    pub last_action: Option<LastAction>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LastAction {
    pub placement_kind: PlacementKind,
    pub b2b: bool, // as an action
    pub combo: u32,
    pub perfect_clear: bool,
    pub garbage_sent: u32,
    pub time: Time,
    pub movements_history: Vec<PieceMovement>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PlacementKind {
    None,
    Clear1,
    Clear2,
    Clear3,
    Clear4,
    MiniTspin,
    MiniTspin1,
    MiniTspin2,
    Tspin,
    Tspin1,
    Tspin2,
    Tspin3,
}

fn is_b2b_enabled(placement_kind: PlacementKind) -> bool {
    use PlacementKind::*;
    matches!(
        placement_kind,
        Clear4 | MiniTspin | MiniTspin1 | MiniTspin2 | Tspin | Tspin1 | Tspin2 | Tspin3
    )
}

impl State {
    pub fn new_random_state() -> Self {
        let mut next_pieces: Vec<_> = Piece::iter().collect();
        let mut rng = thread_rng();
        next_pieces.shuffle(&mut rng);
        let mut next_pieces = VecDeque::from(next_pieces);

        State {
            board: Board::new(),
            current_piece: Some(next_pieces.pop_front().unwrap()),
            hold_piece: None,
            next_pieces,
            b2b: false,
            last_action: None,
        }
    }

    pub fn extend_next_pieces(&mut self) {
        let mut new_next_pieces: Vec<_> = Piece::iter().collect();
        let mut rng = thread_rng();
        new_next_pieces.shuffle(&mut rng);
        self.next_pieces.extend(new_next_pieces);
    }

    pub fn receive_garbage(&mut self, garbage: u32) {
        self.board = self.board.receive_garbage(garbage);
    }

    pub fn is_dead(&self) -> bool {
        let initial_movment_state = MovementState::new_from_piece(
            self.current_piece.unwrap(),
            self.hold_piece,
            self.next_pieces.clone(),
        );

        !self.board.attempt(initial_movment_state.field_piece)
    }

    fn next_state(&self, mut movement_state: MovementState, time: Time) -> State {
        use PlacementKind::*;

        let (new_board, placement_kind) = self.board.place_piece(movement_state.field_piece);

        if placement_kind == PlacementKind::None {
            return State {
                board: new_board,
                current_piece: movement_state.next_pieces.pop_front(),
                hold_piece: movement_state.hold_piece,
                next_pieces: movement_state.next_pieces,
                b2b: is_b2b_enabled(placement_kind),
                last_action: Some(LastAction {
                    placement_kind,
                    b2b: false,
                    combo: 0,
                    perfect_clear: false,
                    garbage_sent: 0,
                    time: time + DEFAULT_ACTION_TIME.place,
                    movements_history: movement_state.movements_history,
                }),
            };
        }

        if new_board.is_empty() {
            return State {
                board: new_board,
                current_piece: movement_state.next_pieces.pop_front(),
                hold_piece: movement_state.hold_piece,
                next_pieces: movement_state.next_pieces,
                b2b: is_b2b_enabled(placement_kind),
                last_action: Some(LastAction {
                    placement_kind,
                    b2b: false,
                    combo: 1,
                    perfect_clear: true,
                    garbage_sent: DEFAULT_SPECIAL_ATTACK.perfect_clear,
                    time: time + DEFAULT_ACTION_TIME.perfect_clear,
                    movements_history: movement_state.movements_history,
                }),
            };
        }

        let base_attack = base_attack(placement_kind);
        let combo = match &self.last_action {
            Some(last_action) => last_action.combo + 1,
            Option::None => 0,
        };
        let combo_attack = combo_attack(combo);
        let b2b_attack = if self.b2b && is_b2b_enabled(placement_kind) {
            DEFAULT_SPECIAL_ATTACK.b2b
        } else {
            0
        };
        let garbage_sent = base_attack + combo_attack + b2b_attack;
        let action_time = match placement_kind {
            Clear1 | MiniTspin1 | Tspin1 => DEFAULT_ACTION_TIME.single,
            Clear2 | MiniTspin2 | Tspin2 => DEFAULT_ACTION_TIME.double,
            Clear3 | Tspin3 => DEFAULT_ACTION_TIME.triple,
            Clear4 => DEFAULT_ACTION_TIME.tetris,
            _ => DEFAULT_ACTION_TIME.place,
        };

        State {
            board: new_board,
            current_piece: movement_state.next_pieces.pop_front(),
            hold_piece: movement_state.hold_piece,
            next_pieces: movement_state.next_pieces,
            b2b: is_b2b_enabled(placement_kind),
            last_action: Some(LastAction {
                placement_kind,
                b2b: self.b2b && is_b2b_enabled(placement_kind),
                combo,
                perfect_clear: false,
                garbage_sent,
                time: time + action_time,
                movements_history: movement_state.movements_history,
            }),
        }
    }

    // dijkstra's algorithm
    pub fn legal_actions(&self) -> Vec<State> {
        if self.current_piece.is_none() || self.is_dead() {
            return vec![];
        }

        let initial_movment_state = MovementState::new_from_piece(
            self.current_piece.unwrap(),
            self.hold_piece,
            self.next_pieces.clone(),
        );

        // priority queue
        let mut queue = BinaryHeap::new();
        let mut movement_times = HashMap::new();

        queue.push(initial_movment_state.clone());
        movement_times.insert(initial_movment_state.clone(), 0);

        if let Some(held_movement_state) = initial_movment_state.hold() {
            if self.board.attempt(held_movement_state.field_piece) {
                let held_movement_time = held_movement_state.time;
                queue.push(held_movement_state.clone());
                movement_times.insert(held_movement_state, held_movement_time);
            }
        };

        while let Some(current_movement_state) = queue.pop() {
            let current_movement_time = current_movement_state.time;

            if let Some(best_time) = movement_times.get(&current_movement_state) {
                if current_movement_time > *best_time {
                    continue;
                }
            };

            for next_movement_state in self.board.legal_moves(current_movement_state) {
                let next_time = current_movement_time + next_movement_state.time;

                if let Some(best_time) = movement_times.get(&next_movement_state) {
                    if next_time >= *best_time {
                        continue;
                    }
                }

                movement_times.insert(next_movement_state.clone(), next_time);
                queue.push(next_movement_state);
            }
        }

        movement_times
            .into_iter()
            .filter(|(movement_state, _)| movement_state.field_piece.is_locked)
            .map(|(movement_state, time)| self.next_state(movement_state, time))
            .collect()
    }
}
