fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let list1 = handle_input(&input);

    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let list2 = handle_input(&input);

    let res = Solution::merge_two_lists(list1, list2);
    let res = list_to_vec(&res);
    println!("{:?}", res);
}

fn handle_input(input: &String) -> Option<Box<ListNode>> {
    let nums1 = input
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let list1 = vec_to_list(&nums1);
    list1
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

//Option中的元素不支持copy()时，Option实例不用take()也会转移元素所有权，但原实例无效化(不能再使用)，使用take()转移所有权后，原实例为None
struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = dummy.as_mut();
        let mut list1 = list1;
        let mut list2 = list2;
        while let (Some(node1), Some(node2)) = (list1.as_ref(), list2.as_ref()) {
            if node1.val <= node2.val {
                cur.next = list1;
                cur = cur.next.as_mut().unwrap();
                list1 = cur.next.take();
            } else {
                cur.next = list2;
                cur = cur.next.as_mut().unwrap();
                list2 = cur.next.take();
            };
        }
        cur.next = list1.or(list2);
        dummy.next
    }
}

struct Solution2 {}
impl Solution2 {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = dummy.as_mut();
        let mut list1 = list1;
        let mut list2 = list2;
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                cur.next = list1;
                cur = cur.next.as_mut().unwrap();
                list1 = cur.next.take();
            } else {
                cur.next = list2;
                cur = cur.next.as_mut().unwrap();
                list2 = cur.next.take();
            };
        }
        cur.next = list1.or(list2);
        dummy.next
    }
}

/*
input:
[1,2,4]
[1,3,4]
output:
[1,1,2,3,4,4]



*/
