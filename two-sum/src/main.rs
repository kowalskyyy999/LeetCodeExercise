use core::num;


#[derive(Debug)]
struct Solution();

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut outputs: Vec<i32> = Vec::new();

        let mut length = nums.len();
        
        for i in 0..length {
            let a = nums[i];
            for j in i+1..length {
                let b = nums[j];
                if a + b == target {
                    let index1: i32 = i.to_string().parse().unwrap();
                    let index2: i32 = j.to_string().parse().unwrap();
                    outputs.push(index1);
                    outputs.push(index2);
                }
            }

        }

        outputs
    }
}


fn main() {

    let nums = vec![2,7,11,15];
    let target = 9;

    let output_solution = Solution::two_sum(nums, target);
    println!("{:?}", output_solution);


}
