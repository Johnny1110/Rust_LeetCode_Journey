# Merge Intervals

<br>

---

<br>


link: https://leetcode.com/problems/merge-intervals/description

<br>
<br>

---

## Thinking

<br>

At first, I wanna trying to guess the topic.

 hmmm, I have no idea, I think juest iterate though all the intervals, and create result intervals based on some conditions, or combind into exists intervals.

 Let's check the topic hit:

* Array
* Sorting

<br>

Yeah, seems like I'm on the right track. I think the only problem is how to sorting the intervals, maybe based on `interval[0]` which means we can sort by intervals start number. 

After we got this ordered intervals array. we can do the aggregate work easily. 

```rust
intervals.sort_by_key(|interval| interval[0])
```

<br>
<br>

## Let's Coding


```rust
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // TODO
    }
}
```
