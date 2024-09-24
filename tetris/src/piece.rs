// TODO: put in order
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
