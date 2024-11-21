use strum::EnumIter;

type Position = (i32, i32);

// TODO: put in order
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
pub enum Piece {
    S,
    Z,
    J,
    L,
    T,
    O,
    I,
}

impl Piece {
    pub fn initial_position(&self) -> Position {
        // NOTE: this is configured for tetrio
        // TODO: confirm this is correct
        match self {
            Piece::S => (4, 20),
            Piece::Z => (4, 20),
            Piece::J => (4, 20),
            Piece::L => (4, 20),
            Piece::T => (4, 20),
            Piece::O => (4, 21),
            Piece::I => (4, 20),
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
            (@normal [$(($x:expr, $y:expr)),*]) => {
                [
                    [$(($x, $y)),*],         // North
                    [$(($y, -$x)),*],        // East
                    [$((-$x, -$y)),*],       // South
                    [$((-$y, $x)),*]         // West
                ]
            };
            (@i_piece [$(($x:expr, $y:expr)),*]) => {
                [
                    [$(($x, $y)),*],              // North
                    [$(($y + 1, -$x)),*],         // East
                    [$((-$x + 1, -$y - 1)),*],    // South
                    [$((-$y, $x - 1)),*]          // West
                ]
            };
            () => {
                [
                    generate_cells!(@normal [(-1, 0), (0, 0), (0, 1), (1, 1)]),     // S
                    generate_cells!(@normal [(-1, 1), (0, 1), (0, 0), (1, 0)]),     // Z
                    generate_cells!(@normal [(-1, 0), (0, 0), (1, 0), (-1, 1)]),    // J
                    generate_cells!(@normal [(-1, 0), (0, 0), (1, 0), (1, 1)]),     // L
                    generate_cells!(@normal [(-1, 0), (0, 0), (1, 0), (0, 1)]),     // T
                    generate_cells!(@normal [(0, 0), (1, 0), (0, -1), (1, -1)]),    // O
                    generate_cells!(@i_piece [(-1, 0), (0, 0), (1, 0), (2, 0)])     // I
                ]
            };
        }

        const CELLS: &[[[Position; 4]; 4]; 7] = &generate_cells!();

        CELLS[self.piece as usize][self.rotation as usize]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FieldPiece {
    pub piece_state: PieceState,
    pub position: Position,
    pub super_rotation_state: Option<u32>, // Some: 0, 1, 2, 3
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
            super_rotation_state: None,
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
            super_rotation_state: None,
            ..*self
        }
    }

    pub fn rotate_left(&self) -> FieldPiece {
        FieldPiece {
            piece_state: self.piece_state.rotate_left(),
            super_rotation_state: None,
            ..*self
        }
    }

    pub fn rotate_right(&self) -> FieldPiece {
        FieldPiece {
            piece_state: self.piece_state.rotate_right(),
            super_rotation_state: None,
            ..*self
        }
    }

    pub fn set_super_rotation_state(&self, state: u32) -> FieldPiece {
        FieldPiece {
            super_rotation_state: Some(state),
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
