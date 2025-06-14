use crate::common::Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        let input = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let output = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        let result = Solution::merge(input);
        assert_eq!(result, output);
    }

    #[test]
    fn test_merge_intervals_empty() {
        let input: Vec<Vec<i32>> = vec![];
        let output: Vec<Vec<i32>> = vec![];
        let result = Solution::merge(input);
        assert_eq!(result, output);
    }
}