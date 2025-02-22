mod hard;
mod easy;
mod medium;
mod common;

use crate::common::Solution;

fn main() {
    //let res = Solution::is_match(String::from("abbb"), String::from("a*b"));
    //println!("final answer: {}", res);

    let res = Solution::is_match(String::from("a"), String::from("aa"));
    println!("final answer: {}", res);
}
