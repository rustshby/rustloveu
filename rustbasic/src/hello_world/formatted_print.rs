//! hello_world::formated_print module shows the basic printing by a seris of macros
//! defined in std::fmt
//!
//! format! - write formatted text to String
//!
//! print! - same as format! but the text is printed to the console (io::stdout).
//!
//! println! - same as print! but a newline is appended.
//!
//! eprint! - same as format! but the text is printed to the standard error (io::stderr).
//!
//! eprintln! - same as eprint! but a new line is append.alloc
//! 
//! # std::fmt 
//! contains many traits which govern the display of text.
//! The base form of two import ones are:
//! 
//! fmt::Debug - Uses the {:?} marker. Format text for debugging purposes.
//!     
//! fmt::Display - Uses the {} marker. Format text in a more elegant, use friendly fashion.


/// formatted_show shows some formated features
///
#[allow(dead_code)]
pub fn formatted_show() {
    // In general, the `{}` will be automatically replaced with
    // any arguments. These will be stringified.
    println!("{} days", 31);

    // without a suffix, 31 becomes an i32.
    // You can change what type 31 is by providing a suffix.
    // The number 31i64 example has the type i64.

    // There are various optional patterns this works with.
    // Positional arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can naed arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // You can right-align text with a specified width.
    // This will output "     1". 5 white spaces add a "1".
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>0width$}", number = 1, width = 6);
}