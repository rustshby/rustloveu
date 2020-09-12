//! main entry

mod hello_world;
use hello_world::ferris as hello_world_ferris;

/// main runs the result
fn main() {
    hello_world::basic_show_helloworld();
    hello_world_ferris::ferris_show("Hello, world! Rust Loves You!");
}