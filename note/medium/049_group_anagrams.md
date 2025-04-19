# 49. Group Anagrams

<br>

---

<br>

problem link: https://leetcode.com/problems/group-anagrams/description/

<br>

## Topic 

* Array
* Hash Table
* String
* Sorting

<br>
<br>

## Brain Strom

Given a string array and we have to group the words that using same alphabet together.

As the topic stated, we can utlize hash table and sorting.

I immediately thought of iterating through the string array, sorting the letters in each word in order, then put the word into hash table -- using the sorted letters as key, and orignal word as value. 

Finally we can output all of the hash table's value as 2D array.

It's could be a good apporach to solve this problem maybe. But what if we don't have to sort the letter in every words?

I'm asking chatGPT for a better approach, and here is what I got:

<br>

### Alternative: Frequency Count Approach

How It Works

* Letter Counting:

    Instead of sorting, iterate through each character in the word and build a frequency array (or tuple) for all 26 letters (assuming lowercase letters only). For example, "eat" might be represented as (1, 0, 0, …, 1, …, 1, …, 0) where each position corresponds to a letter.

* Hash Table Insertion:
    Use this frequency tuple as a key and group the original words together.

* Output Construction:
    Extract all the groups from the dictionary and return them.

<br>

I want to try __Frequency Count Approach__ directly.

<br>
<br>


## Resolve Code

```rust
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

            frequency_map.entry(frequency)
            .or_insert(vec![])
            .push(s);
        }

        return frequency_map.into_iter()
            .map(|(_, v)| v)
            .collect();
    }
}
```

