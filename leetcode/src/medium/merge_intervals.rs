use crate::common::Solution;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        let input = vec![vec![2, 6], vec![1, 3], vec![15, 18], vec![8, 10]];
        let output = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        let result = Solution::merge(input);
        assert_eq!(result, output);
    }

    #[test]
    fn test_merge_intervals_empty() {
        let input: Vec<Vec<i32>> = vec![];
        let output: Vec<Vec<i32>> = vec![];
        let result = Solution::merge(input);
        assert_eq!(result, output);
    }
}