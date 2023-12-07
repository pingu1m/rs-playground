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

    pub fn pretty_print_tree(root: Option<Rc<RefCell<TreeNode>>>) {
        fn gather_tree_info(
            node: &Option<Rc<RefCell<TreeNode>>>,
            level: usize,
            position: isize,
            tree_info: &mut Vec<Vec<(i32, isize)>>,
        ) {
            if let Some(n) = node {
                let borrowed = n.borrow();
                if level >= tree_info.len() {
                    tree_info.push(vec![]);
                }
                tree_info[level].push((borrowed.val, position));

                gather_tree_info(&borrowed.left, level + 1, position * 2 - 1, tree_info);
                gather_tree_info(&borrowed.right, level + 1, position * 2, tree_info);
            }
        }

        fn print_tree_structure(tree_info: &Vec<Vec<(i32, isize)>>) {
            for level in tree_info.iter() {
                for (value, position) in level.iter() {
                    print!("{:^4}", value);
                    for _ in 0..(position.trailing_zeros() as usize) {
                        print!("----");
                    }
                }
                println!();
            }
        }
        let mut tree_info = Vec::new();
        gather_tree_info(&root, 0, 1, &mut tree_info);
        print_tree_structure(&tree_info);
    }

    // Function to pretty print the binary tree
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

use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        fn solve(node: &Node, p: i32, q: i32) -> (Node, i32) {
            match node {
                None => (None, 0),
                Some(n) => {
                    let b = n.borrow();

                    let l = solve(&b.left, p, q);
                    let r = solve(&b.right, p, q);
                    if l.0.is_some() {
                        return l;
                    }
                    if r.0.is_some() {
                        return r;
                    }

                    let count = if b.val == p || b.val == q { 1 } else { 0 } + l.1 + r.1;

                    if count == 2 {
                        (Some(Rc::clone(n)), count)
                    } else {
                        (None, count)
                    }
                }
            }
        }
        let q = q.unwrap().borrow().val;
        let p = p.unwrap().borrow().val;
        solve(&root, p, q).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = TreeNode::from_string("3,5,1,6,2,0,8,null,null,7,4".to_string());
        let p = node(TreeNode::new(5));
        let q = node(TreeNode::new(1));
        assert_eq!(Solution::lowest_common_ancestor(input.clone(), p, q), input)
    }

    #[test]
    fn ex2() {
        let input = TreeNode::from_string("3,5,1,6,2,0,8,null,null,7,4".to_string());
        let p = node(TreeNode::new(5));
        let q = node(TreeNode::new(4));
        TreeNode::pretty_print_tree2(input.clone());
        assert_eq!(
            Solution::lowest_common_ancestor(input, p, q),
            node(TreeNode::new(5))
        )
    }

    #[test]
    fn ex3() {
        let input = TreeNode::from_string("1,2".to_string());
        let p = node(TreeNode::new(1));
        let q = node(TreeNode::new(2));
        TreeNode::pretty_print_tree2(input.clone());
        assert_eq!(Solution::lowest_common_ancestor(input.clone(), p, q), input)
    }
}
