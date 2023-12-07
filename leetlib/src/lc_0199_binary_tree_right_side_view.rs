#![allow(unused)]

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
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

impl Solution {
    pub fn right_side_view(root: Node) -> Vec<i32> {
        fn solve(node: Node) -> Vec<i32> {
            let mut queue = VecDeque::new();
            queue.push_back((node, 0));
            let mut res_map: HashMap<i32, Vec<i32>> = HashMap::new();

            while !queue.is_empty() {
                if let Some(n) = queue.pop_front() {
                    let (n, level) = n;
                    if let Some(el) = n {
                        let b = el.borrow();
                        res_map
                            .entry(level)
                            .and_modify(|x| x.push(b.val))
                            .or_insert(vec![b.val]);
                        queue.push_back((b.left.clone(), level + 1));
                        queue.push_back((b.right.clone(), level + 1));
                    }
                }
            }
            let mut t = (0..res_map.len())
                .map(|x| {
                    let x = x as i32;
                    if let Some(el) = res_map.get_mut(&x) {
                        el.pop()
                    } else {
                        None
                    }
                })
                .rev()
                .flatten()
                .collect::<Vec<_>>();
            t
        }
        let mut res = solve(root);
        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = TreeNode::from_string("1,2,3,null,5,null,4".to_string());
        assert_eq!(Solution::right_side_view(input), vec![1, 3, 4]);
    }

    #[test]
    fn ex2() {
        let input = TreeNode::from_string("1,null,3".to_string());
        assert_eq!(Solution::right_side_view(input), vec![1, 3]);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::right_side_view(None), vec![]);
    }
}
