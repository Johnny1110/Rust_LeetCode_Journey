use crate::common::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        1
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
    fn test_max_sub_array_empty() {
        assert_eq!(Solution::max_sub_array(vec![]), 0);
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