mod hello_world;

pub fn run_hello_world() {
    hello_world::basic_show_helloworld();
    hello_world::formatted_print::formatted_show();
    hello_world::ferris::ferris_show("Hello, world! Rust Loves You!");
    hello_world::debug::debug_show();
    hello_world::display::display_show();
}
