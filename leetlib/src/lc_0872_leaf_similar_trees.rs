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

fn node(n: TreeNode) -> Node {
    Some(Rc::new(RefCell::new(n)))
}

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

    pub fn leaf_similar(root1: Node, root2: Node) -> bool {
        fn solve(node: &Node, acc: &mut Vec<i32>) {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    if n.left.is_none() && n.right.is_none() {
                        acc.push(n.val);
                    }
                    solve(&n.left, acc);
                    solve(&n.right, acc);
                }
                None => (),
            }
        }
        let mut v1: Vec<i32> = vec![];
        let mut v2: Vec<i32> = vec![];
        solve(&root1, &mut v1);
        solve(&root2, &mut v2);

        v1 == v2
    }

    pub fn leaf_similar_2nd(root1: Node, root2: Node) -> bool {
        fn solve(node: &Node, acc: Vec<i32>) -> Vec<i32> {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    if n.left.is_none() && n.right.is_none() {
                        return [vec![n.val], acc].concat();
                    } else {
                        return [solve(&n.left, acc.clone()), solve(&n.right, acc)].concat();
                    }
                }
                None => {
                    return acc;
                }
            }
        }
        solve(&root1, vec![]) == solve(&root2, vec![])
    }

    pub fn leaf_similar_3nd(root1: Node, root2: Node) -> bool {
        fn solve(node: &Node) -> Vec<i32> {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    if n.left.is_none() && n.right.is_none() {
                        return [vec![n.val], solve(&n.left), solve(&n.right)].concat();
                    }
                    [solve(&n.left), solve(&n.right)].concat()
                }
                None => vec![],
            }
        }
        solve(&root1) == solve(&root2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut n1 = TreeNode::new(2);
        let n2 = TreeNode::new(9);
        let mut n3 = TreeNode::new(19);
        let n4 = TreeNode::new(15);
        let n5 = TreeNode::new(7);
        n3.left = Some(Rc::new(RefCell::new(n4)));
        n3.right = Some(Rc::new(RefCell::new(n5)));
        n1.left = Some(Rc::new(RefCell::new(n2)));
        n1.right = Some(Rc::new(RefCell::new(n3)));

        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        // assert_eq!(Solution::leaf_similar(node(n1), node(t1)), true);
        assert_eq!(Solution::leaf_similar_3nd(node(n1), node(t1)), true);
    }

    #[test]
    fn ex2() {
        let mut n1 = TreeNode::new(1);
        let n2 = TreeNode::new(2);
        n1.right = Some(Rc::new(RefCell::new(n2)));

        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(5);
        t1.left = Some(Rc::new(RefCell::new(t2)));

        // assert_eq!(Solution::leaf_similar(node(n1), node(t1)), false);
        assert_eq!(Solution::leaf_similar_3nd(node(n1), node(t1)), false);
    }
}
