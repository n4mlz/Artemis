use crate::*;

type Position = (i32, i32);

// TODO: put in order
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Piece {
    S,
    Z,
    J,
    L,
    I,
    O,
    T,
}

impl Piece {
    pub fn initial_position(&self) -> Position {
        // NOTE: this is configured for tetrio
        // TODO: confirm this is correct
        match self {
            Piece::S => (4, 19),
            Piece::Z => (4, 19),
            Piece::J => (4, 19),
            Piece::L => (4, 19),
            Piece::I => (4, 19),
            Piece::O => (4, 18),
            Piece::T => (4, 19),
        }
    }
}

// TODO: put in order
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RotationState {
    North,
    East,
    South,
    West,
}

impl RotationState {
    pub fn rotate_left(&self) -> RotationState {
        use RotationState::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn rotate_right(&self) -> RotationState {
        use RotationState::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PieceState {
    pub piece: Piece,
    pub rotation: RotationState,
}

impl PieceState {
    pub fn rotate_left(&self) -> PieceState {
        PieceState {
            piece: self.piece,
            rotation: self.rotation.rotate_left(),
        }
    }

    pub fn rotate_right(&self) -> PieceState {
        PieceState {
            piece: self.piece,
            rotation: self.rotation.rotate_right(),
        }
    }

    // TODO: fix clippy warning
    pub fn cells(&self) -> [Position; 4] {
        macro_rules! generate_cells {
            ($([$(($x:expr, $y:expr)),*]),*) => {
                [$(
                    [$(($x, $y)),*],   // North
                    [$(($y, -$x)),*],  // East
                    [$((-$x, -$y)),*], // South
                    [$((-$y, $x)),*]   // West
                ),*]
            };
        }

        const CELLS: &[[Position; 4]] = &generate_cells!(
            [(-1, 0), (0, 0), (0, 1), (1, 1)],  // S
            [(-1, 1), (0, 1), (0, 0), (1, 0)],  // Z
            [(-1, 0), (0, 0), (1, 0), (-1, 1)], // J
            [(-1, 0), (0, 0), (1, 0), (1, 1)],  // L
            [(-1, 0), (0, 0), (1, 0), (2, 0)],  // I
            [(0, 0), (1, 0), (0, 1), (1, 1)],   // O
            [(-1, 0), (0, 0), (1, 0), (0, 1)]   // T
        );

        let index = self.piece as usize * 4 + self.rotation as usize;
        CELLS[index]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SuperRotationState {
    None,
    Mini, // T-spin only
    Normal,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FieldPiece {
    pub piece_state: PieceState,
    pub position: Position,
    pub super_rotation_state: SuperRotationState,
    pub is_locked: bool,
}

impl FieldPiece {
    pub fn new_from_piece(piece: Piece) -> FieldPiece {
        FieldPiece {
            piece_state: PieceState {
                piece,
                rotation: RotationState::North,
            },
            position: piece.initial_position(),
            super_rotation_state: SuperRotationState::None,
            is_locked: false,
        }
    }

    pub fn cells(&self) -> [Position; 4] {
        let cells = self.piece_state.cells();
        cells.map(|(x, y)| (x + self.position.0, y + self.position.1))
    }

    pub fn move_by(&self, dx: i32, dy: i32) -> FieldPiece {
        FieldPiece {
            position: (self.position.0 + dx, self.position.1 + dy),
            ..*self
        }
    }

    pub fn rotate_left(&self) -> FieldPiece {
        FieldPiece {
            piece_state: self.piece_state.rotate_left(),
            ..*self
        }
    }

    pub fn rotate_right(&self) -> FieldPiece {
        FieldPiece {
            piece_state: self.piece_state.rotate_right(),
            ..*self
        }
    }
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

pub struct MovementTime {
    pub move_one: Time,
    pub hard_drop: Time,
    pub rotate: Time,
    pub hold: Time,
}

// TODO: make the value correct
pub const DEFAULT_MOVEMENT_TIME: MovementTime = MovementTime {
    move_one: 100,
    hard_drop: 100,
    rotate: 100,
    hold: 100,
};
