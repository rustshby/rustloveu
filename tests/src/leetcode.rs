#[cfg(test)]
mod tests {
    extern crate leetcode;

    #[test]
    fn it_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = leetcode::twonumber::two_sum(nums, target);
        assert_eq!(result, [0, 1]);
    }
}
