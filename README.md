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

* 045 - Jump Game II (2025/03/15) [problem link](https://leetcode.com/problems/jump-game-ii/) [solution](note/medium/045_jump_game_2.md)

* 046 - Permutations (2025/03/29) [problem link](https://leetcode.com/problems/permutations/description/) [solution](note/medium/046_permutations.md)

* 047 - Permutations II (2025/04/03) [problem link](https://leetcode.com/problems/permutations-ii/description/) [solution](note/medium/047_permutations_II.md)

* 048 - Rotate Image (2025/04/03) [problem link](https://leetcode.com/problems/rotate-image/description/) [solution](note/medium/048_rotate_image.md)

* 049 - Group Anagrams (2025/04/12) [problem link](https://leetcode.com/problems/group-anagrams/description/) [solution](note/medium/049_group_anagrams.md)

* 050 - Pow(x, n) (2025/04/19)  [problem link](https://leetcode.com/problems/powx-n/description/) [solution](note/medium/050_pow_x_n.md)

* 053 - Maximum Subarray (2025/05/11) 👈 [problem link](https://leetcode.com/problems/maximum-subarray/description/) [solution](note/medium/maximum_subarray.md)
<br>

### Hard

* 044 - Wildcard Matching (2025/02/20) [problem link](https://leetcode.com/problems/wildcard-matching/description/) [solution](note/hard/044_wildcard_matching.md)

* 051 - N-Queens (2025/04/26) [problem link](https://leetcode.com/problems/n-queens/description/) [solution](note/hard/051_n_queens.md)

* 052 - N-Queens II (2025/05/03) [problem link](https://leetcode.com/problems/n-queens-ii/description/) [solution](note/hard/052_n_queens_ii.md)

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