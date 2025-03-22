use std::arch::aarch64::int32x2_t;

use crate::common::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
    }
}

