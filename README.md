# Rust LeetCode Journey

<br>

---

<br>

Weekly leetcode training with Rust, start from 2025/02/20.


<br>

## Category

<br>

### Easy

<br>

### Medium

* 045 - Jump Game II 👈 (2025/03/15) [problem link](https://leetcode.com/problems/jump-game-ii/) [solution](note/hard/045_jump_game_2.md)

<br>

### Hard

* 044 - Wildcard Matching (2025/02/20) [problem link](https://leetcode.com/problems/wildcard-matching/description/) [solution](note/hard/044_wildcard_matching.md)

<br>
<br>

## Bonus

<br>

---

<br>

## About Test:

Rust cargo unit test example:

<br>

wildcard_matching.rs:

```rust

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        ...
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
}
```

<br>

cargo cli:
```
cargo test hard::wildcard_matching
```