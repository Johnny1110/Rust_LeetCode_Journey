mod hard;
mod easy;
mod medium;
mod common;

use core::num;

use crate::common::Solution;

fn main() {
    //let res = Solution::is_match(String::from("abbb"), String::from("a*b"));
    //println!("final answer: {}", res);
    let res = Solution::jump(vec![1,2,3]);
    println!("final answer: {}", res);
}
