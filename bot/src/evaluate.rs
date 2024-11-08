use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tetris::{Board, State};

pub type Score = i32;
pub type Reward = Score;
pub type Value = Score;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
pub struct Evaluator {
    // value
    pub bumpiness: i32,
    pub bumpiness_sq: i32,
    pub cavities: i32,
    pub cavities_sq: i32,
    pub overhangs: i32,
    pub overhangs_sq: i32,
    pub covers: i32,
    pub covers_sq: i32,
    pub well_depth_1: i32,
    pub well_depth_1_sq: i32,
    pub well_depth_2: i32,
    pub well_depth_2_sq: i32,
    pub hight: i32,
    pub hight_sq: i32,
    pub b2b: i32,
    pub holding: [i32; 5], // (S, Z), (J, L), T, O, I  // TODO: put in order

    // reward
    pub move_time: i32,
    pub wasted_i: i32,
    pub b2b_clear: i32,
    pub combo_garbage: i32,
    pub clear1: i32,
    pub clear2: i32,
    pub clear3: i32,
    pub clear4: i32,
}

impl Evaluator {
    pub fn evaluate(&self, state: &State) -> (Reward, Value) {
        let (mut reward, mut value) = (0, 0);

        let bumpiness = bumpiness(&state.board);
        value += bumpiness * self.bumpiness;
        value += bumpiness * bumpiness * self.bumpiness_sq;

        let (cavities, overhangs) = cavities_and_overhangs(&state.board);
        value += cavities * self.cavities;
        value += cavities * cavities * self.cavities_sq;
        value += overhangs * self.overhangs;
        value += overhangs * overhangs * self.overhangs_sq;

        let covers = covers(&state.board);
        value += covers * self.covers;
        value += covers * covers * self.covers_sq;

        let well_depths = well_depths(&state.board);

        let (depth_1, depth_2) = two_deepest_well_depths(&well_depths);
        value += depth_1 * self.well_depth_1;
        value += depth_1 * depth_1 * self.well_depth_1_sq;
        value += depth_2 * self.well_depth_2;
        value += depth_2 * depth_2 * self.well_depth_2_sq;

        let hight = hight(&state.board);
        value += hight * self.hight;
        value += hight * hight * self.hight_sq;

        value += state.b2b as i32 * self.b2b;

        if let Some(hold_piece) = state.hold_piece {
            match hold_piece {
                tetris::Piece::S | tetris::Piece::Z => value += self.holding[0],
                tetris::Piece::J | tetris::Piece::L => value += self.holding[1],
                tetris::Piece::T => value += self.holding[2],
                tetris::Piece::O => value += self.holding[3],
                tetris::Piece::I => value += self.holding[4],
            }
        }

        if let Some(last_action) = &state.last_action {
            reward += last_action.time as i32 * self.move_time;

            if last_action.placed_piece == tetris::Piece::I
                && last_action.placement_kind == tetris::PlacementKind::None
            {
                reward += self.wasted_i;
            }

            if last_action.b2b {
                reward += self.b2b_clear;
            }

            reward += last_action.combo as i32 * self.combo_garbage;

            match last_action.placement_kind {
                tetris::PlacementKind::Clear1 => reward += self.clear1,
                tetris::PlacementKind::Clear2 => reward += self.clear2,
                tetris::PlacementKind::Clear3 => reward += self.clear3,
                tetris::PlacementKind::Clear4 => reward += self.clear4,
                _ => {}
            }
        }

        (reward, value)
    }
}

fn bumpiness(board: &Board) -> i32 {
    let mut bumpiness = 0;
    for x in 0..9 {
        bumpiness += (board.collumn_heights[x] as i32 - board.collumn_heights[x + 1] as i32).abs();
    }
    bumpiness
}

fn cavities_and_overhangs(board: &Board) -> (i32, i32) {
    let mut cavities = 0;
    let mut overhangs = 0;

    for x in 0..10 {
        for y in 0..board.collumn_heights[x] {
            if board.occupied(x as i32, y as i32) {
                continue;
            }

            if x > 1 && board.collumn_heights[x - 1] < y && board.collumn_heights[x - 2] < y + 1 {
                overhangs += 1;
                continue;
            }

            if x < 8 && board.collumn_heights[x + 1] < y && board.collumn_heights[x + 2] < y + 1 {
                overhangs += 1;
                continue;
            }

            cavities += 1;
        }
    }

    (cavities, overhangs)
}

fn covers(board: &Board) -> i32 {
    let mut covers = 0;
    for x in 0..10 {
        let mut exist_empty = false;
        for y in 0..board.collumn_heights[x] {
            if !board.occupied(x as i32, y as i32) {
                exist_empty = true;
            } else if exist_empty {
                covers += 1;
            }
        }
    }
    covers
}

fn well_depths(board: &Board) -> [i32; 10] {
    let mut well_depths = [0; 10];
    for x in 0..10 {
        let left = if x == 0 {
            40
        } else {
            board.collumn_heights[x - 1]
        };
        let right = if x == 9 {
            40
        } else {
            board.collumn_heights[x + 1]
        };
        well_depths[x] = (left.min(right) as i32 - board.collumn_heights[x] as i32).max(0);
    }
    well_depths
}

fn two_deepest_well_depths(well_depths: &[i32; 10]) -> (i32, i32) {
    well_depths
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .copied()
        .collect_tuple()
        .unwrap()
}

fn hight(board: &Board) -> i32 {
    *board.collumn_heights.iter().max().unwrap() as i32
}

// TODO: remove
pub fn debug_evaluation(state: &State) {
    let bumpiness = bumpiness(&state.board);
    println!("bumpiness: {}", bumpiness);

    let (cavities, overhangs) = cavities_and_overhangs(&state.board);
    println!("cavities: {}, overhangs: {}", cavities, overhangs);

    let covers = covers(&state.board);
    println!("covers: {}", covers);

    let well_depths = well_depths(&state.board);
    println!("well_depths: {:?}", well_depths);

    let (depth_1, depth_2) = two_deepest_well_depths(&well_depths);
    println!("depth_1: {}, depth_2: {}", depth_1, depth_2);

    let hight = hight(&state.board);
    println!("hight: {}", hight);
}
