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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(-1));
        let mut rear = head.as_mut();
        #[warn(unused_assignments)]
        let (mut new_value, mut increase) = (0, 0);

        while l1 != None || l2 != None {
            let (value1, value2) = (
                if l1.is_some() {
                    let temp = l1.as_ref().unwrap().val;
                    l1 = l1.unwrap().next;
                    temp
                } else {
                    0
                },
                if l2.is_some() {
                    let temp = l2.as_ref().unwrap().val;
                    l2 = l2.unwrap().next;
                    temp
                } else {
                    0
                },
            );
            let sum = value1 + value2 + increase;
            (new_value, increase) = (sum % 10, sum / 10);
            rear.next = Some(Box::new(ListNode::new(new_value)));
            rear = rear.next.as_mut().unwrap();
            l1 = l1
        }
        if increase == 1 {
            rear.next = Some(Box::new(ListNode::new(1)));
        }

        head.next
    }
}
