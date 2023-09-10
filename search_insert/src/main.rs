struct Solution();

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {

        let last_index = nums.len() as i32 - 1;
        fn recurse_search(nums: Vec<i32>, target: i32, left: i32, right: i32) -> i32 {
            if left > right {
                return left;
            }

            let mid = left + (right - left) / 2;

            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] > target {
                recurse_search(nums, target, left, mid - 1);
            } else if nums[mid as usize] < target {
                recurse_search(nums, target, mid - 1, right);
            }

            mid
        }

        recurse_search(nums, target, 0, last_index)

    }
}


fn main() {
    let nums = vec![1,3,5,6];
    let target = 5;

    let output = Solution::search_insert(nums, target);
    println!("{}", output);
}
