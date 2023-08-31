cargo_component_bindings::generate!();
use bindings::comp2::hello_world;

fn main() {
    println!("greet {}", hello_world());
}
