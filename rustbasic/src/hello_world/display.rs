//! hello_world::display module
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
//!
//! fmt::Debug hardly looks compact and clean,
//! so it is often advantageous to customize the output appearance.
//!
//! This is done by manually implementing fmt::Display,
//! which uses the {} print marker.

use std::fmt;

// Define a structure for which `fmt::Display` will be implemented.
// This is a tuple struct named `DisplayPrintable` that contains an `i32`.
#[allow(dead_code)]
struct DisplayPrintable(i32);

// To use the `{}` marker, the trait `fmt::Display` must be  implemented
// manually for the type.
impl fmt::Display for DisplayPrintable {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether
        // the operation successed or failed.
        // Note that `write!` use syntax which is very similar to `println!`
        write!(f, "{}", self.0)
        //Ok(write!(f, "{}", self.0)?)
    }
}

// A structure holding 2 numbers. `Debug` will be derived so the results
// can be contrasted with `Display`.
#[allow(dead_code)]
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

// Define a structure where the fields are namealbe for comparison.
#[allow(dead_code)]
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Define a structure named `List` containing a `Vec`.
#[allow(dead_code)]
#[derive(Debug)]
struct List(Vec<i32>);

// Implementing fmt::Display for a structure
// where the elements must each be handled sequentially is tricky.
// The problem is that each write! generates a fmt::Result.
// Proper handling of this requires dealing with all the results.
// Rust provides the ? operator for exactly this purpose.
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

/// display_show shows some difference for fmt::Debug and fmt::Display differences.
///
#[allow(dead_code)]
pub fn display_show() {
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Debug Beauti: {:#?}", point);

    let v = List(vec![1, 2, 3]);
    println!("Compare List:");
    println!("Display: {}", v);
    println!("Debug: {:?}", v);
}
