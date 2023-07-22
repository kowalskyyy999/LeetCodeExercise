use core::num;

struct Solution ();

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) {
        let mut in_degree = vec![0; num_courses as usize];
        let mut descendants: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
        println!("{:?}", &in_degree);
        println!("{:?}", &descendants);

        for p in prerequisites {
            println!("p: {:?}", &p);
            in_degree[p[0] as usize] += 1;
            descendants[p[1] as usize].push(p[0]);
        }

        println!("{:?}", &in_degree);
        println!("{:?}", &descendants);

        let mut queue = vec![];

        for (i, d) in in_degree.iter().enumerate(){
            if *d == 0 {
                queue.push(i)
            }
        }
        println!("queue: {:?}", queue);

        let mut finished = 0;
        while let Some(i) = queue.pop(){
            finished += 1;
            for &j in descendants[i].iter() {
                in_degree[j as usize] -= 1;

                if in_degree[j as usize] == 0 {
                    queue.push(j as usize);
                }
            }
        }

        println!("queue: {:?}", queue);
        println!("{}", finished == num_courses);
    }
}


fn main() {
    let num_courses = 2;
    // let prerequisites:Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1]];
    let prerequisites:Vec<Vec<i32>> = vec![vec![1, 0]];

    let is_finish = Solution::can_finish(num_courses, prerequisites);
}
