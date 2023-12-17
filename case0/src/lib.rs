cargo_component_bindings::generate!();
use bindings::exports::component::case0::random::{Algorithm, Guest, Seed};

struct Component;

impl Guest for Component {
    fn rand(seed: Seed) -> u32 {
        match seed.algorithm {
            Algorithm::Goblin => seed.value + 1,
            Algorithm::Orge => seed.value + 2,
        }
    }
}
