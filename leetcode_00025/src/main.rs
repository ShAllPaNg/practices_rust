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
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let list = vec_to_list(&nums);
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let k = input.trim().parse::<i32>().unwrap();
    let res = Solution2::reverse_k_group(list, k);
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur = &mut head;
        for _ in 0..k {
            if let Some(node) = cur.as_mut() {
                cur = &mut node.next;
            } else {
                return head;
            }
        }

        let mut new_head = Self::reverse_k_group(cur.take(), k);

        for _ in 0..k {
            if let Some(mut node) = head.take() {
                head = node.next.take();
                node.next = new_head.take();
                new_head = Some(node);
            }
        }

        new_head
    }
}

//和迭代思路基本一致，切分成多个链表段，从后往前翻转，先翻转的链接到后翻转的链表之后
struct Solution2 {}
impl Solution2 {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut save = Vec::<Option<Box<ListNode>>>::new();
        let (mut list1, mut list2, mut flag) = Self::cut_list(head, k);
        while flag {
            save.push(list1.take());
            (list1, list2, flag) = Self::cut_list(list2.take(), k);
        }
        let mut new_head = list1.take();
        for list in save.into_iter().rev() {
            new_head = Self::reverse_list_and_link_tail(list, new_head.take());
        }

        new_head
    }

    fn cut_list(
        head: Option<Box<ListNode>>,
        k: i32,
    ) -> ((Option<Box<ListNode>>, Option<Box<ListNode>>, bool)) {
        if head.is_none() {
            return (None, None, false);
        }
        let mut head = head;
        let mut cur = &mut head;
        for _ in 0..k - 1 {
            if let Some(_node) = cur.as_mut().unwrap().next.as_mut() {
                cur = &mut cur.as_mut().unwrap().next;
            } else {
                return (head, None, false);
            }
        }
        let new_head = cur.as_mut().unwrap().next.take();//next置为None，断开两条链连接
        (head, new_head, true)
    }

    fn reverse_list_and_link_tail(
        head: Option<Box<ListNode>>,
        tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut new_head = tail;//翻转部分接在尾巴前面
        while let Some(mut node) = head.take() {
            head = node.next.take();
            node.next = new_head.take();
            new_head = Some(node);
        }
        new_head
    }
}

/*
input:
[1,2,3,4,5]
2
output:
[2,1,4,3,5]
*/
