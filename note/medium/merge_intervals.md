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
        if intervals.len() <= 1 {
            return intervals;
        }

        // 1. first of all, we need to sort the intervals based on the first element.
        let mut intervals = intervals;
        intervals.sort_by_key(|interval| interval[0]);
        dbg!(&intervals);

        // 2. merge the intervals.
        let mut merged_intervals: Vec<Vec<i32>> = Vec::new();

        let mut current_interval = if let Some(first) = intervals.first() {
            first.clone()
        } else {
            panic!("The intervals vector is empty, cannot merge.")
        };
    
        for idx in 1..intervals.len() {
            let interval = &intervals[idx];
            if interval[0] <= current_interval[1] {
                // merge this intervals
                current_interval[1] = current_interval[1].max(interval[1]);
            } else {
                // push current interval to merged_intervals
                merged_intervals.push(current_interval);
                // init current_interval
                current_interval = interval.clone();
            }
        }

        // push the last interval
        merged_intervals.push(current_interval);

        merged_intervals
    }
}
```


<br>

I solved this problem, but runtime result did not satisfy me.

![1](imgs/056_1.jpg)

-. -

<br>

Let's revamp this.

<br>

## Revamp

<br>

What's wrong with it? I think that maybe because of I am using clone too much.

Nope. it's because I forgot to delete `!dbg()`

![2](imgs/056_2.jpg)

<br>

Revamp Code:

```rust
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() <= 1 {
            return intervals;
        }

        // 1. first of all, we need to sort the intervals based on the first element.
        let mut intervals = intervals;
        intervals.sort_by_key(|interval| interval[0]);

        // 2. merge the intervals.
        let mut merged_intervals: Vec<Vec<i32>> =Vec::with_capacity(intervals.len());
        let mut current_interval = intervals[0].clone();

        for interval in intervals.into_iter().skip(1) {
            if interval[0] <= current_interval[1] {
                // merge the intervals
                current_interval[1] = current_interval[1].max(interval[1]);
            } else {
                // push 
                merged_intervals.push(current_interval);
                current_interval = interval;
            }
        }
    

        // push the last interval
        merged_intervals.push(current_interval);

        merged_intervals
    }
}
```

Removed Unnecessary Clone and __pre-allocate__ merged_intervals size.

![3](imgs/056_3.png)