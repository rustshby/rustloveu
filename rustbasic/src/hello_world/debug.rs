//! hello_world::debug module
//!
//! All types which want to use std::fmt formatting traits reuqire
//! an implementation to be printable.
//!
//! Automatic implementations are only provied for types such as in the std library.
//! All others must be manually implemented somehow.
//!
//! The fmt::Debug trait makes this very straightforward.
//! All types can derive(automatically create) the fmt::Debug implementation.
//! This is not true for fmt::Display which must be manually implemented.

// This structure cannot be printed either with `fmt::Display` or with `fmt::Debug`.
#[allow(dead_code)]
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` with `fmt::Debug`.
#[allow(dead_code)]
#[derive(Debug)]
struct DebugPrintable(i32);

/// debug_show shows some fetures for fmt::Debug and fmt::Display
///
#[allow(dead_code)]
pub fn debug_show() {
    // All std library types automatically are printable with `{:?}`.
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor is"
    );

    // Put a `DebugPrintable` inside of the structure `Deep`.
    // Make it printable also.
    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("Now {:?} will print!", DebugPrintable(520));
    println!("Now {:?} will print!", Deep(DebugPrintable(520)));

    // fmt::Debug definitely makes this printable but sacrifices some elegance.
    // Rust also provides "pretty printing" with {:#?}.
    println!("Now {:#?} will print!", DebugPrintable(520));
    println!("Now {:#?} will print!", Deep(DebugPrintable(520)));
}
