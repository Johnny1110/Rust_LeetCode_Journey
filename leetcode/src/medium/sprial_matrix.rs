use crate::common::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_spiral_order_empty() {
        let matrix: Vec<Vec<i32>> = vec![];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_spiral_order_single_row() {
        let matrix = vec![vec![1, 2, 3]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3]);
    }
}