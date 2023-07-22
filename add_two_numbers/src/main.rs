#[derive(Debug)]
pub struct Solution();

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl Solution {
    pub fn add_two_numberse(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l1 = l1.clone();
        let l2 = l2.clone();

        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                let total = n1.val + n2.val;
                if total < 10 {
                    Some(Box::new(ListNode {
                        val: total,
                        next: Solution::add_two_numberse(n1.next, n2.next)
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: total - 10,
                        next: Solution::add_two_numberse(Solution::add_two_numberse(carry, n1.next), n2.next)
                    }))
                }
            },
            (Some(n), _) | (_, Some(n)) => {
                Some(n)
            },
            (_, _) => {
                None
            }
            
        }
    } 
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None 
        }
    }    
}

fn helper(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut current = &mut head;

    for x in v {
        current.next = Some(Box::new(ListNode::new(x)));
        current = current.next.as_mut().unwrap();
    }
    // println!("{:?}", head);
    head.next
}


fn main() {

    let l1 = vec![9,9,9,9,9,9,9];
    let l2 = vec![9,9,9,9];

    let outputs = Solution::add_two_numberse(helper(l1.clone()), helper(l2.clone())).unwrap();

    println!("{:?}", outputs);
}
