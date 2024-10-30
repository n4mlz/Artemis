use itertools::Itertools;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tetris::{row_x, Board, State};

pub type Score = i32;
pub type Reward = Score;
pub type Value = Score;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Evaluator {
    // TODO: add some parameters
}

impl Evaluator {
    pub fn evaluate(&self, state: &State) -> (Reward, Value) {
        // height of the highest column
        let height = *state.board.collumn_heights.iter().max().unwrap();
        (0, height as Value * -100)
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

            if x > 1 && board.collumn_heights[x - 1] < y && board.collumn_heights[x - 2] - 1 < y {
                overhangs += 1;
                continue;
            }

            if x < 8 && board.collumn_heights[x + 1] < y && board.collumn_heights[x + 2] - 1 < y {
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

fn deepest_well_collumn(well_depths: &[i32; 10]) -> i32 {
    let mut col = 0;
    for i in 1..10 {
        if well_depths[i] > well_depths[col] {
            col = i;
        }
    }
    col as i32
}

fn deepest_well_clearable_lines(board: &Board, well_col: i32) -> i32 {
    let mut clearable_lines = 0;
    for y in board.collumn_heights[well_col as usize]..20 {
        if (board.cells[y as usize] | row_x(well_col)) != 0x3ff {
            break;
        }
        clearable_lines += 1;
    }
    clearable_lines
}

fn hight(board: &Board) -> i32 {
    *board.collumn_heights.iter().max().unwrap() as i32
}
