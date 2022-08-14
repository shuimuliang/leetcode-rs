/// simple LRU Cache
/// https://leetcode.com/problems/lru-cache/
/// Doubly-linked list + hashmap, O(1) complexity

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt::Debug;
use std::collections::HashMap;

type MaybePrevNode<K, V> = Option<Weak<RefCell<Node<K, V>>>>;
type MaybeNextNode<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

#[derive(Debug)]
pub struct Node<K: Default + Debug, V: Default + Debug> {
    key: K,
    val: V,
    prev: MaybePrevNode<K, V>,
    next: MaybeNextNode<K, V>,
}

impl<K, V> Default for Node<K, V>
    where K: Default + Debug, V: Default + Debug
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
    where K: Default + Debug, V: Default + Debug {
    pub fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

impl<K: Default + Debug, V: Default + Debug> Drop for Node<K, V>
{
    fn drop(&mut self) {
        // println!("drop node: {:?}", self);
    }
}

pub struct List<K: Default + Debug, V: Default + Debug> {
    count: usize,
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> List<K, V>
    where K: Default + Debug, V: Default + Debug
{
    pub fn new(capacity: usize) -> Self {
        let head = Rc::new(RefCell::new(Node::default()));
        let tail = Rc::new(RefCell::new(Node::default()));
        (*head).borrow_mut().next = Some(Rc::clone(&tail));
        (*tail).borrow_mut().prev = Some(Rc::downgrade(&head));
        assert!(capacity > 0);
        Self {
            count: 0,
            head: Some(head),
            tail: Some(tail),
        }
    }

    pub fn push_front(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        match self.head {
            Some(ref head) => {
                let mut next = head.borrow_mut().next.take();
                next.as_mut().unwrap().borrow_mut().prev = Some(Rc::downgrade(&node));
                node.borrow_mut().next = next;
                node.borrow_mut().prev = Some(Rc::downgrade(head));
                head.borrow_mut().next = Some(node);
                self.count += 1;
            }
            None => ()
        }
    }

    pub fn pop_back(&mut self) -> Option<Rc<RefCell<Node<K, V>>>> {
        match self.tail {
            Some(ref tail) => {
                let node = tail.borrow_mut().prev.take().unwrap().upgrade().unwrap();
                let prev = node.borrow_mut().prev.take().unwrap().upgrade().unwrap();
                tail.borrow_mut().prev = Some(Rc::downgrade(&prev));
                prev.borrow_mut().next = Some(tail.clone());
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;
                self.count -= 1;
                Some(node)
            }
            _ => {
                None
            }
        }
    }

    pub fn detach(&mut self, node: Rc<RefCell<Node<K, V>>>) -> Rc<RefCell<Node<K, V>>> {
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

impl<K: Default + Debug, V: Default + Debug> Drop for List<K, V>
{
    fn drop(&mut self) {
        while self.count > 1 {
            self.pop_back();
        }
        drop(&self.head);
        drop(&self.tail);
    }
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, MaybeNextNode<i32, i32>>,
    list: List<i32, i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            list: List::new(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        self.get_node(key).0
    }

    fn get_node(&mut self, key: i32) -> (i32, MaybeNextNode<i32, i32>) {
        let (mut ret, mut ret_node) = (-1, None);
        let maybe_node = self.map.get(&key);
        match maybe_node {
            None => (),
            Some(x) => {
                let node = self.list.detach(Rc::clone(x.as_ref().unwrap()));
                ret = node.borrow().val;
                ret_node = Some(Rc::clone(&node));
                self.list.push_front(node);
            }
        }
        (ret, ret_node)
    }

    fn put(&mut self, key: i32, value: i32) {
        let (ret, ret_node) = self.get_node(key);
        if ret != -1 {
            ret_node.unwrap().borrow_mut().val = value;
        } else {
            if self.list.count() >= self.capacity {
                let pop_node = self.list.pop_back();
                self.map.remove(&pop_node.unwrap().borrow().key);
            }
            let node = Rc::new(RefCell::new(Node::new(key, value)));
            self.map.insert(key, Some(node.clone()));
            self.list.push_front(node);
        }

    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[ignore]
    #[test]
    fn test_case_1() {
        let mut ret: i32;
        let mut cache: LRUCache = LRUCache::new(2);
        cache.put(1, 1); // cache is {1=1}
        cache.put(2, 2); // cache is {1=1, 2=2}
        ret = cache.get(1); // return 1
        assert_eq!(1, ret);

        cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        ret = cache.get(2); // returns -1 (not found)
        assert_eq!(-1, ret);

        cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        ret = cache.get(1); // return -1 (not found)
        assert_eq!(-1, ret);

        ret = cache.get(3); // return 3
        assert_eq!(3, ret);
        ret = cache.get(4); // return 4
        assert_eq!(4, ret);
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let mut ret: i32;
        let mut cache: LRUCache = LRUCache::new(2);
        cache.put(1, 0); // cache is {1=0}
        cache.put(2, 2); // cache is {1=0, 2=2}
        ret = cache.get(1); // return 0, cache is {2=2, 1=0}
        assert_eq!(0, ret);

        cache.put(3, 3); // cache is {1=0, 3=3}
        ret = cache.get(2); // returns -1 (not found)
        assert_eq!(-1, ret);

        cache.put(4, 4); // cache is {4=4, 3=3}
        ret = cache.get(1); // return -1 (not found)
        assert_eq!(-1, ret);

        ret = cache.get(3); // return 3
        assert_eq!(3, ret);
        ret = cache.get(4); // return 4
        assert_eq!(4, ret);
    }
}
