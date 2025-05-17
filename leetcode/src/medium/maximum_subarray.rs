use crate::common::Solution;

impl Solution {
    
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        Self::divide_and_conquer(&nums, 0, nums.len() - 1)
    }

    pub fn divide_and_conquer(nums : &Vec<i32>, low: usize, high: usize) -> i32 {
        if low == high {
            return nums[low];
        }
        
        let mid =low + (high - low) / 2;

        // calculate the maximum subarray sum in the left half
        let left_max = Self::divide_and_conquer(nums, low, mid);
        let right_max = Self::divide_and_conquer(nums, mid+1, high);

        // calculate the maximum subarray sum that crosses the midpoint
        let mut left_sum_max = i32::MIN;
        let mut sum = 0;
        for i in (low..=mid).rev() {
            sum += nums[i];
            left_sum_max = left_sum_max.max(sum);
        }

        let mut right_sum_max = i32::MIN;
        sum = 0;
        for i in mid+1..=high {
            sum += nums[i];
            right_sum_max = right_sum_max.max(sum);
        }

        let crossing_max = left_sum_max + right_sum_max;

        // return the maximum of the left, right, and crossing sums
        *[left_max, right_max, crossing_max].iter().max().unwrap()
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

    #[test]
    fn test_max_sub_array_standard() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}