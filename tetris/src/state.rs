use std::collections::{BinaryHeap, HashMap, VecDeque};

use crate::*;

pub struct State {
    pub board: Board,
    pub current_piece: piece::Piece,
    pub hold_piece: Option<Piece>,
    pub next_pieces: VecDeque<Piece>,
    pub combo: u32,
    pub b2b: bool,
    pub last_action: LastAction,
}

pub struct LastAction {
    pub placement_kind: PlacementKind,
    pub b2b: bool,
    pub combo: u32,
    pub perfect_clear: bool,
    pub garbage_sent: u32,
    pub time: Time,
    pub piece_movements: Vec<PieceMovement>,
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
    // TODO: implement
    // dijkstra's algorithm
    pub fn legal_actions(&self) -> Vec<State> {
        if self.next_pieces.is_empty() {
            return vec![];
        }

        let mut new_next_pieces = self.next_pieces.clone();

        let initial_movment_state = MovementState::new_from_piece(
            new_next_pieces.pop_front().unwrap(),
            self.hold_piece,
            new_next_pieces,
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
