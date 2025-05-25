use crate::common::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
        assert!(Solution::can_jump(vec![0]));
        assert!(Solution::can_jump(vec![1, 2, 3]));
        assert!(!Solution::can_jump(vec![2, 0, 0]));
    }
}