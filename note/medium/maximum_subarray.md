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

Alright! now we know how's that DP looks like, let's implement this solution by code.

## Let's coding

```rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // TODO
    }
}
```