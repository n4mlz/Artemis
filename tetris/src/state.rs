use crate::*;

pub struct State {
    pub board: Box<dyn Board>,
    pub current: piece::Piece,
    pub hold: Option<Piece>,
    pub next: Vec<Piece>,
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
    pub operations: Vec<PieceMovement>,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PieceMovement {
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateLeft,  // Counter-clockwise
    RotateRight, // Clockwise
    Hold,
}
