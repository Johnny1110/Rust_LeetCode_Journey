# 53. Maximum Subarray

<br>

link: https://leetcode.com/problems/maximum-subarray/description/

<br>

---

## Topic

* Array
* Divide and Conquer
* Dynamic Programming

<br>

## Brain Strom

<br>

The Topic say __DP__ and __Divide and Conquer__.

Let's put Divide and Conquer behind, I think it's another approach, maybe leave it to next week challange.

Now, let's focus on DP.

What is DP? Exchange space for time. define a array, maybe 2D. and put partial result into it.

<br>

We have a question here, how do we define DP? let's try it with some paper work.

<br>

![1](imgs/053_1.jpg)

<br>

I give this hand writting DP solution to Chat-GPT and I got some advice:

<br>
<br>

### Chat-GPT Advice

All you really need is a 1D DP:

Let `dp[i]` = maximum subarray ending at index i.

Recurrence:

```
dp[0] = nums[0]
dp[i] = max(dp[i-1] + nums[i], nums[i])
Then the answer is max(dp[i]) over all i.
```

<br>
<br>

## Let's coding

```rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // Define a DP array to store the maximum sum of subarrays ending at each index
        let mut dp = vec![0; nums.len()];
        let mut max_num_result = nums[0];

        // Init dp[0] with the first element of nums
        dp[0] = nums[0];

        // Iterate through the nums array starting from index 1
        for i in 1..nums.len() {
            dp[i] = Self::max(nums[i], dp[i - 1] + nums[i]);
            max_num_result = Self::max(max_num_result, dp[i]);
        }

        max_num_result
    }

    fn max(a: i32, b: i32) -> i32 {
        if a > b {
            a
        } else {
            b
        }
    }
}
```

<br>

![2](imgs/053_2.jpg)

<br>
<br>

Hell yeah! let's do it with Divide and Conquer approach next week.

<br>

---

<br>
<br>

## Another Approach - Divide and Conquer

<br>

### Brain Strom