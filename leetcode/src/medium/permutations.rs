use crate::common::Solution;


impl Solution {

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {

        // create a resut vector.
        let mut results: Vec<Vec<i32>> = vec![];
        let mut nums = nums;

        // call the backtracking function.
        Self::backtracking(&mut results, &mut nums, 0);

        return results;
    }

    fn backtracking(results: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, idx: usize) {
        

        // if the idx is equal to the length of the nums, copy the vector to the results and return.
        if idx == nums.len() {
            results.push(nums.clone());
            return;
        }

        // do swapping.
        for i in idx..nums.len() {
            nums.swap(idx, i);
            Self::backtracking(results, nums, idx + 1);
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
        assert_eq!(Solution::permute(vec![1,2,3]), vec![vec![1,2,3], vec![1,3,2], vec![2,1,3], vec![2,3,1], vec![3,2,1], vec![3,1,2]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::permute(vec![0,1]), vec![vec![0,1], vec![1,0]]);
    }

}