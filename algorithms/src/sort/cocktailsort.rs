//! cocktail sort
//!
//!
//!

pub fn cocktail_sort<T: Ord>(arr: &mut [T]) {
    let last = arr.len() - 1;
    loop {
        let mut swapped = false;
        for i in 0..last {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if swapped == false {
            return;
        }

        swapped = false;
        for i in (0..last - 1).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if swapped == false {
            return;
        }
    }
}
