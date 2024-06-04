// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}


impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut pre: Option<Box<ListNode>> = None;

        while let Some(mut cur_node) = cur {
            let next = cur_node.next.take();
            cur_node.next = pre;
            pre = Some(cur_node);
            cur = next;
        }

        pre
    }

    pub fn reverse_list2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            let nxt = node.next;
            node.next = pre;
            pre = Some(node);
            cur = nxt;
        }
        pre
    }

    pub fn reverse_list3(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre: Option<Box<ListNode>> = None;

        while let Some(mut cur_node) = head.as_mut() {
            let next = cur_node.next.take();
            cur_node.next = pre;
            pre = head;
            head = next;
        }

        pre
    }

    // pub fn reverse_list4(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut pre: Option<Box<ListNode>> = None;
    //     let mut cur = &head;
    //
    //     while !cur.is_none() {
    //         let next = &cur.as_ref().unwrap().next;
    //         cur.as_mut().unwrap().next = pre.clone();
    //         cur = next;
    //     }
    //     pre
    // }

    pub fn reverse_list5(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre: Option<Box<ListNode>> = None;
        let mut cur = head;

        while !cur.is_none() {
            let mut next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = pre.clone();
            pre = cur;
            cur = next.take();
        }
        pre
    }
}

#[test]
fn main() {
    // 创建一个链表：1 -> 2 -> 3 -> None
    let third = Some(Box::new(ListNode { val: 3, next: None }));
    let second = Some(Box::new(ListNode { val: 2, next: third }));
    let first = Some(Box::new(ListNode { val: 1, next: second }));

    // 反转链表
    let reversed_list = Solution::reverse_list5(first);

    // 打印反转后的链表
    let mut node = reversed_list;
    while let Some(boxed_node) = node {
        print!("{} ", boxed_node.val);
        node = boxed_node.next;
    }
    // 输出: 3 2 1
}