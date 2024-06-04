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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1 == None {
            return list2;
        } else if list2 == None {
            return list1;
        }

        let node1 = list1.unwrap();
        let node2 = list2.unwrap();

        if node1.val <= node2.val {
            let mut node = ListNode::new(node1.val);
            if let Some(n) = node1.next {
                node.next = Self::merge_two_lists(Some(n), Some(node2));
            } else {
                node.next = Self::merge_two_lists(None, Some(node2));
            }
            return Some(Box::new(node));
        } else {
            let mut node = ListNode::new(node2.val);
            if let Some(n) = node2.next {
                node.next = Self::merge_two_lists(Some(node1), Some(n));
            } else {
                node.next = Self::merge_two_lists(Some(node1), None);
            }
            return Some(Box::new(node));
        }
    }

    pub fn merge_two_lists2(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    let n = node1.next;
                    node1.next = Solution::merge_two_lists(n, Some(node2));
                    Some(node1)
                }
                else {
                    let n = node2.next;
                    node2.next = Solution::merge_two_lists(Some(node1), n);
                    Some(node2)
                }
            },
            _ => None
        }
    }

    pub fn merge_two_lists3(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }
        let mut res = None;
        let mut p1 = &list1;
        let mut p2 = &list2;
        if p1.as_ref().unwrap().val < p2.as_ref().unwrap().val {
            res = p1.clone();
            p1 = &p1.as_ref().unwrap().next;
        } else {
            res = p2.clone();
            p2 = &p2.as_ref().unwrap().next;
        }
        let mut pres = res.as_mut().unwrap();
        loop {
            if p1.is_none() {
                pres.next = p2.clone();
                break;
            }
            if p2.is_none() {
                pres.next = p1.clone();
                break;
            }
            if p1.as_ref().unwrap().val < p2.as_ref().unwrap().val {
                pres.next = p1.clone();
                p1 = &p1.as_ref().unwrap().next;
            } else {
                pres.next = p2.clone();
                p2 = &p2.as_ref().unwrap().next;
            }
            pres = pres.next.as_mut().unwrap();
        }
        res
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test1() {
        println!("123")
    }
}