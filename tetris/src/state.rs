use crate::*;

struct State {
    board: Box<dyn Board>,
    current: piece::Piece,
    hold: Option<Piece>,
    next: Vec<Piece>,
    ren: u32,
    b2b: bool,
}
