//! comb sort
//!
//!
//!

pub fn comb_sort<T: Ord>(arr: &mut [T]) {
    let mut gap = arr.len() as f64; // Initialize gap size
    let shrink = 1.3; // Set the gap shrink factor
    let mut sorted = false;

    while sorted == false {
        // Update the gap value for a next comb
        gap = gap as f64 / shrink as f64;
        gap = gap.floor();

        if gap <= 1.0 {
            gap = 1.0;
            sorted = true;
        }

        let mut i = 0.0;
        while i + gap < arr.len() as f64 {
            if arr[i as usize] > arr[i as usize + gap as usize] {
                arr.swap(i as usize, i as usize + gap as usize);
                sorted = false;
            }
            i += 1.0;
        }
    }
}