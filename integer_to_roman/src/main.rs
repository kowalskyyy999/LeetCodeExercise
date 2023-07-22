struct Solution();

impl Solution {
    pub fn int_to_roman(num: i32) -> String {

        let mut roman = String::new();
        let mut num = num;
        let mut index = 0 ;
        loop {
            if num / 1000 >= 1 {
                roman.insert(index, 'M');
                num -= 1000;
                index += 1;


            } else if num / 500 >= 1 {
                if num < 1000 && num >= 900 {
                    roman.insert_str(index, "CM");
                    num -= 900;
                    index += 2;   
                } else {
                    roman.insert(index, 'D');
                    num -= 500;
                    index += 1;
                }


            } else if num / 100 >= 1 {
                if num < 500 && num >= 400 {
                    roman.insert_str(index, "CD");
                    num -= 400;
                    index += 2;
                } else {
                    roman.insert(index, 'C');
                    num -= 100;
                    index += 1;
                }
   
            } else if num / 50 >= 1 {

                if num >= 90 {
                    roman.insert_str(index, "XC");
                    num -= 90;
                    index += 2;
                } else if num < 50 && num >= 40 {
                    roman.insert_str(index, "XL");
                    num -= 40;
                    index += 2;
                    
                } else {
                    roman.insert(index, 'L');
                    num -= 50;
                    index += 1;
                }

            } else if num / 10 >= 1 {
                if num < 50 && num >= 40 {
                    roman.insert_str(index, "XL");
                    num -= 40;
                    index += 2;
                } else if num < 10 && num >= 9 {
                    roman.insert_str(index, "IX");
                    num -= 9;
                    index += 2;
                } else {
                    roman.insert(index, 'X');
                    num -= 10;
                    index += 1;
                }
                
            } else if num / 5 >= 1 {

                if num < 10 && num >= 9 {
                    roman.insert_str(index, "IX");
                    num -= 9;
                    index += 2;
                } else if num < 5 && num >= 4 {
                    roman.insert_str(index, "IV");
                    num -= 4;
                    index += 2;
                    
                } else {
                    roman.insert(index, 'V');
                    num -= 5;
                    index += 1;
                }
                
            } else if num / 1 >= 1 {
                if num < 5 && num >= 4 {
                    roman.insert_str(index, "IV");
                    num -= 4;
                    index += 2;
                } else {
                    roman.insert(index, 'I');
                    num -= 1;
                    index += 1;
                }
            } else {
                break;
            }
        }

        roman

    }
}

fn main() {

    let test_case1 = 3;
    let test_case2 = 58;
    let test_case3 = 1994;
    let test_case4 = 2000;

    let result_case1 = Solution::int_to_roman(test_case1);
    let result_case2 = Solution::int_to_roman(test_case2);
    let result_case3 = Solution::int_to_roman(test_case3);
    let result_case4 = Solution::int_to_roman(test_case4);

    assert!(result_case1 == "III");
    assert!(result_case2 == "LVIII");
    assert!(result_case3 == "MCMXCIV");
    assert!(result_case4 == "MM");
}


