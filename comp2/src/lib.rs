cargo_component_bindings::generate!();

// Generated binding method for `rand` function that is defined in comp1
use bindings::comp1::rand;
// Generated types that `rand` function depends on
use bindings::newgyu::comp1::types::{Algorithm, Seed};
// Generated for `hello-world` function that is defined in comp2/world.wit
use bindings::Hello;
struct Component;

impl Hello for Component {
    /// Say hello!
    fn hello_world() -> String {
        let seed = Seed {
            algorithm: Algorithm::Goblin,
            value: 9,
        };
        let n = rand(seed);
        format!("Hello, {}", n)
    }
}
