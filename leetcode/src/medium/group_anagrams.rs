use std::collections::HashMap;

use crate::common::Solution;

impl Solution {

    // Using Frequency Count Approach.
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        

        // 1. create a HashMap to store the frequency of each character in the strings.
        // map from "letter frequency array" -> list of strings
        let mut frequency_map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        // 2. iterate through each string in the input vector.
        for s in strs {
            let mut frequency = [0 as u8; 26];

            for byte in s.bytes() {
                frequency[(byte - b'a') as usize] += 1; 
            }

            frequency_map.entry(frequency).or_insert(vec![]).push(s);
        }

        return frequency_map
            .into_iter()
            .map(|(_, v)| v)
            .collect();
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let output = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        let result = Solution::group_anagrams(input);
        assert_eq!(result.len(), output.len());
    }

    #[test]
    fn test_group_anagrams_empty() {
        let input: Vec<String> = vec![];
        let output: Vec<Vec<String>> = vec![];
        let result = Solution::group_anagrams(input);
        assert_eq!(result, output);
    }

    #[test]
    fn test_group_anagrams_single() {
        let input = vec!["a".to_string()];
        let output = vec![vec!["a".to_string()]];
        let result = Solution::group_anagrams(input);
        assert_eq!(result, output);
    }

    #[test]
    fn test_group_anagrams_identical() {
        let input = vec!["abc".to_string(), "abc".to_string()];
        let output = vec![vec!["abc".to_string(), "abc".to_string()]];
        let result = Solution::group_anagrams(input);
        assert_eq!(result, output);
    }

    #[test]
    fn test_byte_to_number() {
        let string = "abc";
        
        for byte in string.bytes() {
            println!("num: {}", byte - b'a');
        }

    }
}
