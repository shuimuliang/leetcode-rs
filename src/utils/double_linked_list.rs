// use std::collections::HashMap;

#![allow(dead_code)]

use std::rc::Rc;
use std::cell::{RefCell, Ref};

type MaybeNode<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    prev: MaybeNode<T>,
    next: MaybeNode<T>,
}

pub struct LinkedList<T> {
    head: MaybeNode<T>,
    tail: MaybeNode<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            prev: None,
            next: None,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, val: T) {
        let node: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node::new(val)));
        match self.tail.take() {
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
            Some(tail) => {
                tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(tail);
                self.tail = Some(node);
            }
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|node| {
            match node.borrow_mut().prev.take() {
                Some(head) => {
                    head.borrow_mut().next = None;
                    self.tail = Some(head);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
            Rc::try_unwrap(node).ok().unwrap().into_inner().val
        }
        )
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
           Ref::map(node.borrow(), |node| &node.val)
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self)  {
        while self.pop_back().is_some() {
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_linked_list_empty() {}

    #[test]
    fn test_linked_list_head() {}

    #[test]
    fn test_linked_list_pop() {}
}