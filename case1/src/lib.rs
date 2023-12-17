cargo_component_bindings::generate!();

use bindings::{
    component::case1::random::{Algorithm, Seed},
    Guest,
};

struct Component;

impl Guest for Component {
    fn rand(seed: Seed) -> u32 {
        match seed.algorithm {
            Algorithm::Goblin => seed.value + 1,
            Algorithm::Orge => seed.value + 2,
        }
    }
}
