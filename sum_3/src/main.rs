struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out = Vec::new();
        let mut nums = nums.clone();

        if nums.capacity() == 3 {
            if nums[0] + nums[1] + nums[2] == 0 {
                out.insert(0, nums);
                return out;
            } else {
                // out.insert(0, vec![]);
                return out;
            }
        }

        nums.sort();

        let mut index = 0;

        for i in 0..nums.len() {
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                if nums[i] + nums[j] + nums[k] == 0 {
                    let mut out_temp = vec![nums[i], nums[j], nums[k]];
                    out_temp.sort();

                    if !out.contains(&out_temp) {
                        out.insert(index, out_temp);
                        index += 1;
                    }
                    j += 1;
                    k -= 1;
                } else if nums[i] + nums[j] + nums[k] < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        
        // for i in 0..nums.len() {
        //     for j in i..nums.len() {
        //         for k in j..nums.len() {
        //             if (i != j) & (i != k) & (j != k) {
        //                 if nums[i] + nums[j] + nums[k] == 0 {
        //                     let mut out_temp = vec![nums[i], nums[j], nums[k]];
        //                     out_temp.sort();
        //                     if !out.contains(&out_temp) {
        //                         out.insert(index, out_temp);
        //                         index += 1;
        //                     }                            
        //                 }
        //             }
        //         }
        //     }
        // }
        out
    }
}


fn main() {
    let test1 = vec![-1, 0, 1, 2, -1, -4];
    let test2 = vec![0, 1, 1];
    let test3 = vec![0, 0, 0];
    let test4 = vec![0, 0, 0, 0];
    let test5 = vec![3, 0, -2, -1, 1, 2];

    // [3,0,-2,-1,1,2]

    let out1 = Solution::three_sum(test1);
    let out2 = Solution::three_sum(test2);
    let out3 = Solution::three_sum(test3);
    let out4 = Solution::three_sum(test4);
    let out5 = Solution::three_sum(test5);

    println!("{:?}", out1);
    println!("{:?}", out2);
    println!("{:?}", out3);
    println!("{:?}", out4);
    println!("{:?}", out5);
    

}
