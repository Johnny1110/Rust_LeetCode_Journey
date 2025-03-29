use crate::common::Solution;


impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        return vec![vec![]];
    }
}

// -----------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::permute(vec![1,2,3]), vec![vec![1,2,3], vec![1,3,2], vec![2,1,3], vec![2,3,1], vec![3,1,2], vec![3,2,1]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::permute(vec![0,1]), vec![vec![0,1], vec![1,0]]);
    }

}