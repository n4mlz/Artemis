use bot::Evaluator;
use rand::{thread_rng, Rng};

pub trait Gene {
    fn generate() -> Self;
    fn crossover(parent1: &Self, parent2: &Self) -> Self;
}

impl Gene for i32 {
    fn generate() -> Self {
        thread_rng().gen_range(-10..=10)
    }

    fn crossover(v1: &Self, v2: &Self) -> i32 {
        let mut rng = thread_rng();

        (match rng.gen_range(0..100) {
            0..=41 => *v1,                  // 42%
            42..=83 => *v2,                 // 42%
            84..=98 => (v1 + v2) / 2,       // 15%
            _ => rng.gen_range(-150..=150), // 1%
        } + rng.gen_range(-10..=10))
    }
}

impl Gene for Evaluator {
    fn generate() -> Self {
        Evaluator {
            bumpiness: i32::generate(),
            bumpiness_sq: i32::generate(),
            cavities: i32::generate(),
            cavities_sq: i32::generate(),
            overhangs: i32::generate(),
            overhangs_sq: i32::generate(),
            covers: i32::generate(),
            covers_sq: i32::generate(),
            well_depth_1: i32::generate(),
            well_depth_1_sq: i32::generate(),
            well_depth_2: i32::generate(),
            well_depth_2_sq: i32::generate(),
            hight: i32::generate(),
            hight_sq: i32::generate(),
            b2b: i32::generate(),
            holding: [
                i32::generate(),
                i32::generate(),
                i32::generate(),
                i32::generate(),
                i32::generate(),
            ],

            move_time: i32::generate(),
            wasted_i: i32::generate(),
            b2b_clear: i32::generate(),
            perfect_clear: i32::generate(),
            combo_garbage: i32::generate(),
            clear1: i32::generate(),
            clear2: i32::generate(),
            clear3: i32::generate(),
            clear4: i32::generate(),
        }
    }

    fn crossover(parent1: &Self, parent2: &Self) -> Self {
        Evaluator {
            bumpiness: i32::crossover(&parent1.bumpiness, &parent2.bumpiness),
            bumpiness_sq: i32::crossover(&parent1.bumpiness_sq, &parent2.bumpiness_sq),
            cavities: i32::crossover(&parent1.cavities, &parent2.cavities),
            cavities_sq: i32::crossover(&parent1.cavities_sq, &parent2.cavities_sq),
            overhangs: i32::crossover(&parent1.overhangs, &parent2.overhangs),
            overhangs_sq: i32::crossover(&parent1.overhangs_sq, &parent2.overhangs_sq),
            covers: i32::crossover(&parent1.covers, &parent2.covers),
            covers_sq: i32::crossover(&parent1.covers_sq, &parent2.covers_sq),
            well_depth_1: i32::crossover(&parent1.well_depth_1, &parent2.well_depth_1),
            well_depth_1_sq: i32::crossover(&parent1.well_depth_1_sq, &parent2.well_depth_1_sq),
            well_depth_2: i32::crossover(&parent1.well_depth_2, &parent2.well_depth_2),
            well_depth_2_sq: i32::crossover(&parent1.well_depth_2_sq, &parent2.well_depth_2_sq),
            hight: i32::crossover(&parent1.hight, &parent2.hight),
            hight_sq: i32::crossover(&parent1.hight_sq, &parent2.hight_sq),
            b2b: i32::crossover(&parent1.b2b, &parent2.b2b),
            holding: [
                i32::crossover(&parent1.holding[0], &parent2.holding[0]),
                i32::crossover(&parent1.holding[1], &parent2.holding[1]),
                i32::crossover(&parent1.holding[2], &parent2.holding[2]),
                i32::crossover(&parent1.holding[3], &parent2.holding[3]),
                i32::crossover(&parent1.holding[4], &parent2.holding[4]),
            ],

            move_time: i32::crossover(&parent1.move_time, &parent2.move_time),
            wasted_i: i32::crossover(&parent1.wasted_i, &parent2.wasted_i),
            b2b_clear: i32::crossover(&parent1.b2b_clear, &parent2.b2b_clear),
            perfect_clear: i32::crossover(&parent1.perfect_clear, &parent2.perfect_clear),
            combo_garbage: i32::crossover(&parent1.combo_garbage, &parent2.combo_garbage),
            clear1: i32::crossover(&parent1.clear1, &parent2.clear1),
            clear2: i32::crossover(&parent1.clear2, &parent2.clear2),
            clear3: i32::crossover(&parent1.clear3, &parent2.clear3),
            clear4: i32::crossover(&parent1.clear4, &parent2.clear4),
        }
    }
}
