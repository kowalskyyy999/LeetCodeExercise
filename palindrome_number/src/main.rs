struct Solution();

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x {
            i32::MIN..=-1 => false,
            0..=i32::MAX => {
                let x_string = x.to_string();
                let mut temp = Vec::new();

                for c in x_string.chars() {
                    temp.push(c);
                }
                temp.reverse();

                let palindrome = temp.into_iter().collect::<String>().parse::<i64>().unwrap();
                palindrome == x as i64
            }
        }
    }
}

fn main() {
    let _a = 121;
    let _b = 123;
    let c = 1000021;

    let palindrome = Solution::is_palindrome(c);

    println!("is {} palindrome? {}", c, palindrome);
}
