use std::{cmp::min, ops::Index, fmt::format};

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = String::new();

        if strs.len() < 2 {
            let first = strs.get(0).unwrap().to_owned();
            if first.is_empty() {
                return ans;
            } else {
                for i in 0..first.len() {
                    let c = first.chars().nth(i).unwrap();

                    if c.is_alphabetic() {
                        ans.insert(i, c)
                    }
                }
                return ans;
            }

        } else {
            let mut strs = strs.clone();
        
            strs.sort();

            let len_strs = strs.len();
            
            let first = strs.get(0).unwrap().to_owned();
            let last = strs.get(len_strs - 1).unwrap().to_owned();

            if first.is_empty() | last.is_empty() {
                return ans;
            } else {
                for i in 0..=min(first.len() - 1, last.len() - 1) {
                    if first.chars().nth(i).unwrap() != last.chars().nth(i).unwrap() {
                        return ans;
                    }

                    ans.insert(i, first.chars().nth(i).unwrap())
                }
                return  ans;
            }
        }
    }
}


fn main() {
    // println!("Hello, world!");

    let test1 = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    let test2 = vec![String::from("dog"), String::from("racecar"), String::from("car")];
    let test3 = vec![String::from("")];
    let test4 = vec![String::from("a")];
    let test5 = vec![String::from("','")];
    let test6 = vec!["",""];
    let test7 = vec![String::from("ab"), String::from("a")];

    // println!("{:?}", test6.len());

    // let answer1 = Solution::longest_common_prefix(test1);
    // let answer2 = Solution::longest_common_prefix(test2);
    // let answer3 = Solution::longest_common_prefix(test3);
    // let answer4 = Solution::longest_common_prefix(test4);
    // let answer5 = Solution::longest_common_prefix(test5);
    // let answer6 = Solution::longest_common_prefix(test6);
    let answer7 = Solution::longest_common_prefix(test7);

    // println!("{}", answer1);
    // println!("{}", answer2);
    // println!("{}", answer3);
    // println!("{}", answer4);
    // println!("{}", answer5);
    // println!("{}", answer6);
    // println!("{}", answer7);
}
