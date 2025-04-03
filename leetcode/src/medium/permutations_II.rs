use core::num;

use crate::common::Solution;

impl Solution {

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // create a result vector.
        let mut results: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        let mut used: Vec<bool> = vec![false; nums.len()];

        // sort nums in first place
        nums.sort();
        
        // call the backtracking function.
        Self::_backtracking(&mut results, &mut nums, 0, &mut used);
        results
    }

    fn _backtracking(results: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, idx: usize, used: &mut Vec<bool>) {

        // if idx is equals to nums.len() then we have a complete permutation
        if idx == nums.len() {
            results.push(nums.clone());
            return;
        }

        // do swapping
        for i in idx..nums.len() {
            
            // decide if we need to skip the current number.
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            nums.swap(idx, i);
            Self::_backtracking(results, nums, idx+1, used);
            nums.swap(idx, i);
        }
    }
}

// -----------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::permute_unique(vec![1,2,1]), vec![vec![1,1,2], vec![1,2,1], vec![2,1,1]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::permute_unique(vec![1,2,3]), vec![vec![1,2,3], vec![1,3,2], vec![2,1,3], vec![2,3,1], vec![3,2,1], vec![3,1,2]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::permute_unique(vec![0,1]), vec![vec![0,1], vec![1,0]]);
    }

}