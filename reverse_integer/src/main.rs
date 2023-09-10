use std::collections::HashMap;

struct Solution();

impl Solution {
    pub fn reverse(x: i32) -> i32 {

        if x == 0 {
            0
        } else {

            let x_str = x.to_string();
            let mut x_chars: Vec<char> = Vec::new();
            let mut resutt_char: Vec<char> = Vec::new();
    
            for c in x_str.chars() {
                x_chars.push(c);
            }
    
            let mut index = x_chars.len() as i32 - 1;
            let bdr_index = index - 1;
    
            while index >= 0 {
                let value = x_chars[index as usize];
                if index > 0 && index <= bdr_index{
                    
                    let next_value = x_chars[(index - 1 ) as usize];
                    let prev_value = x_chars[(index + 1) as usize];
                    
                    if value == '0' && next_value != '0' {
                        resutt_char.push(value);
                    } else if value == '0' && prev_value != '0' {
                        resutt_char.push(value);
                    }else if value != '0' {
                        resutt_char.push(value);
                    }
                } else {
                    if value != '0' {
                        resutt_char.push(value);
                    }

                }
    
                if value == '-' {
                    resutt_char.insert(0, value);
                    resutt_char.pop();
                }
                index = index - 1;
            }
            
            if resutt_char[0] == '0' {
                resutt_char.remove(0);
            }

            let result_str = resutt_char.iter().collect::<String>();
            let result: i64 = result_str.parse().unwrap();
            
            if result > 2147483647 || result < -2147483648 {
                0
            } else {
                result as i32
            }
        }


    }
}

fn main() {
    let input1 = 123;
    let input2 = -123;
    let input3 = 120;
    let input4 = 0;
    let input5 = 901000;
    let input6 = 1534236469;
    let input7: i32 = 102;
    let input8 = 50078;

    Solution::reverse(input5);
    Solution::reverse(input7);
    Solution::reverse(input8);
    // assert!(Solution::reverse(input1) == 321);
    // assert!(Solution::reverse(input2) == -321);
    // assert!(Solution::reverse(input3) == 21);
}
