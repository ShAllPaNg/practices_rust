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
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let n = input.trim().parse::<i32>().unwrap();
    let mut dummy = Some(Box::new(ListNode::new(0)));

    let mut cur = dummy.as_mut().unwrap().as_mut();
    for &num in nums.iter() {
        cur.next = Some(Box::new(ListNode::new(num)));
        cur = cur.next.as_mut().unwrap().as_mut();
    }
    let head = dummy.unwrap().next;
    let mut res = Solution::remove_nth_from_end(head, n);
    let mut output = Vec::<i32>::new();
    while let Some(node) = res {
        output.push(node.val);
        res = node.next;
    }
    println!("{:?}", output);
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut fast = dummy.as_ref().unwrap().next.as_ref();
        let mut slow = dummy.as_ref();
        for _ in 0..n {
            fast = fast.unwrap().next.as_ref();
        }
        while fast.is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref();
        }
        let slow = slow.unwrap();

        #[allow(mutable_transmutes)]
        let slow: &mut Box<ListNode> = unsafe { std::mem::transmute(slow) };
        slow.next = slow.next.take().unwrap().next; //or slow.next = slow.next.take()?.next; 

        dummy.unwrap().next
    }
}

/*
input:
[1,2,3,4,5]
2
output:
[1,2,3,5]


input:
[1]
1
output:
[]
*/
