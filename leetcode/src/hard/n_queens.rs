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
    fn test_solve_n_queens() {
        let n = 4;
        let result = Solution::solve_n_queens(n);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![".Q..", "...Q", "Q...", "..Q."]);
        assert_eq!(result[1], vec!["..Q.", "Q...", "...Q", ".Q.."]);
    }

    #[test]
    fn test_solve_n_queens_1() {
        let n = 1;
        let result = Solution::solve_n_queens(n);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec!["Q"]);
    }
}