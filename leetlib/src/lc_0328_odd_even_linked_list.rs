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
    pub fn odd_even_list(mut head: Node) -> Node {
        let mut even_head = ListNode::new(-1);
        let mut odd_head = ListNode::new(-1);

        let (mut even, mut odd, mut flipflop) = (&mut even_head, &mut odd_head, true);
        while let Some(mut node) = head {
            head = node.next.take();
            match flipflop {
                true => {
                    even.next = Some(node);
                    even = even.next.as_mut().unwrap();
                }
                false => {
                    odd.next = Some(node);
                    odd = odd.next.as_mut().unwrap();
                }
            };
            flipflop = !flipflop;
        }
        even.next = odd_head.next;
        even_head.next
    }
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

        let mut t1 = ListNode::new(1);
        let mut t2 = ListNode::new(3);
        let mut t3 = ListNode::new(5);
        let mut t4 = ListNode::new(2);
        let mut t5 = ListNode::new(4);
        t4.next = Some(Box::new(t5));
        t3.next = Some(Box::new(t4));
        t2.next = Some(Box::new(t3));
        t1.next = Some(Box::new(t2));

        assert_eq!(
            Solution::odd_even_list(Some(Box::new(n1))),
            Some(Box::new(t1))
        );
    }

    #[test]
    fn ex2() {
        let mut n1 = ListNode::new(2);
        let mut n2 = ListNode::new(1);
        let mut n3 = ListNode::new(3);
        let mut n4 = ListNode::new(5);
        let mut n5 = ListNode::new(6);
        let mut n6 = ListNode::new(4);
        let mut n7 = ListNode::new(7);
        n6.next = Some(Box::new(n7));
        n5.next = Some(Box::new(n6));
        n4.next = Some(Box::new(n5));
        n3.next = Some(Box::new(n4));
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));

        let mut t1 = ListNode::new(2);
        let mut t2 = ListNode::new(3);
        let mut t3 = ListNode::new(6);
        let mut t4 = ListNode::new(7);
        let mut t5 = ListNode::new(1);
        let mut t6 = ListNode::new(5);
        let mut t7 = ListNode::new(4);
        t6.next = Some(Box::new(t7));
        t5.next = Some(Box::new(t6));
        t4.next = Some(Box::new(t5));
        t3.next = Some(Box::new(t4));
        t2.next = Some(Box::new(t3));
        t1.next = Some(Box::new(t2));

        assert_eq!(
            Solution::odd_even_list(Some(Box::new(n1))),
            Some(Box::new(t1))
        );
    }
}
