use std::collections::VecDeque;

use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MovementState {
    field_piece: FieldPiece,
    hold_piece: Option<Piece>,
    movements_history: Vec<PieceMovement>,
    next_pieces: VecDeque<Piece>,
    has_held: bool,
    is_locked: bool,
    time: u32,
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
            is_locked: false,
            time: 0,
        }
    }

    fn next_movement_state(
        &self,
        field_piece: FieldPiece,
        piece_movement: PieceMovement,
        time: u32,
    ) -> MovementState {
        let mut new_movements_history = self.movements_history.clone();
        new_movements_history.push(piece_movement);

        MovementState {
            field_piece,
            hold_piece: self.hold_piece,
            next_pieces: self.next_pieces.clone(),
            movements_history: new_movements_history,
            has_held: self.has_held,
            is_locked: self.is_locked,
            time: self.time + time,
        }
    }

    fn next_movement_state_with_movements(
        &self,
        field_piece: FieldPiece,
        movements: Vec<PieceMovement>,
        time: u32,
    ) -> MovementState {
        let mut new_movements_history = self.movements_history.clone();
        new_movements_history.extend(movements);

        MovementState {
            field_piece,
            hold_piece: self.hold_piece,
            next_pieces: self.next_pieces.clone(),
            movements_history: new_movements_history,
            has_held: self.has_held,
            is_locked: self.is_locked,
            time: self.time + time,
        }
    }

    pub fn hold(&self) -> Option<MovementState> {
        if self.has_held || !self.movements_history.is_empty() || self.time > 0 {
            return None;
        }

        match self.hold_piece {
            Some(hold_piece) => Some(MovementState {
                field_piece: FieldPiece::new_from_piece(hold_piece),
                hold_piece: Some(self.field_piece.piece_state.piece),
                next_pieces: self.next_pieces.clone(),
                movements_history: vec![PieceMovement::Hold],
                has_held: true,
                is_locked: false,
                time: DEFAULT_MOVEMENT_TIME.hold,
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
                    is_locked: false,
                    time: DEFAULT_MOVEMENT_TIME.hold,
                })
            }
        }
    }
}

fn row_x(x: i32) -> u16 {
    1 << (15 - x)
}

pub type Board = [u16; 40];

pub trait FieldCells {
    fn occupied(&self, x: i32, y: i32) -> bool;
    fn attempt(&self, field_piece: FieldPiece) -> bool;
    fn legal_moves(&self, movement_state: MovementState) -> Vec<MovementState>;
}

impl FieldCells for Board {
    fn occupied(&self, x: i32, y: i32) -> bool {
        x < 0 || x >= 10 || y < 0 || y >= 40 || (self[y as usize] & row_x(x) > 0)
    }

    fn attempt(&self, field_piece: FieldPiece) -> bool {
        field_piece
            .cells()
            .iter()
            .all(|&(x, y)| !self.occupied(x, y))
    }

    fn legal_moves(&self, movement_state: MovementState) -> Vec<MovementState> {
        if movement_state.is_locked {
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
                            DEFAULT_MOVEMENT_TIME.move_one,
                        ));
                    }
                }

                PieceMovement::MoveRight => {
                    let new_field_piece = movement_state.field_piece.move_by(1, 0);
                    if self.attempt(new_field_piece) {
                        result.push(movement_state.next_movement_state(
                            new_field_piece,
                            piece_movement,
                            DEFAULT_MOVEMENT_TIME.move_one,
                        ));
                    }
                }

                // MEMO: not currently supported for soft drop of only one cell
                PieceMovement::SoftDrop => {
                    let mut new_field_piece = movement_state.field_piece;
                    let mut count = 0;
                    while self.attempt(new_field_piece.move_by(0, -1)) {
                        new_field_piece = new_field_piece.move_by(0, -1);
                        count += 1;
                    }
                    if count > 0 {
                        result.push(movement_state.next_movement_state_with_movements(
                            new_field_piece,
                            vec![piece_movement; count as usize],
                            DEFAULT_MOVEMENT_TIME.move_one * count,
                        ));
                    }
                }

                PieceMovement::HardDrop => {
                    let mut new_field_piece = movement_state.field_piece;
                    while self.attempt(new_field_piece.move_by(0, -1)) {
                        new_field_piece = new_field_piece.move_by(0, -1);
                    }
                    let mut new_movement_state = movement_state.next_movement_state(
                        new_field_piece,
                        piece_movement,
                        DEFAULT_MOVEMENT_TIME.hard_drop,
                    );
                    new_movement_state.is_locked = true;
                    result.push(new_movement_state);
                }

                // TODO: SRS
                PieceMovement::RotateLeft => {
                    let new_field_piece = movement_state.field_piece.rotate_left();
                    if self.attempt(new_field_piece) {
                        result.push(movement_state.next_movement_state(
                            new_field_piece,
                            piece_movement,
                            DEFAULT_MOVEMENT_TIME.rotate,
                        ))
                    }
                }

                // TODO: SRS
                PieceMovement::RotateRight => {
                    let new_field_piece = movement_state.field_piece.rotate_right();
                    if self.attempt(new_field_piece) {
                        result.push(movement_state.next_movement_state(
                            new_field_piece,
                            piece_movement,
                            DEFAULT_MOVEMENT_TIME.rotate,
                        ))
                    }
                }

                // MEMO: holds are not searched here because next must also be considered
                _ => {}
            }
        }

        result
    }
}
