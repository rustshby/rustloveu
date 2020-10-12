//! stooge sort
//!
//!

pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    if arr.first().unwrap() > arr.last().unwrap() {
        arr.swap(0, len - 1);
    }
    if len - 1 > 1 {
        let t = len / 3;
        stooge_sort(&mut arr[..len - 1]);
        stooge_sort(&mut arr[t..]);
        stooge_sort(&mut arr[..len - 1]);
    }
}

#[allow(dead_code)]
pub fn stooge_sort_works() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before : {:?}\n", numbers);
    stooge_sort(&mut numbers);
    println!("After : {:?}\n", numbers);

    println!("Sort strings alphabetically");
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    stooge_sort(&mut strings);
    println!("After:  {:?}\n", strings);
}
