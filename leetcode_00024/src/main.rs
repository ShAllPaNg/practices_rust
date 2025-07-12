use std::mem::swap;

fn main() {
    println!("Hello, world!");
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
                let mut node4 = node3.next.take();
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
