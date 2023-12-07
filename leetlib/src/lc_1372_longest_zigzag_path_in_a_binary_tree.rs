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

    pub fn from_string(vals: String) -> Node {
        let mut vals = vals
            .split(',')
            .map(|x| {
                if x == "null" {
                    None
                } else {
                    x.parse::<i64>().ok()
                }
            })
            .collect::<Vec<_>>();

        fn helper(values: &[Option<i64>], i: usize) -> Node {
            if values.is_empty() {
                return None;
            }
            if i >= values.len() {
                return None;
            }

            let first = &values[i];

            dbg!(first, i);
            let rr = "1,null,1,1,1,null,null,1,1,null,1,null,null,null,1";

            if let Some(value) = first {
                let l = 2 * i + 1;
                let r = 2 * i + 2;
                dbg!(rr);
                dbg!(l, r);
                let left = helper(values, l);
                let right = helper(values, r);

                Some(Rc::new(RefCell::new(TreeNode {
                    val: *value as i32,
                    left,
                    right,
                })))
            } else {
                None
            }
        }
        let root = helper(&vals, 0);

        let _a = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));

        root
    }
}
struct Solution;

fn node(n: TreeNode) -> Node {
    Some(Rc::new(RefCell::new(n)))
}

use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn longest_zig_zag(root: Node) -> i32 {
        fn depth_first_search(node: &Node, l_count: i32, r_count: i32) -> i32 {
            match &node {
                None => l_count.max(r_count) - 1,
                Some(n) => {
                    let b = n.borrow();
                    depth_first_search(&b.left, r_count + 1, 0).max(depth_first_search(
                        &b.right,
                        0,
                        l_count + 1,
                    ))
                }
            }
        }
        depth_first_search(&root, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input =
            TreeNode::from_string("1,null,1,1,1,null,null,1,1,null,1,null,null,null,1".to_string());

        dbg![input.clone()];
        assert_eq!(Solution::longest_zig_zag(input), 3)
    }

    #[test]
    fn ex2() {
        let input = TreeNode::from_string("1,1,1,null,1,null,null,1,1,null,1".to_string());
        assert_eq!(Solution::longest_zig_zag(input), 4)
    }

    #[test]
    fn ex3() {
        let input = TreeNode::from_string("1".to_string());
        assert_eq!(Solution::longest_zig_zag(input), 0)
    }
}
