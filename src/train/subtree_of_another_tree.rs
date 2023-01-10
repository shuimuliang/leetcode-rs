// Definition for a binary tree node.

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

struct Solution;

// https://leetcode.com/problems/subtree-of-another-tree/
// https://leetcode.com/problems/find-duplicate-subtrees/
// https://leetcode.com/problems/find-duplicate-subtrees/solutions/106030/Python-O(N)-Merkle-Hashing-Approach/
// https://github.com/filecoin-project/merkletree/blob/master/src/merkle.rs
// - build_tree (items) -> tree
// - get_root -> hash
// - gen_proof -> proof
// - validate_proof (proof, leaf, root) -> bool

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;
type MaybeMerKleTree = Option<Rc<RefCell<MerkleTree>>>;

struct MerkleTree {
    pub digest: Digest,
    pub left: MaybeMerKleTree,
    pub right: MaybeMerKleTree,
}

impl From<&MaybeNode> for MerkleTree {
    fn from(root: &MaybeNode) -> Self {
        let node = match root.as_ref() {
            Some(node) => {
                let t = node.borrow();
                let mut context = Context::new();
                let digest = compute(t.val.to_ne_bytes());
                if t.left.is_some() && t.right.is_some() {
                    let left = MerkleTree::from(&t.left);
                    let right = MerkleTree::from(&t.right);

                    context.write(left.digest.as_ref()).unwrap();
                    context.write(digest.as_ref()).unwrap();
                    context.write(right.digest.as_ref()).unwrap();

                    Self {
                        digest: context.compute(),
                        left: Some(Rc::new(RefCell::new(left))),
                        right: Some(Rc::new(RefCell::new(right))),
                    }
                } else if t.left.is_some() && t.right.is_none() {
                    let left = MerkleTree::from(&t.left);

                    context.write(left.digest.as_ref()).unwrap();
                    context.write(digest.as_ref()).unwrap();
                    context.write(Digest::default().as_ref()).unwrap();

                    Self {
                        digest: context.compute(),
                        left: Some(Rc::new(RefCell::new(left))),
                        right: None,
                    }
                } else if t.left.is_none() && t.right.is_some() {
                    let right = MerkleTree::from(&t.right);

                    context.write(Digest::default().as_ref()).unwrap();
                    context.write(digest.as_ref()).unwrap();
                    context.write(right.digest.as_ref()).unwrap();

                    Self {
                        digest: context.compute(),
                        left: None,
                        right: Some(Rc::new(RefCell::new(right))),
                    }
                } else {
                    context.write(Digest::default().as_ref()).unwrap();
                    context.write(digest.as_ref()).unwrap();
                    context.write(Digest::default().as_ref()).unwrap();
                    Self {
                        digest: context.compute(),
                        left: None,
                        right: None,
                    }
                }
            }
            None => Self {
                digest: Digest::default(),
                left: None,
                right: None,
            },
        };
        node
    }
}

impl MerkleTree {
    #[inline]
    pub fn new(digest: Digest) -> Self {
        MerkleTree {
            digest,
            left: None,
            right: None,
        }
    }

    pub fn validate_proof(&self, digest: Digest) -> bool {
        if self.digest == digest {
            return true;
        }
        let hit_left = match self.left {
            Some(ref child) => child.borrow().validate_proof(digest),
            None => false,
        };
        if hit_left {
            return true;
        }
        let hit_right = match self.right {
            Some(ref child) => child.borrow().validate_proof(digest),
            None => false,
        };
        if hit_right {
            return true;
        }
        false
    }
}

use crate::train::md5::{compute, Context, Digest};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let m1 = MerkleTree::from(&root);
        let m2 = MerkleTree::from(&sub_root);
        m1.validate_proof(m2.digest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_case_1() {
        // [3,4,5,1,2], [4,1,2]

        fn build_root() -> MaybeNode {
            let mut t1 = TreeNode::new(3);
            let mut t2 = TreeNode::new(4);
            let t3 = TreeNode::new(5);
            let t4 = TreeNode::new(1);
            let t5 = TreeNode::new(2);
            t2.left = Some(Rc::new(RefCell::new(t4)));
            t2.right = Some(Rc::new(RefCell::new(t5)));
            t1.left = Some(Rc::new(RefCell::new(t2)));
            t1.right = Some(Rc::new(RefCell::new(t3)));
            let tree = Some(Rc::new(RefCell::new(t1)));
            tree
        }

        fn build_sub_root() -> MaybeNode {
            let mut t2 = TreeNode::new(4);
            let t4 = TreeNode::new(1);
            let t5 = TreeNode::new(2);
            t2.left = Some(Rc::new(RefCell::new(t4)));
            t2.left = Some(Rc::new(RefCell::new(t5)));
            let tree = Some(Rc::new(RefCell::new(t2)));
            tree
        }

        let root = build_root();
        let sub_root = build_sub_root();

        assert_eq!(true, Solution::is_subtree(root, sub_root));
    }

    #[test]
    fn test_case_2() {
        // [3,4,5,1,2,null,null,null,null,0]
        // [4,1,2]
    }

    #[test]
    fn test_from_empty_node() {
        let tree = None;
        let merkle_tree = MerkleTree::from(&tree);
        assert_eq!(Digest::default(), merkle_tree.digest);
    }
}
