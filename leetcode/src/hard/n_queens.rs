use crate::common::Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve_n_queens(4), vec![])
    }
}