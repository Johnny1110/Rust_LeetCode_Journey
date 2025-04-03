# 47. Permutations II

<br>

---

<br>


https://leetcode.com/problems/permutations-ii/description/


<br>

## Topic

* Array
* Backtracking
* Sorting

<br>

---

<br>

## Brain Strom

<br>

Extention permutations problem, we got duplicates in input nums.

I think it basically same as permutations (simply using backtracking), but we have to figure out how to reduce duplicate result.

There are many way to do that, like remove duplicate result after all backtracking calculation. But I think it not the best efficient approach.

The Topic give us a hint about __sort__, then what does sort do in this case？

<br>

When we sort the `nums` in first place, we can make all duplicate numbers group together (I mean they are adjacent). This "group" allow us easily detect when we gonna use duplicate number in the same "decision level" in our backtracking recursion.

let's try to sort `nums` and figure out when should we skip backtracking decision level. 

<br>

![1](imgs/047_1.png)

<br>

solution:

<br>

![1](imgs/047_2.png)

<br>

```rust
impl Solution {

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // create a result vector.
        let mut results: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        let mut used: Vec<bool> = vec![false; nums.len()];
        let mut current: Vec<i32> = vec![];

        // sort nums in first place
        nums.sort();
        
        // call the backtracking function.
        Self::_backtracking(&mut results, &nums, &mut current, &mut used);

        results
    }

    fn _backtracking(results: &mut Vec<Vec<i32>>, nums: &Vec<i32>, current: &mut Vec<i32>, visited: &mut Vec<bool>) {
        if current.len() == nums.len() {
            results.push(current.clone());
            return;
        }

        for i in 0..nums.len() {
            // 1. skip the visited numbers.
            if visited[i] {
                continue;
            }

            // !!! IMPORTANT !!!
            // Skip duplicates: if the current number is the same as the previous one
            // and the previous one is not used in this permutation branch (already put into result in previous step).
            if i > 0 && nums[i] == nums[i-1] && !visited[i-1] {
                continue;
            }
            
            // 2. put the current number into the current vector.
            current.push(nums[i]);
            visited[i] = true;
            
            // 3. go to the next level.
            Self::_backtracking(results, nums, current, visited);

            // 4. rollback the current number.
            current.pop();
            visited[i] = false;
        }

    }
}
```

<br>