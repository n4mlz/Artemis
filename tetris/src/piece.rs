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
struct PieceState {
    piece: Piece,
    rotation: RotationState,
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
    pub fn cells(&self) -> [(i32, i32); 4] {
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

        const CELLS: &[[(i32, i32); 4]] = &generate_cells!(
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
