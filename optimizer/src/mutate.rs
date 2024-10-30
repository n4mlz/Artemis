use bot::Evaluator;
use rand::{thread_rng, Rng};

pub trait Gene {
    fn generate() -> Self;
    fn crossover(parent1: &Self, parent2: &Self) -> Self;
}

impl Gene for i32 {
    fn generate() -> Self {
        thread_rng().gen_range(-999..=999)
    }

    fn crossover(v1: &Self, v2: &Self) -> i32 {
        let mut rng = thread_rng();
        let v = match rng.gen_range(0..100) {
            0..=41 => *v1,                  // 42%
            42..=83 => *v2,                 // 42%
            84..=98 => (v1 + v2) / 2,       // 15%
            _ => rng.gen_range(-999..=999), // 1%
        } + rng.gen_range(-10..=10);

        v.clamp(-999, 999)
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
            well_scale: [i32::generate(); 5],
            well_depth_1: i32::generate(),
            well_depth_1_sq: i32::generate(),
            well_depth_2: i32::generate(),
            well_depth_2_sq: i32::generate(),
            well_clearable_lines: i32::generate(),
            well_clearable_lines_sq: i32::generate(),
            hight: i32::generate(),
            hight_sq: i32::generate(),
            b2b: i32::generate(),
            holding: [i32::generate(); 5],
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
            well_scale: [
                i32::crossover(&parent1.well_scale[0], &parent2.well_scale[0]),
                i32::crossover(&parent1.well_scale[1], &parent2.well_scale[1]),
                i32::crossover(&parent1.well_scale[2], &parent2.well_scale[2]),
                i32::crossover(&parent1.well_scale[3], &parent2.well_scale[3]),
                i32::crossover(&parent1.well_scale[4], &parent2.well_scale[4]),
            ],
            well_depth_1: i32::crossover(&parent1.well_depth_1, &parent2.well_depth_1),
            well_depth_1_sq: i32::crossover(&parent1.well_depth_1_sq, &parent2.well_depth_1_sq),
            well_depth_2: i32::crossover(&parent1.well_depth_2, &parent2.well_depth_2),
            well_depth_2_sq: i32::crossover(&parent1.well_depth_2_sq, &parent2.well_depth_2_sq),
            well_clearable_lines: i32::crossover(
                &parent1.well_clearable_lines,
                &parent2.well_clearable_lines,
            ),
            well_clearable_lines_sq: i32::crossover(
                &parent1.well_clearable_lines_sq,
                &parent2.well_clearable_lines_sq,
            ),
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
        }
    }
}
