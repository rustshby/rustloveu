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


