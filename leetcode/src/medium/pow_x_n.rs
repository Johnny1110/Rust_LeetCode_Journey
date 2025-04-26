use crate::common::Solution;

impl Solution {

    pub fn my_pow(x: f64, n: i32) -> f64 {
        // 1. convert n to i64 also make x to mut
        let mut N = n as i64;
        let mut x = x;

        // 2. if n is negative make x to reciprocal and N = abs(N)
        if n < 0 {
            x = 1.0/x;
            N = -N;
        }

        let mut result = 1.0;
        let mut currentPower = x;
        // 3. do fast powering calculate
        while N > 0 {

            if (N & 1) == 1 { // when bin(N) right side is 1
                result *= currentPower;
            }

            currentPower *= currentPower;
            N = N >> 1
        }

        result
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
        assert_eq!(result, 9.261000000000001);
    }

    #[test]
    fn test_my_pow_3() {
        let x = 2.0;
        let n = -2;
        let result = Solution::my_pow(x, n);
        assert_eq!(result, 0.25);
    }

}