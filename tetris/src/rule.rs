use crate::*;

pub struct ActionTime {
    pub move_one: Time,
    pub hard_drop: Time,
    pub rotate: Time,
    pub hold: Time,
    pub place: Time,
    pub single: Time,
    pub double: Time,
    pub triple: Time,
    pub tetris: Time,
    pub perfect_clear: Time,
}

// TODO: make the value correct
pub const DEFAULT_ACTION_TIME: ActionTime = ActionTime {
    move_one: 100,
    hard_drop: 100,
    rotate: 100,
    hold: 100,
    place: 100,
    single: 100,
    double: 100,
    triple: 100,
    tetris: 100,
    perfect_clear: 100,
};

pub fn base_attack(placement_kind: PlacementKind) -> u32 {
    use PlacementKind::*;
    match placement_kind {
        None => 0,
        Clear1 => 0,
        Clear2 => 1,
        Clear3 => 2,
        Clear4 => 4,
        MiniTspin => 0,
        MiniTspin1 => 0,
        MiniTspin2 => 1,
        Tspin => 0,
        Tspin1 => 2,
        Tspin2 => 4,
        Tspin3 => 6,
    }
}

pub fn combo_attack(combo: u32) -> u32 {
    match combo {
        0 => 0,
        1 => 0,
        2 => 1,
        3 => 1,
        4 => 2,
        5 => 2,
        6 => 3,
        7 => 3,
        8 => 4,
        9 => 4,
        10 => 4,
        _ => 5,
    }
}

pub struct SpecialAttack {
    pub b2b: u32,
    pub perfect_clear: u32,
}

pub const DEFAULT_SPECIAL_ATTACK: SpecialAttack = SpecialAttack {
    b2b: 1,
    perfect_clear: 10,
};
