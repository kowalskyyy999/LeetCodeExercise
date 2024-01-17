use std::collections::HashMap;

struct Solution ();

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32{
        
        match s.len() {
            1 => {1},
            _ => {
                let mut start: i32 = 0;
                let mut result: i32 = 0;
                let mut seen: HashMap<char, usize> = std::collections::HashMap::new();
        
                let characters = s.chars();
        
                for (i, c) in characters.enumerate() {
                    
                    if seen.get(&c).map_or(0 as usize, |v| *v) >= start as usize {
                        start = seen.get(&c).map_or(0 as i32, |v| *v as i32) + 1;
                    }
                    
                    result = std::cmp::max(result, i as i32 - start + 1);
                    seen.insert(c, i);
                }
        
                result 
            }
        }
    }
}

fn main() {
    // println!("Hello, world!");

    let test_case1= String::from("abcabccbb");
    let test_case2 = String::from("bbbb");
    let test_case3 = String::from("pwwkew");
    let test_case4 = String::from(" ");

    let result = Solution::length_of_longest_substring(test_case4);
    println!("{result}");
}
