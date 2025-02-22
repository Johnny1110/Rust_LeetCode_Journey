use crate::common::Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        let m = s.len();
        let n = p.len();
        let mut dp = vec![vec![false; n + 1]; m + 1];

        // 1. init dp:

        // 1-1. dp[0][0] = true
        dp[0][0] = true;

        // 1-2. dp[i][0] = False (non-empty string can't match empty pattern). additional: skip i = 0.
        for i in 1..=m {
            dp[i][0] = false;
        }

        // 1-3. dp[0][j] depends on whether p[0:j] consists only of '*'.
        for j in 1..=n {
            let previous_dp = dp[0][j - 1];
            let current_char_is_star = p.chars().nth(j-1).unwrap() == '*';
            dp[0][j] = previous_dp && current_char_is_star;
        }
        
        // 2. fill the dp table
        Self::fill_dp_table(&mut dp, &s, &p);

        // 3. return the result
        dp[m][n]
    }

    fn fill_dp_table(dp: &mut Vec<Vec<bool>>, s: &String, p: &String) {
        // If p[j-1] == s[i-1] or p[j-1] == '?', inherit dp[i-1][j-1].
        // If p[j-1] == '*', we have two choices:
            // dp[i][j-1]: Treat '*' as an empty sequence.
            // dp[i-1][j]: Treat '*' as matching s[i-1] (extend the match).

        let m = s.len();
        let n = p.len();

        for i in 1..=m {
            for j in 1..=n {
                let current_s = s.chars().nth(i-1).unwrap();
                let current_p = p.chars().nth(j-1).unwrap();

                if current_s == current_p || current_p == '?' {
                    // s 跟 pattern 各往前推一步，如果滿足，則繼承（前面都對了，那新的 s, p 也都一樣或者 p 是 '?' 那也必定符合）
                    dp[i][j] = dp[i-1][j-1]; 
                } else if current_p == '*' {
                    dp[i][j] = dp[i][j-1] || dp[i-1][j];
                }
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let s = "abc".to_string();
        let p = "a?c".to_string();
        // Replace false with the correct expected output when implemented.
        assert_eq!(Solution::is_match(s, p), true);
    }

    #[test]
    fn test_edge_case() {
        let s = "".to_string();
        let p = "".to_string();
        assert_eq!(Solution::is_match(s, p), true);
    }

    #[test]
    fn test_1() {
        let s = "aa".to_string();
        let p = "a".to_string();
        assert_eq!(Solution::is_match(s, p), false);
    }

    #[test]
    fn test_2() {
        let s = "aa".to_string();
        let p = "*".to_string();
        assert_eq!(Solution::is_match(s, p), true);
    }

    #[test]
    fn test_3() {
        let s = "cb".to_string();
        let p = "?a".to_string();
        assert_eq!(Solution::is_match(s, p), false);
    }

    #[test]
    fn test_4() {
        let s = "aaabbb".to_string();
        let p = "aaa*b".to_string();
        assert_eq!(Solution::is_match(s, p), true);
    }
}