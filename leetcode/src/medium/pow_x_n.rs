use crate::common::Solution;

impl Solution {

    pub fn my_pow(x: f64, n: i32) -> f64 {
        return 1.0;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_my_pow_1() {
        let x = 2.0;
        let n = 10;
        let result = Solution::my_pow(x, n);
        assert_eq!(result, 1024.0);
    }

    #[test]
    fn test_my_pow_2() {
        let x = 2.1;
        let n = 3;
        let result = Solution::my_pow(x, n);
        assert_eq!(result, 9.26100);
    }

    #[test]
    fn test_my_pow_3() {
        let x = 2.0;
        let n = -2;
        let result = Solution::my_pow(x, n);
        assert_eq!(result, 0.25);
    }

}