use crate::common::Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_n_queens() {
        assert_eq!(Solution::total_n_queens(1), 1);
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(5), 10);
        assert_eq!(Solution::total_n_queens(8), 92);
    }
}