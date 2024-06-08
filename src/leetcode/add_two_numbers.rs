// Definition for singly-linked list.
// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.

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

struct Solution{}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add(l1, l2, 0)
    }

    pub fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, x:i32) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node), None) => Self::one_none(x, node),
            (None, Some(node)) => Self::one_none(x, node),
            (None, None) => {
                if x != 0 {
                    Some(Box::new(ListNode::new(x)))
                } else {
                    None
                }
            },
            (Some(node1), Some(node2)) => {
                let value = node1.val + node2.val + x;
                if value < 10 {
                    let mut cur_node = Box::new(ListNode::new(value));
                    cur_node.next = Self::add(node1.next, node2.next, 0);
                    Some(cur_node)
                } else {
                    let mut cur_node = Box::new(ListNode::new(value % 10));
                    cur_node.next = Self::add(node1.next, node2.next, value / 10);
                    Some(cur_node)
                }
            }
        }
    }

    fn one_none(x: i32, node: Box<ListNode>) -> Option<Box<ListNode>> {
        let value = node.val + x;
        if value < 10 {
            let mut cur_node = Box::new(ListNode::new(value));
            cur_node.next = Self::add(node.next, None, 0);
            Some(cur_node)
        } else {
            let mut cur_node = Box::new(ListNode::new(value % 10));
            cur_node.next = Self::add(node.next, None, value / 10);
            Some(cur_node)
        }
    }

    // pub fn add2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, x:i32) -> Option<Box<ListNode>> {
    //     if !l1.is_none() && !l2.is_none() {
    //         let node1 = l1.unwrap().as_mut();
    //         let node2 = l2.unwrap().as_mut();
    //         let value = node1.val + node2.val + x;
    //         if value < 10 {
    //             let mut cur_node = Box::new(ListNode::new(value));
    //             cur_node.next = Self::add(node1.clone().next, node2.clone().next, 0);
    //             Some(cur_node)
    //         } else {
    //             let mut cur_node = Box::new(ListNode::new(value % 10));
    //             cur_node.next = Self::add(node1.clone().next, node2.clone().next, value / 10);
    //             Some(cur_node)
    //         }
    //     } else if l1.is_none() && l2.is_none() {
    //         if x != 0 {
    //             Some(Box::new(ListNode::new(x)))
    //         } else {
    //             None
    //         }
    //     } else {
    //         let value = node.val + x;
    //         if value < 10 {
    //             let mut cur_node = Box::new(ListNode::new(value));
    //             cur_node.next = Self::add(node.next, None, 0);
    //             Some(cur_node)
    //         } else {
    //             let mut cur_node = Box::new(ListNode::new(value % 10));
    //             cur_node.next = Self::add(node.next, None, value / 10);
    //             Some(cur_node)
    //         }
    //     }
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test1() {
        let mut node1 = ListNode::new(2);
        node1.next = Some(Box::new(ListNode::new(4)));
        node1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        let mut node2 = ListNode::new(5);
        node2.next = Some(Box::new(ListNode::new(6)));
        node2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

        let result = Solution::add_two_numbers(Some(Box::new(node1)), Some(Box::new(node2)));

        
        println!("{:?}", result)
    }
}