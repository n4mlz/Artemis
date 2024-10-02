use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct MovementState {
    field_piece: FieldPiece,
    hold_piece: Option<Piece>,
    movements_history: Vec<PieceMovement>,
    has_held: bool,
    is_locked: bool,
    time: u32,
}

impl MovementState {
    fn new_from_piece(current_piece: Piece, hold_piece: Option<Piece>) -> MovementState {
        MovementState {
            field_piece: FieldPiece::new_from_piece(current_piece),
            hold_piece,
            movements_history: vec![],
            has_held: false,
            is_locked: false,
            time: 0,
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
                        let mut new_movement_state = movement_state.clone();
                        new_movement_state.field_piece = new_field_piece;
                        new_movement_state.movements_history.push(piece_movement);
                        new_movement_state.time += DEFAULT_MOVEMENT_TIME.move_one;
                        result.push(new_movement_state);
                    }
                }

                PieceMovement::MoveRight => {
                    let new_field_piece = movement_state.field_piece.move_by(1, 0);
                    if self.attempt(new_field_piece) {
                        let mut new_movement_state = movement_state.clone();
                        new_movement_state.field_piece = new_field_piece;
                        new_movement_state.movements_history.push(piece_movement);
                        new_movement_state.time += DEFAULT_MOVEMENT_TIME.move_one;
                        result.push(new_movement_state);
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
                        let mut new_movement_state = movement_state.clone();
                        new_movement_state.field_piece = new_field_piece;
                        new_movement_state
                            .movements_history
                            .append(&mut vec![piece_movement; count as usize]);
                        new_movement_state.time += DEFAULT_MOVEMENT_TIME.move_one * count;
                        result.push(new_movement_state);
                    }
                }

                PieceMovement::HardDrop => {
                    let mut new_field_piece = movement_state.field_piece;
                    while self.attempt(new_field_piece.move_by(0, -1)) {
                        new_field_piece = new_field_piece.move_by(0, -1);
                    }
                    let mut new_movement_state = movement_state.clone();
                    new_movement_state.field_piece = new_field_piece;
                    new_movement_state.movements_history.push(piece_movement);
                    new_movement_state.is_locked = true;
                    new_movement_state.time += DEFAULT_MOVEMENT_TIME.hard_drop;
                    result.push(new_movement_state);
                }

                // TODO: SRS
                PieceMovement::RotateLeft => {
                    let new_field_piece = movement_state.field_piece.rotate_left();
                    if self.attempt(new_field_piece) {
                        let mut new_movement_state = movement_state.clone();
                        new_movement_state.field_piece = new_field_piece;
                        new_movement_state.movements_history.push(piece_movement);
                        new_movement_state.time += DEFAULT_MOVEMENT_TIME.rotate;
                        result.push(new_movement_state);
                    }
                }

                // TODO: SRS
                PieceMovement::RotateRight => {
                    let new_field_piece = movement_state.field_piece.rotate_right();
                    if self.attempt(new_field_piece) {
                        let mut new_movement_state = movement_state.clone();
                        new_movement_state.field_piece = new_field_piece;
                        new_movement_state.movements_history.push(piece_movement);
                        new_movement_state.time += DEFAULT_MOVEMENT_TIME.rotate;
                        result.push(new_movement_state);
                    }
                }

                // MEMO: holds are not searched here because next must also be considered
                _ => {}
            }
        }

        result
    }
}
