fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let nums = input
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let list = vec_to_list(&nums);
    let res = Solution::swap_pairs(list);
    let res = list_to_vec(&res);
    println!("{:?}", res);
}

fn vec_to_list(nums: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut cur = &mut dummy;
    for &num in nums {
        cur.next = Some(Box::new(ListNode::new(num)));
        cur = cur.next.as_mut().unwrap();
    }
    dummy.next
}

fn list_to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut cur = list;
    while let Some(node) = cur {
        res.push(node.val);
        cur = &node.next;
    }
    res
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution {}
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut node1 = &mut dummy;
        while let Some(mut node2) = node1.next.take() {
            if let Some(mut node3) = node2.next.take() {
                let node4 = node3.next.take();
                node2.next = node4;
                node3.next = Some(node2);
                node1.next = Some(node3);
                node1 = node1.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                node1.next = Some(node2);
                break;
            }
        }

        dummy.next
    }
}

/*
input:
[1,2,3,4]
output:
[2,1,4,3]
*/