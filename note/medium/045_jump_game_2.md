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

        // init 1D array as DP, eveny element is i32::MAX
        let mut dp = vec![i32::MAX; nums.len()];

        // init the first element as 0
        dp[0] = 0;

        // iterate the nums array
        nums.iter()
            .enumerate()
            .for_each(|(position, steps)| {
                for i in position+1..position+(steps+1) as usize {
                    if i >= nums.len() {
                        break;
                    }
                    dp[i] = Self::min(dp[i], dp[position] + 1);
                }
            });
        
        dp[nums.len()-1]
    }

    fn min(a: i32, b: i32) -> i32 {
        if a < b {
            return a;
        }
        b
    }
}
```

<br>

---

<br>


### Greedy Approach

Now, let's think about using the Greedy Approach to solve this problem.

When we iterate over the nums array, we can see how many step we can move, and we are also 
looking for the next slot that will allow us to move further.

<br>

For example, consider `nums: [2, 3, 1, 1, 4]`

When we iterate over the first element `2`, we know we can move the `nums[1]` and `nums[2]`

* `nums[1]` is 3
* `nums[2]` is 1

Obviously we want to move to `nums[1]`, because it provider more step to move forward.

```rust
abs_step -> i + step

nums[1] -> abs_step = 1 + 3 = 4
nums[2] -> abs_step = 2 + 1 = 3

Since 4 > 3, we choose nums[1] as the next step.
```

<br>

We iterate through the array and choose the index with largest `abs_step` as our next step.
continuing until we reach out the the of nums array.

Let's drill down into this approach further.

<br>

```rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let the_end_position = (nums.len() as i32) - 1;
        let mut total_jump_count = 0;
        let mut current_position = 0;


        while current_position < the_end_position {

            // find the farthest position we can reach
            let possible_jump_step = nums[current_position as usize];
            let mut farthest_position = current_position + possible_jump_step;

            let mut expect_next_round_idx_temp = 0;

            for i in 1..possible_jump_step+1 {

                let abs_jump_position = current_position + i;


                if abs_jump_position >= the_end_position {
                    farthest_position = abs_jump_position;
                    break;
                }

                // find the best value in the possible jump
                let expect_next_round_idx = abs_jump_position + nums[abs_jump_position as usize];
                if (expect_next_round_idx > expect_next_round_idx_temp) {
                    farthest_position = abs_jump_position;
                    expect_next_round_idx_temp = expect_next_round_idx;
                }

            }
            
            current_position = farthest_position;
            total_jump_count += 1;
        }
        
        return total_jump_count;
    }
}
```

done.