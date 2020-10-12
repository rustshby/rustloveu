//! # Problem
//! Given an array of integers, return indices of the two numbers such that they add up to a specific target.
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//!
//! # Example
//! Given nums = [2, 7, 11, 15], target = 9,
//! Because nums[0] + nums[1] = 2 + 7 = 9,
//! return [0, 1].
//!

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut mm: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        match mm.get(&(target - v)) {
            Some(&index) => {
                return vec![index, i as i32];
            }
            None => {
                mm.entry(*v).or_insert(i as i32);
            }
        }
    }
    vec![]
}