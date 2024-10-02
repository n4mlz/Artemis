use std::collections::{HashSet, VecDeque};

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
    pub time: u32,
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
    // MEMO: dijkstra's algorithm
    pub fn legal_actions(&self) -> Vec<State> {
        if self.next_pieces.is_empty() {
            return vec![];
        }

        let initial_movment_state = MovementState::new_from_piece(self.next[0]);

        let mut queue = VecDeque::new();
        queue.push_back(new_moving_piece);

        let mut movements = HashSet::new();
        movements.insert(new_moving_piece);
    }
}
