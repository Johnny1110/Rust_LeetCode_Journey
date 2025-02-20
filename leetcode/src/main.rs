mod hard;
mod easy;
mod medium;
mod common;

fn main() {
    // Call a hard problem's test function
    let res = hard::wildcard_matching::Solution::is_match(String::from("a"), String::from("a"));
    println!("{:#?}", res);
}
