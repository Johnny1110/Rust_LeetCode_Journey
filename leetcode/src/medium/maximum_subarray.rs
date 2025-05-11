use crate::common::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // Define a DP array to store the maximum sum of subarrays ending at each index
        let mut dp = vec![0; nums.len()];
        let mut max_num_result = nums[0];

        // Init dp[0] with the first element of nums
        dp[0] = nums[0];

        // Iterate through the nums array starting from index 1
        for i in 1..nums.len() {
            dp[i] = Self::max(nums[i], dp[i - 1] + nums[i]);
            max_num_result = Self::max(max_num_result, dp[i]);
        }

        max_num_result
    }

    fn max(a: i32, b: i32) -> i32 {
        if a > b {
            a
        } else {
            b
        }
    }
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
        assert_eq!(Solution::max_sub_array(vec![-2, -3, -1]), -1);
    }

    #[test]
    fn test_max_sub_array_single_element() {
        assert_eq!(Solution::max_sub_array(vec![42]), 42);
    }

    #[test]
    fn test_max_sub_array_all_negative() {
        assert_eq!(Solution::max_sub_array(vec![-1, -2, -3]), -1);
    }
}