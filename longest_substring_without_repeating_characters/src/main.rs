struct Solution ();

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32{
        let chars = s.chars();
        let mut unique_char: Vec<char> = Vec::new();

        for c in chars {
            if unique_char.contains(&c) != true {
                unique_char.push(c);
            }
        }

        unique_char.len() as i32

    }
}

fn main() {
    // println!("Hello, world!");

    let test_case1= String::from("abcabccbb");
    let test_case2 = "bbbb";
    let test_case3 = "pwwkew";

    Solution::length_of_longest_substring(test_case1);
}
