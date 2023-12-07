#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

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

type Node = Option<Rc<RefCell<TreeNode>>>;

struct Solution;
impl Solution {
    fn recurse_depth(mut node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        match node {
            Some(node_rc) => {
                let mut node_ref = node_rc.borrow_mut();
                (Self::recurse_depth(node_ref.left.take(), depth + 1))
                    .max(Self::recurse_depth(node_ref.right.take(), depth + 1))
            }
            None => depth,
        }
    }

    pub fn max_depth(root: Node) -> i32 {
        // Self::recurse_depth(root, 0)
        fn solve(root: &Node) -> i32 {
            match root {
                Some(node) => {
                    let node = node.borrow();
                    1 + solve(&node.left).max(solve(&node.right))
                }
                None => 0,
            }
        }
        solve(&root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut n1 = TreeNode::new(3);
        let n2 = TreeNode::new(9);
        let mut n3 = TreeNode::new(20);
        let n4 = TreeNode::new(15);
        let n5 = TreeNode::new(7);
        n3.left = Some(Rc::new(RefCell::new(n4)));
        n3.right = Some(Rc::new(RefCell::new(n5)));
        n1.left = Some(Rc::new(RefCell::new(n2)));
        n1.right = Some(Rc::new(RefCell::new(n3)));

        assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(n1)))), 3);
    }

    #[test]
    fn ex2() {
        let mut n1 = TreeNode::new(1);
        let n2 = TreeNode::new(2);
        n1.right = Some(Rc::new(RefCell::new(n2)));

        assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(n1)))), 2);
    }
}
