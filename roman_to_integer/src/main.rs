#[derive(PartialEq)]
struct Solution();

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // let length_roman = s.len();
        let roman_chars = s.chars();
        let mut prev_chars: char = ' ';
        let mut values = 0;

        for c in roman_chars {
            // println!("{}", prev_chars);
            if c == 'I' {
                let value = 1;
                values += value;
                
            } else if c == 'V' {
                let value = 5;
                if prev_chars == 'I' {
                    values += value - 2 ;
                } else {
                    values += value;
                }
                
            } else if c == 'X' {
                let value = 10;
                if prev_chars == 'I' {
                    values += value - 1 * 2;
                } else if prev_chars == 'V' {
                    values += value - 5 * 2;
                }  else {
                    values += value;
                }
            } else if c == 'L' {
                let value = 50;
                if prev_chars == 'I' {
                    values += value - 1 * 2;
                } else if prev_chars == 'V' {
                    values += value - 5 * 2;
                } else if prev_chars == 'X' {
                    values += value - 10 * 2;
                } else {
                    values += value;
                }
            } else if c == 'C' {
                let value = 100;
                if prev_chars == 'I' {
                    values += value - 1 * 2;
                } else if prev_chars == 'V' {
                    values += value - 5 * 2;
                } else if prev_chars == 'X' {
                    values += value - 10 * 2;
                } else if prev_chars == 'L' {
                    values += value - 50 * 2;
                } else {
                    values += value;
                }
            } else if c == 'D' {
                let value = 500;
                if prev_chars == 'I' {
                    values += value - 1 * 2;
                } else if prev_chars == 'V' {
                    values += value - 5 * 2;
                } else if prev_chars == 'X' {
                    values += value - 10 * 2;
                } else if prev_chars == 'L' {
                    values += value - 50 * 2;
                } else if prev_chars == 'C' {
                    values += value - 100 * 2;
                } else {
                    values += value;
                }
            } else if c == 'M' {
                let value = 1000;
                if prev_chars == 'I' {
                    values += value - 1 * 2;
                } else if prev_chars == 'V' {
                    values += value - 5 * 2;
                } else if prev_chars == 'X' {
                    values += value - 10 * 2;
                } else if prev_chars == 'L' {
                    values += value - 50 * 2;
                } else if prev_chars == 'C' {
                    values += value - 100 * 2;
                } else if prev_chars == 'D' {
                    values += value - 500 * 2;
                } else {
                    values += value;
                }
            } 
            prev_chars = c;
        }

        values

    }

}

fn main() {
    let test_case1 = String::from("III");
    let test_case2 = String::from("LVIII");
    let test_case3 = String::from("MCMXCIV");
    let test_case4 = String::from("CD");

    let result_1 = Solution::roman_to_int(test_case1);
    let result_2 = Solution::roman_to_int(test_case2);
    let result_3 = Solution::roman_to_int(test_case3);
    println!("{}", Solution::roman_to_int(test_case4));

    assert!(result_1 == 3);
    assert!(result_2 == 58);
    assert!(result_3 == 1994);
}
