use core::num;

use crate::common::Solution;

impl Solution {

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // create a result vector.
        let mut results: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        let mut used: Vec<bool> = vec![false; nums.len()];
        let mut current: Vec<i32> = vec![];

        // sort nums in first place
        nums.sort();
        
        // call the backtracking function.
        Self::_backtracking(&mut results, &nums, &mut current, &mut used);

        results
    }

    fn _backtracking(results: &mut Vec<Vec<i32>>, nums: &Vec<i32>, current: &mut Vec<i32>, visited: &mut Vec<bool>) {
        if current.len() == nums.len() {
            results.push(current.clone());
            return;
        }

        for i in 0..nums.len() {
            // 1. skip the visited numbers.
            if visited[i] {
                continue;
            }

            // !!! IMPORTANT !!!
            // Skip duplicates: if the current number is the same as the previous one
            // and the previous one is not used in this permutation branch (already put into result in previous step).
            if i > 0 && nums[i] == nums[i-1] && !visited[i-1] {
                continue;
            }
            
            // 2. put the current number into the current vector.
            current.push(nums[i]);
            visited[i] = true;
            
            // 3. go to the next level.
            Self::_backtracking(results, nums, current, visited);

            // 4. rollback the current number.
            current.pop();
            visited[i] = false;
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
        assert_eq!(Solution::permute_unique(vec![1,2,3]), vec![vec![1,2,3], vec![1,3,2], vec![2,1,3], vec![2,3,1], vec![3,1,2], vec![3,2,1]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::permute_unique(vec![0,1]), vec![vec![0,1], vec![1,0]]);
    }

}