# 044 - Wildcard Matching

<br>

---

<br>

[Back](../..)

## Desc

Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*' where:

'?' Matches any single character.
'*' Matches any sequence of characters (including the empty sequence).
The matching should cover the entire input string (not partial).

<br>

Example 1:

```
Input: s = "aa", p = "a"
Output: false
```

Explanation: "a" does not match the entire string "aa".


Example 2:

```
Input: s = "aa", p = "*"
Output: true
```


Explanation: '*' matches any sequence.


Example 3:

```
Input: s = "cb", p = "?a"
Output: false
```

Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.

<br>

Constraints:

```
0 <= s.length, p.length <= 2000
s contains only lowercase English letters.
p contains only lowercase English letters, '?' or '*'.
```

<br>

## Topic

* String
* Dynamic Programming
* Greedy [Video](https://www.bilibili.com/video/BV18V411b74c/?spm_id_from=333.337.search-card.all.click&vd_source=9780a181ac9f1fee5f680f255ee5bc73), [Article](https://medium.com/@ralph-tech/%E6%BC%94%E7%AE%97%E6%B3%95%E5%AD%B8%E7%BF%92%E7%AD%86%E8%A8%98-%E8%B2%AA%E5%A9%AA%E6%BC%94%E7%AE%97%E6%B3%95-greedy-algorithm-e2666b93d05f)
* Recursion

<br>

## Thinking

| Approach  | Pros  | Cons  |
|---|---|---|
| Recursion  | Simple, intuitive  | 	Slow (exponential) without memoization |
| Dynamic Programming  | Efficient, optimal	  | Uses extra space O(nm)|
| Greedy  | Fastest, O(n + m)	  |  Harder to implement correctly|

I gonna try with DP first, then I will go with Greedy.

<br>

### Dynamic Programming

* Define `dp[i][j]` as whether `s[0:i]` matches `p[0:j]`.

* Transition:
    * If `p[j-1] == s[i-1]` or `p[j-1] == '?'`, inherit `dp[i-1][j-1]`.
    * If `p[j-1] == '*'`, we have two choices:
        * `dp[i][j-1]`: Treat '*' as an empty sequence.
        * `dp[i-1][j]`: Treat '*' as matching `s[i-1]` (extend the match).

* Base cases:

    * `dp[0][0] = True` (empty string matches empty pattern).
    * `dp[i][0] = False` (non-empty string can't match empty pattern).
    * `dp[0][j]` depends on whether `p[0:j]` consists only of '*'.


<br>

### Greedy (Optimized for '*')

If '*' appears, it can match any number of characters, so:

1. Remember the last position where '*' was used.
2. If we reach a mismatch, backtrack to the last '*' and try matching more characters.

This approach avoids unnecessary recursion and speeds up execution.

How to Implement:

* Use two pointers: i for s and j for p.
* Track starIndex (last seen '*') and matchIndex (where matching started).
* If `p[j] == '*'`, store starIndex and move j forward.
* If mismatch occurs and we saw a '*', backtrack and retry from `matchIndex + 1`.

<br>

## Solved:

### version-1: DP

```rust
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
```

<br>

### version-2: Revamp by Greedy

<br>

1. Understand the role of `*`:

    * Key Insight:

        The wildcard `*` can match any sequence include empty. instead of expolring all possibilities like DP, greedy tries to match as many chars as possible and the do "backtracks" if a later mismatch fource you to reconsider how many chars '*' should absorb.

    * Question to Ponder:

          How can you quickly decide how any chars to let '*' match without try every possibility like DP.

<br>

2. Use 2 Pointer

    * Concept:

        Using 2 pointers one is for input String `s` and another is for input pattern `p`. move these 2 pointer forward when char matched or when you encounter pattern `?`.

    * Design Question:

        What should I do when current char not match and pattern is not wildcard ?

<br>

3. Record the postition of the `'*'`

    * Stratgey:

        When you see the star in pattern, store the pattetn position of `*` and also input s position where this `*` was encountered.
        This "bookmark" will allow you to backtrack later if needed.

    * Thinking point:

        How can you use this stored position to "extend" the match `*` if you hit the dead end later ?

4. Handling mismatches with backtracking.

    * Process: 

        If you encounter a mismatch (where the current char not match the pattern or is not `*`  and `?`), check if you have previous `*`

        __YES__: Instead if giving up, use the record `*` position: 

        * Assume that `*` should match one more char from the string.

        * Reset the pattern pointer to to just after the `*`, and move the string pointer forward.

        __NO__: you can just conclude the pattern does not match the string.

<br>

### Reflective question

How do you update your pointers so that you "try again" with `*` can matching a longer sequence.

<br>

<br>

## Overall Flow:

1. Initialize Pointers: Start with both pointers at the beginning of s and p, and initialize variables to record the last '*' encountered and the match position.

2. Iterate Over the String:
    * If characters match (or you have a `?`), move both pointers forward.
    * When encountering `*`, record the positions and advance the pattern pointer.

3. Backtrack if Needed:

    * On a mismatch, if a previous `*` was recorded, adjust the pointers to "extend" the match for the `*`.

4. Handle Trailing Characters in Pattern:
    * Ensure any remaining characters in the pattern are `*` so they can match an empty sequence.


```rust
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
```

[rs_file](../../leetcode/src/hard/wildcard_matching.rs)