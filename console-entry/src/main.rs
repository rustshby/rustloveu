//! main entry
//! 

extern crate algorithms;
extern crate leetcode;
extern crate cratesdepends;
extern crate rustbasic;

use cratesdepends::redis_help;

/// main runs the result
fn main() {
    rustbasic::run_hello_world();
    leetcode::two_sum_works();
    algorithms::heap_sort_works();
    algorithms::quick_sort_works();
    algorithms::stooge_sort_works();
    algorithms::comb_sort_works();
    println!("{:?}", redis_help::fetch_an_integer())
}
