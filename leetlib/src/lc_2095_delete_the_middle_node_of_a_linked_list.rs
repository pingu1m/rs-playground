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

type LNode = Option<Box<ListNode>>;

fn len(node: &LNode) -> usize {
    match node.as_ref() {
        None => 0,
        Some(n) => 1 + len(&n.next),
    }
}

fn del_kth(node: &mut LNode, k: usize) {
    match k {
        0 => *node = node.as_mut().unwrap().next.take(),
        _ => del_kth(&mut node.as_mut().unwrap().next, k - 1),
    }
}

pub fn delete_middle2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mid = len(&head) / 2;
    del_kth(&mut head, mid);
    head
}

pub fn delete_middle(mut head: LNode) -> LNode {
    let mut fast = &(head.clone());
    let mut slow = &mut head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &mut (slow.as_mut().unwrap().next);
        fast = &(fast.as_ref()?.next.as_ref()?.next);
        // fast = fast.as_ref()?.next.as_ref()?.next.as_ref();
        // Option<&Box<ListNode>>
        // fast = &(fast.as_ref()?.next.as_ref()?.next);
        // &Option<Box<ListNode>>
    }

    *slow = slow.as_mut()?.next.take();
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut n1 = ListNode::new(1);
        let mut n2 = ListNode::new(3);
        let mut n3 = ListNode::new(4);
        let mut n4 = ListNode::new(7);
        let mut n5 = ListNode::new(1);
        let mut n6 = ListNode::new(2);
        let mut n7 = ListNode::new(6);
        n6.next = Some(Box::new(n7));
        n5.next = Some(Box::new(n6));
        n4.next = Some(Box::new(n5));
        n3.next = Some(Box::new(n4));
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));

        let mut t1 = ListNode::new(1);
        let mut t2 = ListNode::new(3);
        let mut t3 = ListNode::new(4);
        let mut t4 = ListNode::new(1);
        let mut t5 = ListNode::new(2);
        let mut t6 = ListNode::new(6);
        t5.next = Some(Box::new(t6));
        t4.next = Some(Box::new(t5));
        t3.next = Some(Box::new(t4));
        t2.next = Some(Box::new(t3));
        t1.next = Some(Box::new(t2));

        assert_eq!(delete_middle(Some(Box::new(n1))), Some(Box::new(t1)));
    }

    #[test]
    fn ex2() {
        let mut n1 = ListNode::new(1);
        let mut n2 = ListNode::new(2);
        let mut n3 = ListNode::new(3);
        let mut n4 = ListNode::new(4);
        n3.next = Some(Box::new(n4));
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));

        let mut t1 = ListNode::new(1);
        let mut t2 = ListNode::new(2);
        let mut t4 = ListNode::new(4);
        t2.next = Some(Box::new(t4));
        t1.next = Some(Box::new(t2));

        assert_eq!(delete_middle(Some(Box::new(n1))), Some(Box::new(t1)));
    }

    #[test]
    fn ex3() {
        let mut n1 = ListNode::new(2);
        let mut n2 = ListNode::new(1);
        n1.next = Some(Box::new(n2));

        let mut t1 = ListNode::new(2);

        assert_eq!(delete_middle(Some(Box::new(n1))), Some(Box::new(t1)));
    }
}
