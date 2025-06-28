
use crate::common::Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_intervals() {
        let input = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let output = vec![vec![1, 5], vec![6, 9]];
        let result = Solution::insert(input, new_interval);
        assert_eq!(result, output);
    }

    #[test]
    fn test_insert_intervals_no_overlap() {
        let input = vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]];
        let new_interval = vec![4, 8];
        let output = vec![vec![1, 2], vec![3, 5], vec![4, 8], vec![9, 10], vec![12, 16]];
        let result = Solution::insert(input, new_interval);
        assert_eq!(result, output);
    }   

    #[test]
    fn test_insert_intervals_empty() {
        let input: Vec<Vec<i32>> = vec![];
        let new_interval = vec![5, 7];
        let output = vec![vec![5, 7]];
        let result = Solution::insert(input, new_interval);
        assert_eq!(result, output);
    }

}