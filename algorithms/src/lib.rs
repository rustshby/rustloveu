//! algorithms in rust
//!
//!
//!

pub mod sort;
use sort::{heapsort, quicksort, stoogesort};

pub fn heap_sort_works() {
    println!("------heap_sort_works------");
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    heapsort::heap_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);

    println!("Sort strings alphabetically");
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    heapsort::heap_sort(&mut strings);
    println!("After:  {:?}\n", strings);
}

pub fn quick_sort_works() {
    println!("------quick_sort_works------");
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    quicksort::quick_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);

    println!("Sort strings alphabetically");
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    quicksort::quick_sort(&mut strings);
    println!("After:  {:?}\n", strings);
}

pub fn stooge_sort_works() {
    println!("------stooge_sort_works------");
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before : {:?}\n", numbers);
    stoogesort::stooge_sort(&mut numbers);
    println!("After : {:?}\n", numbers);

    println!("Sort strings alphabetically");
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    stoogesort::stooge_sort(&mut strings);
    println!("After:  {:?}\n", strings);
}

