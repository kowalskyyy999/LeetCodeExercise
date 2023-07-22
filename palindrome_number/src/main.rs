struct Solution();

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_string = x.to_string();
        let first_char = x_string.chars().next().unwrap();
        
        &x_string.starts_with(first_char) == &x_string.ends_with(first_char)
    }
}

fn main() {
    let _a = 121;
    let _b = 123;
    let c = 1000021;

    let palindrome = Solution::is_palindrome(c);

    println!("is {} palindrome? {}", c, palindrome);
}
