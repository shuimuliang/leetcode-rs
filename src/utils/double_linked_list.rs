//! A doubly-linked list in stable and safe Rust.

use std::cell::{RefCell};
use std::rc::{Rc, Weak};
use std::fmt::Debug;
// The node type stores the data and two pointers.
//
// It uses Option to represent nullability in safe Rust. It has zero overhead
// over a null pointer due to the NonZero optimization.
//
// It uses an Rc (Reference Counted) pointer to give ownership of the next node
// to the current node. And a Weak (weak Reference Counted) pointer to reference
// the previous node without owning it.
//
// It uses RefCell for interior mutability. It allows mutation through
// shared references.

type MaybePrevNode<K, V> = Option<Weak<RefCell<Node<K, V>>>>;
type MaybeNextNode<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

#[derive(Debug)]
pub struct Node<K: Default+Debug, V: Default+Debug> {
    key: K,
    val: V,
    prev: MaybePrevNode<K, V>,
    next: MaybeNextNode<K, V>,
}

impl<K, V> Default for Node<K, V>
    where K: Default+Debug, V: Default+Debug
{
    fn default() -> Self {
        Self {
            key: K::default(),
            val: V::default(),
            prev: None,
            next: None,
        }
    }
}

impl<K, V> Node<K, V>
    where K: Default+Debug, V: Default+Debug {
    pub fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

impl<K: Default+Debug, V: Default+Debug> Drop for Node<K, V>
{
    fn drop(&mut self)  {
        // println!("drop node: {:?}", self);
    }
}

pub struct List<K: Default+Debug, V: Default+Debug> {
    capacity: usize,
    count: usize,
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> List<K, V>
    where K: Default+Debug, V: Default+Debug
{
    // Constructs an empty list
    pub fn new(capacity: usize) -> Self {
        let head = Rc::new(RefCell::new(Node::default()));
        let tail = Rc::new(RefCell::new(Node::default()));
        (*head).borrow_mut().next = Some(Rc::clone(&tail));
        (*tail).borrow_mut().prev = Some(Rc::downgrade(&head));
        assert!(capacity > 0);
        Self {
            capacity,
            count: 0,
            head: Some(head),
            tail: Some(tail),
        }
    }

    pub fn push_front(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        // 将节点添加到双向链表头部
        // head -> node -> next
        // you need to first call as_ref and unwrap to get a reference to the Rc inside the Option
        // before you can call borrow_mut to get a reference to the Node.

        while self.count >= self.capacity {
            self.pop_back();
        }

        match self.head {
            Some(ref head) => {
                let mut next = head.borrow_mut().next.take();
                // next.prev unlink
                next.as_mut().unwrap().borrow_mut().prev = Some(Rc::downgrade(&node));
                node.borrow_mut().next = next;
                node.borrow_mut().prev = Some(Rc::downgrade(head));
                head.borrow_mut().next = Some(node);
                self.count += 1;
            }
            None => (), // TODO: head not found Err
        }
    }

    // TODO testcase: upgrade 返回Option<Rc<_>>, Weak引用的资源可能不在了

    pub fn pop_back(&mut self) -> Option<Rc<RefCell<Node<K, V>>>>{
        // FIFO  prev -> node -> tail
        match self.tail {
            Some(ref tail) => {
                // tail.prev unlink
                let node = tail.borrow_mut().prev.take().unwrap().upgrade().unwrap();
                let prev = node.borrow_mut().prev.take().unwrap().upgrade().unwrap();
                tail.borrow_mut().prev = Some(Rc::downgrade(&prev));
                prev.borrow_mut().next = Some(tail.clone());
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;
                self.count -= 1;
                return Some(node)
            }
            _ => {
                return None
            }
        }
    }

    pub fn detach(&mut self, node: Rc<RefCell<Node<K, V>>>) -> Rc<RefCell<Node<K, V>>> {
        // 删除双向链表的某个引用节点
        let next = node.borrow_mut().next.take().unwrap();
        let prev = node.borrow_mut().prev.as_ref().unwrap().upgrade().unwrap();
        prev.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;
        self.count -= 1;
        node
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl<K: Default+Debug, V: Default+Debug> Drop for List<K, V>
{
    fn drop(&mut self)  {
        while self.count > 1 {
            self.pop_back();
        }
        drop(&self.head);
        drop(&self.tail);
    }
}



#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::{List, Node};

    #[test]
    fn test_case_empy() {
        let list: List<i32, i32> = List::new(2);
        assert!(list.head.is_some());
        let head_key = list.head.as_ref().unwrap().borrow().key;
        assert_eq!(i32::default(), head_key);

        assert!(list.tail.is_some());
        let tail_key = list.tail.as_ref().unwrap().borrow().key;
        assert_eq!(i32::default(), tail_key);
    }

    #[test]
    fn test_pop_back_2_items() {
        // one exist, one empty
        // push 1-5
        let mut list : List<i32, i32> = List::new(10);
        for x in 0..5 {
            list.push_front(Rc::new(RefCell::new(Node::new(x, x))));
        }
        list.pop_back();
        list.pop_back();
        assert_eq!(3, list.count);
    }

    #[test]
    fn test_pop_exist_item() {
        let mut list : List<i32, i32>= List::new(10);

        let node1 = Rc::new(RefCell::new(Node::new(1, 1)));
        list.push_front(node1.clone());

        // push node 2,3,4
        for i in 2..5 {
            list.push_front(
                Rc::new(RefCell::new(Node::new(i, i)))
            );
        }
        assert_eq!(4, list.count);
        list.detach(node1);
        assert_eq!(3, list.count);
    }

    #[test]
    fn test_push_front() {
        // push 1-5
        let mut list : List<i32, i32> = List::new(10);
        for x in 0..5 {
           list.push_front(Rc::new(RefCell::new(Node::new(x, x))));
        }
        assert_eq!(5, list.count);
    }

    #[test]
    fn test_push_front_1_stackoverflow() {
        // capacity 2, push (1,2,3), left (2,3)
        let mut list : List<i32, i32> = List::new(2);
        for x in 1..=3 {
            list.push_front(Rc::new(RefCell::new(Node::new(x, x))));
        }
        assert_eq!(2, list.count);
    }

    #[test]
    fn test_push_front_2_stackoverflow() {
        // capacity 1, push (1,2,3), left (3)
        let mut list : List<i32, i32> = List::new(1);
        for x in 1..=3 {
            list.push_front(Rc::new(RefCell::new(Node::new(x, x))));
        }
        assert_eq!(1, list.count);
    }

    #[test]
    fn test_drop_success() {
        let mut list : List<i32, i32> = List::new(10);
        for x in 1..2 {
            list.push_front(Rc::new(RefCell::new(Node::new(x, x))));
        }
        assert_eq!(1, list.count());
        drop(&list);
    }
}