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

```rust
// do swapping
        for i in idx..nums.len() {
            
            // decide if we need to skip the current number.
            if i > 0 && nums[i] == nums[i-1] && ??? {
                continue;
            }

            nums.swap(idx, i);
            Self::_backtracking(results, nums, idx+1);
            nums.swap(idx, i);
        }
```

the most important part is decide if we need to skip the current number.

if we need to skip this number, it must meet three conditions.

1. i > 0
2. nums[i] == nums[i-1] // meet duplicate to previous one
3. those 2 duplicate nums not been count in.

<br>