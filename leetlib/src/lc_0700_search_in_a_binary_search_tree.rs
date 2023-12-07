#![allow(unused)]

use std::cell::RefCell;
use std::ops::Deref;
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

    pub fn pretty_print_tree2(root: Option<Rc<RefCell<TreeNode>>>) {
        fn display_tree(node: &Option<Rc<RefCell<TreeNode>>>, prefix: &str, is_left: bool) {
            if let Some(n) = node {
                let borrowed = n.borrow();

                let node_type = if is_left { "├──" } else { "└──" };

                println!("{}{}{}", prefix, node_type, borrowed.val);

                let new_prefix = if is_left {
                    format!("{}│   ", prefix)
                } else {
                    format!("{}    ", prefix)
                };

                display_tree(&borrowed.left, &new_prefix, true);
                display_tree(&borrowed.right, &new_prefix, false);
            }
        }

        display_tree(&root, "", false);
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

            if let Some(value) = first {
                let left = helper(values, 2 * i + 1);
                let right = helper(values, 2 * i + 2);

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

type Node = Option<Rc<RefCell<TreeNode>>>;
use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn search_bst(root: Node, val: i32) -> Node {
        fn solve(node: &Node, v: i32) -> (Node, bool) {
            match node {
                Some(el) => {
                    let b = el.borrow();
                    let l = solve(&b.left, v);
                    let r = solve(&b.right, v);
                    if l.1 {
                        return l;
                    }
                    if r.1 {
                        return r;
                    }

                    if b.val == v {
                        (node.clone(), true)
                    } else {
                        (None, false)
                    }
                }
                None => (None, false),
            }
        }
        solve(&root, val).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = TreeNode::from_string("4,2,7,1,3".to_string());
        let res = TreeNode::from_string("2,1,3".to_string());
        assert_eq!(Solution::search_bst(input, 2), res);
    }

    #[test]
    fn ex2() {
        let input = TreeNode::from_string("4,2,7,1,3".to_string());
        assert_eq!(Solution::search_bst(input, 5), None);
    }
}
