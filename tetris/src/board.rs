use crate::*;
use std::{collections::VecDeque, hash::Hash};

pub type Time = u32;

#[derive(Debug, Clone)]
pub struct MovementState {
    pub field_piece: FieldPiece,
    pub hold_piece: Option<Piece>,
    pub movements_history: Vec<PieceMovement>,
    pub next_pieces: VecDeque<Piece>,
    pub has_held: bool,
    pub time: Time,
}

impl MovementState {
    pub fn new_from_piece(
        current_piece: Piece,
        hold_piece: Option<Piece>,
        next_pieces: VecDeque<Piece>,
    ) -> MovementState {
        MovementState {
            field_piece: FieldPiece::new_from_piece(current_piece),
            hold_piece,
            next_pieces,
            movements_history: vec![],
            has_held: false,
            time: 0,
        }
    }

    fn next_movement_state(
        &self,
        field_piece: FieldPiece,
        piece_movement: PieceMovement,
        time: Time,
    ) -> MovementState {
        let mut new_movements_history = self.movements_history.clone();
        new_movements_history.push(piece_movement);

        MovementState {
            field_piece,
            hold_piece: self.hold_piece,
            next_pieces: self.next_pieces.clone(),
            movements_history: new_movements_history,
            has_held: self.has_held,
            time: self.time + time,
        }
    }

    fn next_movement_state_with_movements(
        &self,
        field_piece: FieldPiece,
        movements: Vec<PieceMovement>,
        time: Time,
    ) -> MovementState {
        let mut new_movements_history = self.movements_history.clone();
        new_movements_history.extend(movements);

        MovementState {
            field_piece,
            hold_piece: self.hold_piece,
            next_pieces: self.next_pieces.clone(),
            movements_history: new_movements_history,
            has_held: self.has_held,
            time: self.time + time,
        }
    }

    pub fn hold(&self) -> Option<MovementState> {
        if self.has_held || !self.movements_history.is_empty() {
            return None;
        }

        match self.hold_piece {
            Some(hold_piece) => Some(MovementState {
                field_piece: FieldPiece::new_from_piece(hold_piece),
                hold_piece: Some(self.field_piece.piece_state.piece),
                next_pieces: self.next_pieces.clone(),
                movements_history: vec![PieceMovement::Hold],
                has_held: true,
                time: DEFAULT_ACTION_TIME.hold,
            }),

            None => {
                if self.next_pieces.is_empty() {
                    return None;
                }
                let mut new_next_pieces = self.next_pieces.clone();
                Some(MovementState {
                    field_piece: FieldPiece::new_from_piece(new_next_pieces.pop_front().unwrap()),
                    hold_piece: Some(self.field_piece.piece_state.piece),
                    next_pieces: new_next_pieces,
                    movements_history: vec![PieceMovement::Hold],
                    has_held: true,
                    time: DEFAULT_ACTION_TIME.hold,
                })
            }
        }
    }
}

impl Ord for MovementState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for MovementState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for MovementState {}

impl PartialEq for MovementState {
    fn eq(&self, other: &Self) -> bool {
        self.field_piece == other.field_piece
            && self.hold_piece == other.hold_piece
            && self.next_pieces == other.next_pieces
            && self.has_held == other.has_held
    }
}

impl Hash for MovementState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.field_piece.hash(state);
        self.hold_piece.hash(state);
        self.next_pieces.hash(state);
        self.has_held.hash(state);
    }
}

fn row_x(x: i32) -> u16 {
    1 << (15 - x)
}

pub type Board = [u16; 40];

pub trait FieldCells {
    fn new() -> Self;
    fn occupied(&self, x: i32, y: i32) -> bool;
    fn is_empty(&self) -> bool;
    fn attempt(&self, field_piece: FieldPiece) -> bool;
    fn legal_moves(&self, movement_state: MovementState) -> Vec<MovementState>;
    fn place_piece(&self, field_piece: FieldPiece) -> (Board, PlacementKind);
}

impl FieldCells for Board {
    fn new() -> Board {
        [0; 40]
    }

    fn occupied(&self, x: i32, y: i32) -> bool {
        !(0..10).contains(&x) || !(0..40).contains(&y) || (self[y as usize] & row_x(x) > 0)
    }

    fn is_empty(&self) -> bool {
        self[39] == 0
    }

    fn attempt(&self, field_piece: FieldPiece) -> bool {
        field_piece
            .cells()
            .iter()
            .all(|&(x, y)| !self.occupied(x, y))
    }

    fn legal_moves(&self, movement_state: MovementState) -> Vec<MovementState> {
        if movement_state.field_piece.is_locked {
            return vec![];
        }

        let mut result = vec![];

        // TODO: limit the number of lateral moves and rotations at the same height (game rules)
        for piece_movement in [
            PieceMovement::MoveLeft,
            PieceMovement::MoveRight,
            PieceMovement::SoftDrop,
            PieceMovement::HardDrop,
            PieceMovement::RotateLeft,
            PieceMovement::RotateRight,
        ] {
            match piece_movement {
                PieceMovement::MoveLeft => {
                    let new_field_piece = movement_state.field_piece.move_by(-1, 0);
                    if self.attempt(new_field_piece) {
                        result.push(movement_state.next_movement_state(
                            new_field_piece,
                            piece_movement,
                            DEFAULT_ACTION_TIME.move_one,
                        ));
                    }
                }

                PieceMovement::MoveRight => {
                    let new_field_piece = movement_state.field_piece.move_by(1, 0);
                    if self.attempt(new_field_piece) {
                        result.push(movement_state.next_movement_state(
                            new_field_piece,
                            piece_movement,
                            DEFAULT_ACTION_TIME.move_one,
                        ));
                    }
                }

                // MEMO: not currently supported for soft drop of only one cell
                PieceMovement::SoftDrop => {
                    let mut new_field_piece = movement_state.field_piece;
                    let mut count = 0;
                    while self.attempt(new_field_piece.move_by(0, 1)) {
                        new_field_piece = new_field_piece.move_by(0, 1);
                        count += 1;
                    }
                    if count > 0 {
                        result.push(movement_state.next_movement_state_with_movements(
                            new_field_piece,
                            vec![piece_movement; count as usize],
                            DEFAULT_ACTION_TIME.move_one * count,
                        ));
                    }
                }

                PieceMovement::HardDrop => {
                    let mut new_field_piece = movement_state.field_piece;
                    while self.attempt(new_field_piece.move_by(0, 1)) {
                        new_field_piece = new_field_piece.move_by(0, 1);
                    }
                    let mut new_movement_state = movement_state.next_movement_state(
                        new_field_piece,
                        piece_movement,
                        DEFAULT_ACTION_TIME.hard_drop,
                    );
                    new_movement_state.field_piece.is_locked = true;
                    result.push(new_movement_state);
                }

                // TODO: SRS
                PieceMovement::RotateLeft => {
                    let new_field_piece = movement_state.field_piece.rotate_left();
                    if self.attempt(new_field_piece) {
                        result.push(movement_state.next_movement_state(
                            new_field_piece,
                            piece_movement,
                            DEFAULT_ACTION_TIME.rotate,
                        ));
                    }
                }

                // TODO: SRS
                PieceMovement::RotateRight => {
                    let new_field_piece = movement_state.field_piece.rotate_right();
                    if self.attempt(new_field_piece) {
                        result.push(movement_state.next_movement_state(
                            new_field_piece,
                            piece_movement,
                            DEFAULT_ACTION_TIME.rotate,
                        ));
                    }
                }

                // MEMO: holds are not searched here because next must also be considered
                _ => {}
            }
        }

        result
    }

    fn place_piece(&self, field_piece: FieldPiece) -> (Board, PlacementKind) {
        use PlacementKind::*;

        let mut new_board = *self; // copy
        for &(x, y) in field_piece.cells().iter() {
            new_board[y as usize] |= row_x(x);
        }

        let mut cleared_rows = 0;
        // reverse order to avoid shifting
        for y in (0..40).rev() {
            if new_board[y] == 0xffc0 {
                cleared_rows += 1;
            } else if cleared_rows > 0 {
                new_board[y + cleared_rows as usize] = new_board[y];
            }
        }
        for y in 0..cleared_rows {
            new_board[y as usize] = 0;
        }

        if field_piece.piece_state.piece != Piece::T {
            let placement_kind = match cleared_rows {
                0 => None,
                1 => Clear1,
                2 => Clear2,
                3 => Clear3,
                4 => Clear4,
                _ => None,
            };

            return (new_board, placement_kind);
        }

        match field_piece.super_rotation_state {
            SuperRotationState::None => {
                let placement_kind = match cleared_rows {
                    0 => None,
                    1 => Clear1,
                    2 => Clear2,
                    3 => Clear3,
                    4 => Clear4,
                    _ => None,
                };

                (new_board, placement_kind)
            }
            SuperRotationState::Mini => {
                let placement_kind = match cleared_rows {
                    0 => MiniTspin,
                    1 => MiniTspin1,
                    2 => MiniTspin2,
                    _ => MiniTspin,
                };

                (new_board, placement_kind)
            }
            SuperRotationState::Normal => {
                let placement_kind = match cleared_rows {
                    0 => Tspin,
                    1 => Tspin1,
                    2 => Tspin2,
                    3 => Tspin3,
                    _ => Tspin,
                };

                (new_board, placement_kind)
            }
        }
    }
}
