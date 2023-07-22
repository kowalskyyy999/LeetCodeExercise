struct Solution();

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx += 1;
            }
            println!("{:?}", nums);
        }
        idx as i32
    }
}

fn main() {
    let mut nums = vec![0,1,2,2,3,0,4,2];
    let val = 2;

    let result = Solution::remove_element(&mut nums, val);
    println!("Result: {}", result);

}
