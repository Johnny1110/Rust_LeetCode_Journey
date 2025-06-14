# Jump Game

<br>

link: https://leetcode.com/problems/jump-game/

<br>

---

<br>

## Topic

* Array
* Dynamic Programming
* Greedy

<br>

## Brain Strom

Classic jump gaming, usually this is the first example for demo Dynamic Programming algorithm.

I think there is nothing to say, ez one.

<br>

When it comes to DP, the most difficult thing is how we define the `dp[]`.
So as I know, the dp awlays store the result.

SO the define is:

```
dp[i] = true if you can reach index i from index 0, otherwise false.
```

<br>

Let's try some coding works.

<br>

## Let's Coding


```rust
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {

        if nums.len() == 1 {
            return true;
        }
        
        // Define the DP array, dp[i] = true if we can reach the end from index i
        let mut dp = vec![false; nums.len()];
        // init first DP value as true.
        dp[0] = true;
        let mut steps = nums[0] as usize;
        for i in 0..steps + 1 {
            if i < nums.len() {
                dp[i] = true;
            }
        }

        dbg!("after init dp: {:?}", &dp);

        // Iterate the nums array.
        for i in 1..nums.len() {
            if dp[i] {
                // if we can reach this index, then we can calculate how far we can jump from here.
                steps = nums[i] as usize;
                if steps > 0 {
                    for j in 1..steps+1 {
                        let can_reach = i + j;
                        if can_reach >= nums.len() {
                            break;
                        }
                        // If we can reach this index, then we can mark it as true.
                        dp[i+j] = true;
                    }
                }
            }
        }

        dbg!(&dp);

        dp[nums.len() - 1]
    }
}
```

This func can solved the problem, but I hit Time Limit Exceeded, when it comes to a huge input nums.

So what I did wrong?

The main issue causing TLE is redundant work - updating the same positions multiple times.

<br>
<br>

### Better Approach: Greedy-style DP

<br>

The pointer, we defined the dp is one a right track. the problem is how do we how far we can go.

<br>

```
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        
        // Define the DP array, dp[i] = true if we can reach the end from index i
        let mut dp = vec![false; nums.len()];
        // init first DP value as true.
        dp[0] = true;
        let mut max_reach = nums[0] as usize;

        for i in 0..nums.len() {
            if i > max_reach {
                return false;
            }

                dp[i] = true;
                max_reach = max_reach.max(i + nums[i] as usize);

                if max_reach >= nums.len() - 1 {
                    return true;
                }
        }
        

        dp[nums.len() - 1]
    }
}
```