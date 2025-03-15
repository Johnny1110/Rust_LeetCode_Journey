# 45. Jump Game II

<br>

---

<br>

https://leetcode.com/problems/jump-game-ii/

<br>

## Topic 

* Array
* Dynamic Programming
* Greedy

<br>

## Brain Strom

<br>

### DP Approach

<br>

First of all, I want to try with DP approach.

So how should I define the data in DP? Let's try using a 1D array. every dp position `i` means the min step from `nums[0]` to `nums[i]`.

Consider the array: 

```
nums = [2, 3, 1, 1, 4]
```

Initialization:

* dp[0] = 0
* dp[1..4] = ∞ (or a large number)

<br>

From the index 0:

* `nums[0]` is 2, so I can jump to index 1 & index 2:


```
// from 0~1
dp[1] = min(∞, dp[0]+1)

// from 0~2
dp[2] = min(∞, dp[0]+1)
```

<br>

From the index 1:

* `nums[1]` is 3, so I can jump the index 2, 3, 4:

```
// from 1~2
dp[2] = min(dp[2], dp[1] + 1) // -> min(1, 2) = 1
// from 1~3
dp[3] = min(dp[3], dp[1] + 1) // -> min(∞, 2) = 2
// from 1~4
dp[4] = min(dp[4], dp[1] + 1) // -> min(∞, 2) = 2
```

From the index 2:

...

<br>

Implement:

```rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        2
    }
}
```

