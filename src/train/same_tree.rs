// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn is_same_tree(p: MaybeNode, q: MaybeNode) -> bool {
        // iter root node in pre order
        let is_the_same = match (p, q) {
            (Some(v1), Some(v2)) if v1.borrow().val == v2.borrow().val => {
                Self::is_same_tree(v1.borrow().left.clone(), v2.borrow().left.clone()) &&
                    Self::is_same_tree(v1.borrow().right.clone(), v2.borrow().right.clone())
            },
            (None, None) => true,
            _ => false,
            // (Some(v1), None) => false,
            // (None, Some(v2)) => false,
        };
        is_the_same
    }
}

// merkle tree
// impl Solution {
//
// }

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::TreeNode;
    use super::Solution;

    #[test]
    fn case_0() {
        let mut t1_node_1 = TreeNode::new(1);
        let t1_node_2 = TreeNode::new(2);
        let t1_node_3 = TreeNode::new(3);
        t1_node_1.left = Some(Rc::new(RefCell::new(t1_node_2)));
        t1_node_1.right = Some(Rc::new(RefCell::new(t1_node_3)));
        let root1 = Some(Rc::new(RefCell::new(t1_node_1)));

        let mut t2_node_1 = TreeNode::new(1);
        let t2_node_2 = TreeNode::new(2);
        let t2_node_3 = TreeNode::new(3);
        t2_node_1.left = Some(Rc::new(RefCell::new(t2_node_2)));
        t2_node_1.right = Some(Rc::new(RefCell::new(t2_node_3)));
        let root2 = Some(Rc::new(RefCell::new(t2_node_1)));

        let be_same_tree = Solution::is_same_tree(root1, root2);
        assert_eq!(true, be_same_tree);
    }

    #[test]
    fn case_1() {
        let mut t1_node_1 = TreeNode::new(1);
        let t1_node_l = TreeNode::new(2);
        let t1_node_r = TreeNode::new(1);
        t1_node_1.left = Some(Rc::new(RefCell::new(t1_node_l)));
        t1_node_1.right = Some(Rc::new(RefCell::new(t1_node_r)));
        let root1 = Some(Rc::new(RefCell::new(t1_node_1)));

        let mut t2_node_1 = TreeNode::new(1);
        let t2_node_l = TreeNode::new(1);
        let t2_node_r = TreeNode::new(2);
        t2_node_1.left = Some(Rc::new(RefCell::new(t2_node_l)));
        t2_node_1.right = Some(Rc::new(RefCell::new(t2_node_r)));
        let root2 = Some(Rc::new(RefCell::new(t2_node_1)));

        let be_same_tree = Solution::is_same_tree(root1, root2);
        assert_eq!(false, be_same_tree);
    }
}
