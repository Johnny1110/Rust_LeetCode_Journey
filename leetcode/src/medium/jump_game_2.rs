use std::arch::aarch64::int32x2_t;

use crate::common::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {

        // init 1D array as DP, eveny element is i32::MAX
        let mut dp = vec![i32::MAX; nums.len()];

        // init the first element as 0
        dp[0] = 0;

        // iterate the nums array
        nums.iter()
            .enumerate()
            .for_each(|(position, steps)| {
                for i in position+1..position+(steps+1) as usize {
                    if i >= nums.len() {
                        break;
                    }
                    dp[i] = Self::min(dp[i], dp[position] + 1);
                }
            });
        
        dp[nums.len()-1]
    }

    fn min(a: i32, b: i32) -> i32 {
        if a < b {
            return a;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
    }
}

