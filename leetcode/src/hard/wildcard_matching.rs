use crate::common::Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // 1. Initialize Pointers: Start with both pointers at the beginning of s and p, 
        // and initialize variables to record the last '*' encountered and the match position.
        let mut s_ptr = 0;
        let mut p_ptr = 0;
        let mut match_ptr = 0;
        let mut star_ptr = -1;

        // 2. iterate through the string s
        while s_ptr < s.len() {
            let the_char = s..chars().nth(s_ptr).upwrap();
            let the_pattern = p.chars().nth(p_str).upwrap();
            // If characters match (or you have a '?'), move both pointers forward.
            // When encountering '*', record the positions and advance the pattern pointer.
            if the_char == the_pattern || the_pattern == '?' {
                s_ptr += 1;
                p_ptr += 1;
            } else if the_pattern == '*' {
                star_ptr = p_ptr;
                match_ptr = s_ptr;
                s_ptr += 1;
                p_ptr += 1;
            } else if star_ptr != -1 {
                // TODO
            }
        }
        

        return false;
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