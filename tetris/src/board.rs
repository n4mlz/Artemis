use crate::*;
use rand::{seq::IteratorRandom, thread_rng, Rng};
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

pub fn row_x(x: i32) -> u16 {
    1 << x
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Board {
    pub cells: [u16; 40],
    pub collumn_heights: [u32; 10],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [0; 40],
            collumn_heights: [0; 10],
        }
    }

    pub fn occupied(&self, x: i32, y: i32) -> bool {
        !(0..10).contains(&x) || !(0..40).contains(&y) || (self.cells[y as usize] & row_x(x) > 0)
    }

    pub fn is_empty(&self) -> bool {
        self.cells[0] == 0
    }

    fn calc_collumn_heights(&self) -> [u32; 10] {
        let mut collumn_heights = [0; 10];
        for x in 0..10 {
            for y in (0..40).rev() {
                if self.occupied(x, y) {
                    collumn_heights[x as usize] = y as u32 + 1;
                    break;
                }
            }
        }
        collumn_heights
    }

    pub fn attempt(&self, field_piece: FieldPiece) -> bool {
        field_piece
            .cells()
            .iter()
            .all(|&(x, y)| !self.occupied(x, y))
    }

    pub fn legal_moves(&self, movement_state: MovementState) -> Vec<MovementState> {
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
                    while self.attempt(new_field_piece.move_by(0, -1)) {
                        new_field_piece = new_field_piece.move_by(0, -1);
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
                    while self.attempt(new_field_piece.move_by(0, -1)) {
                        new_field_piece = new_field_piece.move_by(0, -1);
                    }
                    let mut new_movement_state = movement_state.next_movement_state(
                        new_field_piece,
                        piece_movement,
                        DEFAULT_ACTION_TIME.hard_drop,
                    );
                    new_movement_state.field_piece.is_locked = true;
                    result.push(new_movement_state);
                }

                // check if the SRS works correctly
                PieceMovement::RotateLeft => {
                    // check if the following kick table is correct
                    const KICK_TABLE_NORMAL: [[(i32, i32); 5]; 4] = [
                        [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                        [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                        [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                        [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                    ];

                    const KICK_TABLE_I: [[(i32, i32); 5]; 4] = [
                        [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                        [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                        [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                        [(0, 0), (1, 0), (-2, 0), (-2, -1), (1, 2)],
                    ];

                    let new_field_piece = movement_state.field_piece.rotate_left();

                    match movement_state.field_piece.piece_state.piece {
                        Piece::O => {
                            if self.attempt(new_field_piece) {
                                result.push(movement_state.next_movement_state(
                                    new_field_piece,
                                    piece_movement,
                                    DEFAULT_ACTION_TIME.rotate,
                                ));
                            }
                        }
                        Piece::I => {
                            for (srs_state, kick) in KICK_TABLE_I.iter().enumerate() {
                                let srs_field_piece = new_field_piece.move_by(kick[0].0, kick[0].1);
                                if self.attempt(srs_field_piece) {
                                    result.push(movement_state.next_movement_state(
                                        srs_field_piece.set_super_rotation_state(srs_state as u32),
                                        piece_movement,
                                        DEFAULT_ACTION_TIME.rotate,
                                    ));
                                    break;
                                }
                            }
                        }
                        _ => {
                            for (srs_state, kick) in KICK_TABLE_NORMAL.iter().enumerate() {
                                let srs_field_piece = new_field_piece.move_by(kick[0].0, kick[0].1);
                                if self.attempt(srs_field_piece) {
                                    result.push(movement_state.next_movement_state(
                                        srs_field_piece.set_super_rotation_state(srs_state as u32),
                                        piece_movement,
                                        DEFAULT_ACTION_TIME.rotate,
                                    ));
                                    break;
                                }
                            }
                        }
                    }
                }

                // check if the SRS works correctly
                PieceMovement::RotateRight => {
                    // check if the following kick table is correct
                    const KICK_TABLE_NORMAL: [[(i32, i32); 5]; 4] = [
                        [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                        [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                        [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                        [(0, 0), (-2, 0), (-2, -1), (0, 2), (-1, 2)],
                    ];

                    const KICK_TABLE_I: [[(i32, i32); 5]; 4] = [
                        [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                        [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                        [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                        [(0, 0), (-2, 0), (1, 0), (1, -2), (-2, 1)],
                    ];

                    let new_field_piece = movement_state.field_piece.rotate_right();

                    match movement_state.field_piece.piece_state.piece {
                        Piece::O => {
                            if self.attempt(new_field_piece) {
                                result.push(movement_state.next_movement_state(
                                    new_field_piece,
                                    piece_movement,
                                    DEFAULT_ACTION_TIME.rotate,
                                ));
                            }
                        }
                        Piece::I => {
                            for (srs_state, kick) in KICK_TABLE_I.iter().enumerate() {
                                let srs_field_piece = new_field_piece.move_by(kick[0].0, kick[0].1);
                                if self.attempt(srs_field_piece) {
                                    result.push(movement_state.next_movement_state(
                                        srs_field_piece.set_super_rotation_state(srs_state as u32),
                                        piece_movement,
                                        DEFAULT_ACTION_TIME.rotate,
                                    ));
                                    break;
                                }
                            }
                        }
                        _ => {
                            for (srs_state, kick) in KICK_TABLE_NORMAL.iter().enumerate() {
                                let srs_field_piece = new_field_piece.move_by(kick[0].0, kick[0].1);
                                if self.attempt(srs_field_piece) {
                                    result.push(movement_state.next_movement_state(
                                        srs_field_piece.set_super_rotation_state(srs_state as u32),
                                        piece_movement,
                                        DEFAULT_ACTION_TIME.rotate,
                                    ));
                                    break;
                                }
                            }
                        }
                    }
                }

                // MEMO: holds are not searched here because next must also be considered
                _ => {}
            }
        }

        result
    }

    pub fn place_piece(&self, movement_state: &MovementState) -> (Board, PlacementKind) {
        use PlacementKind::*;

        let mut new_board = *self; // copy
        let field_piece = movement_state.field_piece;
        for &(x, y) in field_piece.cells().iter() {
            new_board.cells[y as usize] |= row_x(x);
        }

        let mut cleared_rows = 0;
        // reverse order to avoid shifting
        for y in 0..40 {
            if new_board.cells[y] == 0x3ff {
                cleared_rows += 1;
            } else if cleared_rows > 0 {
                new_board.cells[y - cleared_rows as usize] = new_board.cells[y];
            }
        }
        for y in 0..cleared_rows {
            new_board.cells[39 - y as usize] = 0;
        }

        new_board.collumn_heights = new_board.calc_collumn_heights();

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

        // t-spin detection
        // TODO: check if the following code is correct
        let movements_history = &movement_state.movements_history;
        if (movements_history.len() > 1)
            && matches!(
                movements_history[movements_history.len() - 2], // get the second to last one since the last one is HardDrop
                PieceMovement::RotateLeft | PieceMovement::RotateRight
            )
        {
            let t_piece_corners = [(-1, 1), (1, 1), (1, -1), (-1, -1)];
            let t_piece_corners = (0..4)
                .map(|i| {
                    self.occupied(
                        field_piece.position.0
                            + t_piece_corners[(i + field_piece.piece_state.rotation as usize) / 4]
                                .0,
                        field_piece.position.1
                            + t_piece_corners[(i + field_piece.piece_state.rotation as usize) / 4]
                                .1,
                    )
                })
                .collect::<Vec<_>>();

            // t-spin
            if t_piece_corners.iter().filter(|&&x| x).count() >= 3 {
                // t-spin mini
                if (!t_piece_corners[0] || !t_piece_corners[1])
                    && field_piece.super_rotation_state != Some(3)
                {
                    let placement_kind = match cleared_rows {
                        0 => MiniTspin,
                        1 => MiniTspin1,
                        2 => MiniTspin2,
                        _ => MiniTspin,
                    };

                    return (new_board, placement_kind);
                }

                // t-spin normal
                let placement_kind = match cleared_rows {
                    0 => Tspin,
                    1 => Tspin1,
                    2 => Tspin2,
                    3 => Tspin3,
                    _ => Tspin,
                };

                return (new_board, placement_kind);
            }
        }

        // normal
        let placement_kind = match cleared_rows {
            0 => None,
            1 => Clear1,
            2 => Clear2,
            3 => Clear3,
            _ => None,
        };

        (new_board, placement_kind)
    }

    pub fn receive_garbage(&self, garbage: u32) -> Board {
        let mut rng = thread_rng();
        let mut new_board = *self; // copy
        let mut hole_positions = vec![rng.gen_range(0..10)];

        for _ in 1..garbage as usize {
            if rng.gen_range(0.0..1.0) < SAME_HOLE_POSITION_RATE {
                hole_positions.push(*hole_positions.last().unwrap());
            } else {
                hole_positions.push(
                    (0..10)
                        .filter(|&x| x != *hole_positions.last().unwrap())
                        .choose(&mut rng)
                        .unwrap(),
                );
            }
        }

        for y in (0..40).rev() {
            if y >= garbage as usize {
                new_board.cells[y] = new_board.cells[y - garbage as usize];
            } else {
                new_board.cells[y] = 0x3ff & !row_x(hole_positions.pop().unwrap());
            }
        }

        new_board.collumn_heights = new_board.collumn_heights.map(|h| h + garbage);

        new_board
    }
}
