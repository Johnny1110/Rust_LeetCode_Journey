# 46.  Permutations

<br>

---

<br>

https://leetcode.com/problems/permutations/description/

<br>

## Topic

* Array
* Backtracking


<br>

---

<br>

## Brain Strom

I remember I did this before in Golang implement [link](https://github.com/Johnny1110/GO_LeetCode_Journey/tree/main/bonus/permutation).

just convert to rust language. let try this with backtracking approach, and I think it is gonna be more difficult because of Rust's ownership stuff...

<br>

### Backtracking

```rust
impl Solution {

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {

        // create a resut vector.
        let mut results: Vec<Vec<i32>> = vec![];
        let mut nums = nums;

        // call the backtracking function.
        Self::backtracking(&mut results, &mut nums, 0);

        return results;
    }

    fn backtracking(results: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, idx: usize) {
        

        // if the idx is equal to the length of the nums, copy the vector to the results and return.
        if idx == nums.len() {
            results.push(nums.clone());
            return;
        }

        // do swapping.
        for i in idx..nums.len() {
            nums.swap(idx, i);
            Self::backtracking(results, nums, idx + 1);
            nums.swap(idx, i);
        }
    }

}
```

<br>

pretty ez.