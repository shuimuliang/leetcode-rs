// https://leetcode.com/problems/binary-search-tree-iterator/
// https://rust-unofficial.github.io/too-many-lists/
// https://course.rs/advance/smart-pointer/box.html smart pointer
// https://doc.rust-lang.org/std/option/enum.Option.html#method.and

use std::cell::{Cell, RefCell};
use std::rc::Rc;

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;
type IterRes = i32;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: MaybeNode,
    pub right: MaybeNode,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct BSTIterator {
    // root: MaybeNode,
    results: Vec<IterRes>,
    index: Cell<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * middle order
 */
impl BSTIterator {
    fn new(root: MaybeNode) -> Self {
        fn dfs(root: MaybeNode, results: &mut Vec<IterRes>) {
            if let Some(x) = root {
                dfs(x.borrow().left.clone(), results);
                results.push(x.borrow().val);
                dfs(x.borrow().right.clone(), results);
            }
        }
        let mut results: Vec<IterRes> = Vec::new();
        dfs(root, &mut results);
        BSTIterator {
            results,
            index: Cell::new(0),
        }
    }

    fn next(&self) -> i32 {
        let pos = self.index.get();
        self.index.set(pos + 1);
        self.results[pos]
    }

    fn has_next(&self) -> bool {
        self.index.get() < self.results.len()
    }
}

// Your BSTIterator object will be instantiated and called as such:
// let obj = BSTIterator::new(root);
// let ret_1: i32 = obj.next();
// let ret_2: bool = obj.has_next();

#[cfg(test)]
mod tests {
    use super::BSTIterator;
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn case_0() {
        let mut res: i32;
        let mut has_next: bool;

        let mut node_7 = TreeNode::new(7);
        let node_3 = TreeNode::new(3);
        let mut node_15 = TreeNode::new(15);
        let node_9 = TreeNode::new(9);
        let node_20 = TreeNode::new(20);
        node_7.left = Some(Rc::new(RefCell::new(node_3)));
        node_15.left = Some(Rc::new(RefCell::new(node_9)));
        node_15.right = Some(Rc::new(RefCell::new(node_20)));
        node_7.right = Some(Rc::new(RefCell::new(node_15)));
        let root = Some(Rc::new(RefCell::new(node_7)));

        let bst_iterator: BSTIterator = BSTIterator::new(root);

        res = bst_iterator.next(); // return 3
        assert_eq!(res, 3);
        res = bst_iterator.next(); // return 7
        assert_eq!(res, 7);
        has_next = bst_iterator.has_next(); // return True
        assert_eq!(has_next, true);
        res = bst_iterator.next(); // return 9
        assert_eq!(res, 9);
        has_next = bst_iterator.has_next(); // return True
        assert_eq!(has_next, true);
        res = bst_iterator.next(); // return 15
        assert_eq!(res, 15);
        has_next = bst_iterator.has_next(); // return True
        assert_eq!(has_next, true);
        res = bst_iterator.next(); // return 20
        assert_eq!(res, 20);
        has_next = bst_iterator.has_next(); // return False
        assert_eq!(has_next, false);
    }
}
