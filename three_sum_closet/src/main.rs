use std::f64::INFINITY;

struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {

        let mut gaps = INFINITY as i32;
        let mut out = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                for k in j..nums.len() {
                    if (i != j) & (i != k) & (j != k) {
                        if (nums[i] + nums[j] + nums[k] - target).abs() < gaps {
                            gaps = (nums[i] + nums[j] + nums[k] - target).abs();
                            out = nums[i] + nums[j] + nums[k];
                        }
                    }
                }
            }
        }
        out
    }
}



fn main() {
    // println!("Hello, world!");
    let test1 = vec![-1, 2, 1, -4];
    let target1 = 1;
    let answer1 = Solution::three_sum_closest(test1, target1);
    println!("{}", answer1);

    let test2 = vec![0, 0, 0];
    let target2 = 1;
    let answer2 = Solution::three_sum_closest(test2, target2);
    println!("{}", answer2);

    let test3 = vec![1,1,1,0];
    let target3 = -100;
    let answer3 = Solution::three_sum_closest(test3, target3);
    println!("{}", answer3);
}
