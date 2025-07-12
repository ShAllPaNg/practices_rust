fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let lists = input
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split("],[")
        .map(|s| vec_to_list(&string_to_vec(s)))
        .collect::<Vec<Option<Box<ListNode>>>>();
    let res = Solution::merge_k_lists(lists);
    let res = list_to_vec(&res);
    println!("{:?}", res);
}

fn string_to_vec(s: &str) -> Vec<i32> {
    s.trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }
        let mut lists = lists;
        let mut res = None;
        for list in lists {
            res = Self::merge_two_lists(res, list);
        }
        res
    }

    fn merge_two_lists(
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

use std::{
    cmp::{Ord, PartialOrd, Reverse},
    collections::BinaryHeap,
};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.val.cmp(&other.val))
    }
}
struct Solution2 {}
impl Solution2 {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut node = None;
        let mut cur = &mut node;
        let mut heap = BinaryHeap::new();
        for list in lists {
            if let Some(x) = list {
                heap.push(Reverse(x));
            }
        }

        while let Some(mut x) = heap.pop() {
            if let Some(y) = x.0.next.take() {
                heap.push(Reverse(y));
            }
            cur = &mut cur.insert(x.0).next;
        }

        node
    }
}

struct Solution3 {}
impl Solution3 {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut node = None;
        let mut cur = &mut node;
        let mut heap = BinaryHeap::new();
        for mut list in lists {
            while let Some(mut x) = list.take() {
                list = x.next.take();
                heap.push(Reverse(x));
            }
        }

        while let Some(mut x) = heap.pop() {
            cur = &mut cur.insert(x.0).next;
        }

        node
    }
}

/*
input:
[[1,4,5],[1,3,4],[2,6]]
output:
[1,1,2,3,4,4,5,6]


*/
