use core::num;

use crate::common::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        
        // Define the DP array, dp[i] = true if we can reach the end from index i
        let mut dp = vec![false; nums.len()];
        // init first DP value as true.
        dp[0] = true;
        let mut max_reach = nums[0] as usize;

        for i in 0..nums.len() {
            if i > max_reach {
                return false;
            }

                dp[i] = true;
                max_reach = max_reach.max(i + nums[i] as usize);

                if max_reach >= nums.len() - 1 {
                    return true;
                }
        }
        

        dp[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump_1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test_can_jump_2() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn test_can_jump_3() {
        assert!(Solution::can_jump(vec![0]));
    }

    #[test]
    fn test_can_jump_4() {
        assert!(Solution::can_jump(vec![1, 2, 3]));
    }

    #[test]
    fn test_can_jump_5() {
        assert!(Solution::can_jump(vec![2, 0, 0]));
    }
}