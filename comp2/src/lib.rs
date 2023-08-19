cargo_component_bindings::generate!();

use bindings::Example;

struct Component;

impl Example for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}
