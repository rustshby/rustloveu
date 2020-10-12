//! leetcode using rust
//!
//!
//!

pub mod twonumber;

pub fn two_sum_works() {
    println!("------stooge_sort_works------");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = twonumber::two_sum(nums, target);

    // use & to loop vec, this will not lose ownership
    for i in &result {
        println!("{:?}", i);
    }
}


