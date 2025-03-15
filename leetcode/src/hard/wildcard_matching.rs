use crate::common::Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // Convert Strings to vectors of chars for efficient access.
        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();

        // define 2 pointer for s and p
        let (mut p_ptr, mut s_ptr) = (0, 0);

        // star_ptr will store the index in p where '*' was last seen.
        // match_ptr stores the position in s corresponding to the last match after '*'.
        let (mut start_ptr, mut match_ptr) = (None, 0);

        // Loop through the string s.
        while s_ptr < s_chars.len() {
            let p_is_not_over = p_ptr < p_chars.len();

            // Direct match or '?' wildcard.
            if p_is_not_over && (p_chars[p_ptr] == '?' || p_chars[p_ptr] == s_chars[s_ptr]) {
                p_ptr += 1;
                s_ptr += 1;
            }
            // Record position of '*' in pattern and current position in s
            else if p_is_not_over && p_chars[p_ptr] == '*' {
                start_ptr = Some(p_ptr);
                match_ptr = s_ptr;
                p_ptr += 1;
                // why we not increment s_ptr here? because we want to match '*' with 0 or more characters.
            }
            // Mismatch occurred, but there was a previous '*'
            // Reset p_ptr to one past the '*' and advance match_ptr.
            else if let Some(star) = start_ptr {
                p_ptr = star + 1;
                match_ptr += 1;
                s_ptr = match_ptr;
            } 
            // No '*' to fallback to and characters do not match.
            else {
                return false;
            }
        }

        // Check for remaining characters in pattern.
        while p_ptr < p_chars.len() && p_chars[p_ptr] == '*' {
            p_ptr += 1;
        }

        p_ptr == p_chars.len()
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