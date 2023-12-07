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
    pub fn path_sum(root: Node, target_sum: i32) -> i32 {
        fn solve(node: &Node, map: &mut HashMap<i64, i32>, current_sum: i64, target: i32) -> i32 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    let mut freq = 0;
                    let current_sum = current_sum + n.val as i64;
                    dbg!(map.clone());

                    if current_sum == target as i64 {
                        freq += 1;
                    }
                    freq += map.get(&(current_sum - target as i64)).unwrap_or(&0);

                    map.entry(current_sum).and_modify(|e| *e += 1).or_insert(1);

                    freq += solve(&n.left, map, current_sum, target)
                        + solve(&n.right, map, current_sum, target);

                    map.entry(current_sum).and_modify(|e| *e -= 1).or_insert(0);
                    freq
                }
                None => 0,
            }
        }
        let mut map: HashMap<i64, i32> = HashMap::new();
        solve(&root, &mut map, 0, target_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = TreeNode::from_string("10,5,-3,3,2,null,11,3,-2,null,1".to_string());
        assert_eq!(Solution::path_sum(input, 8), 3)
    }

    #[test]
    fn ex2() {
        let input = TreeNode::from_string("5,4,8,11,null,13,4,7,2,null,null,5,1".to_string());
        assert_eq!(Solution::path_sum(input, 22), 3)
    }

    #[test]
    fn ex3() {
        let input = TreeNode::from_string("1,-2,-3".to_string());
        assert_eq!(Solution::path_sum(input, -1), 1)
    }

    #[test]
    fn ex4() {
        let input = TreeNode::from_string(
            "1000000000,1000000000,null,294967296,null,1000000000,null,1000000000,null,1000000000"
                .to_string(),
        );
        assert_eq!(Solution::path_sum(input, 0), 0)
    }

    #[test]
    fn create_tree_from_string() {
        let expect = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        let input = "10,5,-3,3,2,null,11,3,-2,null,1".to_string();
        assert_eq!(TreeNode::from_string(input), expect)
    }
}
