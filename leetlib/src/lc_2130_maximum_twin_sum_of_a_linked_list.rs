#![allow(unused, dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub struct Solution;

type Node = Option<Box<ListNode>>;
impl Solution {
    pub fn pair_sum(mut head: Node) -> i32 {
        let mut max = i32::MIN;
        let mut nums = vec![];
        while let Some(node) = head {
            nums.push(node.val);
            head = node.next;
        }
        for i in 0..(nums.len() / 2) {
            max = max.max(nums[i] + nums[nums.len() - i - 1])
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut n1 = ListNode::new(5);
        let mut n2 = ListNode::new(4);
        let mut n3 = ListNode::new(2);
        let mut n4 = ListNode::new(1);
        n3.next = Some(Box::new(n4));
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));

        assert_eq!(Solution::pair_sum(Some(Box::new(n1))), 6);
    }

    #[test]
    fn ex2() {
        let mut n1 = ListNode::new(4);
        let mut n2 = ListNode::new(2);
        let mut n3 = ListNode::new(2);
        let mut n4 = ListNode::new(3);
        n3.next = Some(Box::new(n4));
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));

        assert_eq!(Solution::pair_sum(Some(Box::new(n1))), 7);
    }
    #[test]
    fn ex3() {
        let mut n1 = ListNode::new(1);
        let mut n2 = ListNode::new(100000);
        n1.next = Some(Box::new(n2));

        assert_eq!(Solution::pair_sum(Some(Box::new(n1))), 100001);
    }
}
