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
    pub fn reverse_list(head: Node) -> Node {
        let (mut prev, mut curr) = (None, head);
        while let Some(mut node) = curr {
            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

fn reverse(head: Node, mut prev: Node) -> Node {
    if let Some(mut curr) = head {
        let temp = curr.next.take();
        curr.next = prev;
        prev = Some(curr);
        return reverse(temp, prev);
    };
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut n1 = ListNode::new(1);
        let mut n2 = ListNode::new(2);
        let mut n3 = ListNode::new(3);
        let mut n4 = ListNode::new(4);
        let mut n5 = ListNode::new(5);
        n4.next = Some(Box::new(n5));
        n3.next = Some(Box::new(n4));
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));

        let mut t1 = ListNode::new(5);
        let mut t2 = ListNode::new(4);
        let mut t3 = ListNode::new(3);
        let mut t4 = ListNode::new(2);
        let mut t5 = ListNode::new(1);
        t4.next = Some(Box::new(t5));
        t3.next = Some(Box::new(t4));
        t2.next = Some(Box::new(t3));
        t1.next = Some(Box::new(t2));

        assert_eq!(
            Solution::reverse_list(Some(Box::new(n1))),
            Some(Box::new(t1))
        );
    }

    #[test]
    fn ex2() {
        let mut n1 = ListNode::new(1);
        let mut n2 = ListNode::new(2);
        n1.next = Some(Box::new(n2));

        let mut t1 = ListNode::new(2);
        let mut t2 = ListNode::new(1);
        t1.next = Some(Box::new(t2));

        assert_eq!(
            Solution::reverse_list(Some(Box::new(n1))),
            Some(Box::new(t1))
        );
    }
    #[test]
    fn ex3() {
        assert_eq!(Solution::reverse_list(None), None);
    }
}
