cargo_component_bindings::generate!();

use bindings::newgyu::comp1::types::Algorithm;
use bindings::{RandomGenerator, Seed};

struct Component;

impl RandomGenerator for Component {
    fn rand(seed: Seed) -> u32 {
        match seed.algorithm {
            Algorithm::Goblin => seed.value + 100,
            Algorithm::Orge => seed.value + 1000,
        }
    }
}
