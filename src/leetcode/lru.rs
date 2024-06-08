use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct ListNode {
    pub key: i32,
    pub val: i32,
    pub prev: Option<Weak<RefCell<ListNode>>>,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(key: i32, val: i32) -> Self {
        ListNode {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LRUCache {
    capacity: i32,
    count: i32,
    map: HashMap<i32,Rc<RefCell<ListNode>>>,
    head: Rc<RefCell<ListNode>>,
    tail: Rc<RefCell<ListNode>>,
}


impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(ListNode::new(999, 999)));
        let tail = Rc::clone(&head);
        LRUCache {
            capacity,
            count: 0,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        // 获取节点的引用并解除对self的不可变借用
        if let Some(node) = self.map.get(&key).cloned() {
            // 在新作用域中可变借用self
            self.move_to_head(node.clone());
            return node.borrow().val;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        let node = Rc::new(RefCell::new(ListNode::new(key, value)));
        // 如果put的key已经存在，那么直接move_to_head
        if let Some(exist_node) = self.map.get(&key) {
            exist_node.borrow_mut().val = value;
            self.move_to_head(Rc::clone(exist_node));
        } else {
            if self.count >= self.capacity {
                let key_temp = self.remove_last();
                self.map.remove(&key_temp);
            }
            self.add_to_head(Rc::clone(&node));
            self.map.insert(key, node);
        }
    }

    fn add_to_head(&mut self, node_rc: Rc<RefCell<ListNode>>) {
        //let node_rc = Rc::new(RefCell::new(node));
        if Rc::ptr_eq(&self.head, &self.tail) {
            node_rc.borrow_mut().prev = Some(Rc::downgrade(&self.head));
            self.head.borrow_mut().next = Some(Rc::clone(&node_rc));
            self.tail = Rc::clone(&node_rc);

        } else {
            let next_node = self.head.borrow_mut().next.take().unwrap();
            node_rc.borrow_mut().prev = Some(Rc::downgrade(&self.head));
            node_rc.borrow_mut().next = Some(Rc::clone(&next_node));
            next_node.borrow_mut().prev = Some(Rc::downgrade(&node_rc));
            self.head.borrow_mut().next = Some(Rc::clone(&node_rc));
        }
        self.count += 1;
    }

    fn remove_last(&mut self) -> i32 {
        let key = self.tail.borrow().key;
        let pre_node = self.tail.borrow_mut().prev.take().unwrap();
        let pre_rc = pre_node.upgrade().unwrap();
        self.tail.borrow_mut().prev = None;
        pre_rc.borrow_mut().next = None;
        self.tail = pre_rc;
        key
    }


    fn move_to_head(&mut self, node_rc: Rc<RefCell<ListNode>>) {
        let pre_node = node_rc.borrow_mut().prev.as_ref().unwrap().upgrade().unwrap();
        // 如果移动的是head后的节点，则不用动
        if Rc::ptr_eq(&self.head, &pre_node) {
            return;
        }

        let next_node_op = node_rc.borrow_mut().next.take();
        let head_next_node = self.head.borrow_mut().next.take().unwrap();
        // 把这个节点移动到head后
        node_rc.borrow_mut().prev = Some(Rc::downgrade(&self.head));
        node_rc.borrow_mut().next = Some(Rc::clone(&head_next_node));
        head_next_node.borrow_mut().prev = Some(Rc::downgrade(&node_rc));
        self.head.borrow_mut().next = Some(Rc::clone(&node_rc));

        // 如果移动的是tail
        if Rc::ptr_eq(&self.tail, &node_rc) {
            pre_node.borrow_mut().next = None;
            self.tail = pre_node;
        } else {
            let next_node = next_node_op.unwrap();
            pre_node.borrow_mut().next = Some(Rc::clone(&next_node));
            next_node.borrow_mut().prev = Some(Rc::downgrade(&pre_node));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test1() {
        // let mut lru = LRUCache::new(2);
        // lru.put(2,1);
        // lru.put(1,1);
        // lru.put(2,3);
        // lru.put(4,1);
        // println!("{:?}", lru.head);
        // lru.get(1);
        // lru.get(2);
        // println!("{:?}", lru.head);


        let mut lru = LRUCache::new(1);
        lru.put(2,1);
        println!("{:?}", lru.head);
        lru.get(2);
        println!("{:?}", lru.head);
        lru.put(3,2);
        println!("{:?}", lru.head);


    }
}