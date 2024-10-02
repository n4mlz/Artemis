use std::collections::{BinaryHeap, HashMap, VecDeque};

use crate::*;

pub struct State {
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub hold_piece: Option<Piece>,
    pub next_pieces: VecDeque<Piece>,
    pub combo: u32,
    pub b2b: bool, // as a state
    pub last_action: LastAction,
}

pub struct LastAction {
    pub placement_kind: PlacementKind,
    pub b2b: bool, // whether used or not
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

impl State {
    fn next_state(&self, mut movement_state: MovementState, time: Time) -> State {
        let (new_board, placement_kind) = self.board.place_piece(movement_state.field_piece);

        if placement_kind == PlacementKind::None {
            return State {
                board: new_board,
                current_piece: movement_state.next_pieces.pop_front(),
                hold_piece: movement_state.hold_piece,
                next_pieces: movement_state.next_pieces,
                combo: 0,
                b2b: false,
                last_action: LastAction {
                    placement_kind,
                    b2b: false,
                    combo: 0,
                    perfect_clear: false,
                    garbage_sent: 0,
                    time: time + DEFAULT_ACTION_TIME.place,
                    movements_history: movement_state.movements_history,
                },
            };
        }

        // TODO: implement
        State {
            board: new_board,
            current_piece: movement_state.next_pieces.pop_front(),
            hold_piece: movement_state.hold_piece,
            next_pieces: movement_state.next_pieces,
            combo: 0,
            b2b: false,
            last_action: LastAction {
                placement_kind,
                b2b: false,
                combo: 0,
                perfect_clear: false,
                garbage_sent: 0,
                time: time + DEFAULT_ACTION_TIME.place,
                movements_history: movement_state.movements_history,
            },
        }
    }

    // TODO: implement
    // dijkstra's algorithm
    pub fn legal_actions(&self) -> Vec<State> {
        if self.current_piece.is_none() {
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

        queue.push(MovementWithTime {
            movement_state: initial_movment_state.clone(),
            time: 0,
        });
        movement_times.insert(initial_movment_state.clone(), 0);

        if let Some(held_movement_with_time) = initial_movment_state.hold() {
            queue.push(held_movement_with_time.clone());
            movement_times.insert(
                held_movement_with_time.movement_state,
                held_movement_with_time.time,
            );
        };

        while let Some(MovementWithTime {
            movement_state: current_movement_state,
            time: current_time,
        }) = queue.pop()
        {
            if let Some(best_time) = movement_times.get(&current_movement_state) {
                if current_time > *best_time {
                    continue;
                }
            };

            for next_movement in self.board.legal_moves(current_movement_state) {
                let next_movement_state = next_movement.movement_state;
                let next_time = current_time + next_movement.time;

                if let Some(best_time) = movement_times.get(&next_movement_state) {
                    if next_time >= *best_time {
                        continue;
                    }
                }

                movement_times.insert(next_movement_state.clone(), next_time);
                queue.push(MovementWithTime {
                    movement_state: next_movement_state,
                    time: next_time,
                });
            }
        }

        // TODO: implement
        vec![]
    }
}
