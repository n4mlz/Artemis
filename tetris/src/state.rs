use std::collections::{HashMap, VecDeque};

use crate::*;

pub struct State {
    pub board: Board,
    pub current: piece::Piece,
    pub hold: Option<Piece>,
    pub next: VecDeque<Piece>,
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
    pub fn legal_moves(&self) -> Vec<State> {
        if self.next.is_empty() {
            return vec![];
        }

        let new_piece = self.next[0];
        let initial_field_piece = FieldPiece::new_from_piece(new_piece);

        let mut queue = VecDeque::new();
        queue.push_back(initial_field_piece);

        let mut field_piece_operations = HashMap::new();
        field_piece_operations.insert(initial_field_piece, vec![]);
    }
}
