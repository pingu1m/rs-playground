#![allow(unused)]

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

fn node(n: TreeNode) -> Node {
    Some(Rc::new(RefCell::new(n)))
}

use std::cell::RefCell;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn good_nodes(root: Node) -> i32 {
        fn solve(node: &Node, max: i32) -> Vec<bool> {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    [
                        vec![max > n.val],
                        solve(&n.left, max.max(n.val)),
                        solve(&n.right, max.max(n.val)),
                    ]
                    .concat()
                }
                None => vec![],
            }
        }
        let val = root.clone().unwrap();
        let val = val.borrow().val;
        let res = solve(&root, val);
        dbg!(res.clone());

        res.iter()
            .map(|x| match x {
                true => 0,
                false => 1,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut n1 = TreeNode::new(3);
        let mut n1_left = TreeNode::new(1);
        let n1_left_left = TreeNode::new(3);
        let mut n2 = TreeNode::new(4);
        let n2_left = TreeNode::new(1);
        let n2_right = TreeNode::new(5);

        n2.left = Some(Rc::new(RefCell::new(n2_left)));
        n2.right = Some(Rc::new(RefCell::new(n2_right)));

        n1_left.left = Some(Rc::new(RefCell::new(n1_left_left)));
        n1.left = Some(Rc::new(RefCell::new(n1_left)));
        n1.right = Some(Rc::new(RefCell::new(n2)));

        assert_eq!(Solution::good_nodes(node(n1)), 4);
    }

    #[test]
    fn ex2() {
        let mut n1 = TreeNode::new(3);
        let mut n1_left = TreeNode::new(3);
        let n1_left_left = TreeNode::new(4);

        n1_left.left = Some(Rc::new(RefCell::new(n1_left_left)));
        n1.left = Some(Rc::new(RefCell::new(n1_left)));

        // assert_eq!(Solution::leaf_similar(node(n1), node(t1)), true);
        assert_eq!(Solution::good_nodes(node(n1)), 3);
    }

    #[test]
    fn ex3() {
        let mut n1 = TreeNode::new(1);
        assert_eq!(Solution::good_nodes(node(n1)), 1);
    }
}
