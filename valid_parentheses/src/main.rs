struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {

        if s.len() % 2 == 1 {
            return false;

        }

        let mut vec_char = Vec::with_capacity(s.len());

        for c in s.chars() {
            match c { 
                '(' | '[' | '{' => vec_char.push(c),
                _ => match vec_char.pop() {
                    Some('(') if c == ')' => (),
                    Some('[') if c == ']' => (),
                    Some('{') if c == '}' => (),
                    _ => return false,
                    
                }
            }
        }

        vec_char.is_empty()
        
    }
}

fn main() {
    let test1 = String::from("()");
    let test2  = String::from("()[]{}");
    let test3 = String::from("(]");
    let test4 = String::from("(");
    let test5 = String::from("()[");
    let test6 = String::from("([])");

    let solution1 = Solution::is_valid(test1);
    let solution2 = Solution::is_valid(test2);
    let solution3 = Solution::is_valid(test3);
    let solution4 = Solution::is_valid(test4);
    let solution5 = Solution::is_valid(test5);
    let solution6 = Solution::is_valid(test6);

    assert!(solution1 == true);
    assert!(solution2 == true);
    assert!(solution3 == false);
    assert!(solution4 == false);
    assert!(solution5 == false);
    assert!(solution6 == true);
}
