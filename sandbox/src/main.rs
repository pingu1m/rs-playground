#![allow(dead_code, unused)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn traverse_recursive(&self) {
        // Print the value of the current node.
        println!("{}", self.val);

        // Recursive call on the next node if it exists.
        if let Some(ref next) = self.next {
            next.traverse_recursive();
        }
    }
}

fn traverse_list_recursive(node: &Option<Box<ListNode>>) {
    match node {
        Some(inner_node) => {
            // Print the value of the current node.
            println!("{}", inner_node.val);
            // inner_node.val += 0;

            // Recursive call on the next node if it exists.
            traverse_list_recursive(&inner_node.next);
        }
        None => {}
    }
}

pub fn remove_nodes_with_value(head: Option<Box<ListNode>>, target: i32) -> Option<Box<ListNode>> {
    match head {
        Some(mut node) => {
            if node.val == target {
                // If the current node's value matches the target, skip it and recursively process the rest.
                remove_nodes_with_value(node.next.take(), target)
            } else {
                // If the current node's value doesn't match the target, process it and recursively process the rest.
                node.next = remove_nodes_with_value(node.next.take(), target);
                Some(node)
            }
        }
        None => None, // End of the list
    }
}

// pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     match head {
//         Some(mut node) => {
//             let mut next_node = node.next.take();
//             let mut last_node = reverse_list(next_node);
//             next_node.as_mut().unwrap().next = Some(node);
//
//             next_node
//         }
//         None => None, // End of the list
//     }
// }

fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn reverse_recursive(
        current: Option<Box<ListNode>>,
        prev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match current {
            Some(mut node) => {
                let next_node = node.next.take();
                node.next = prev;
                reverse_recursive(next_node, Some(node))
            }
            None => prev,
        }
    }

    reverse_recursive(head, None)
}

fn ll_sum_rec(head: Option<Box<ListNode>>) -> i32 {
    match head {
        Some(mut node) => node.val + ll_sum_rec(node.next.take()),
        None => 0,
    }
}

fn ll_sum_rec2(head: &Option<Box<ListNode>>) -> i32 {
    // Some(ref node) == Some(node)
    match head {
        Some(ref node) => node.val + ll_sum_rec2(&node.next),
        None => 0,
    }
}

fn ll_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut result = 0;
    let mut current = head;

    while let Some(node) = current {
        result += node.val;
        current = node.next;
    }
    result
}

fn main() {
    let node1 = ListNode::new(1);
    let node2 = ListNode::new(2);
    let node3 = ListNode::new(3);
    let node4 = ListNode::new(2);
    let node5 = ListNode::new(5);

    let mut head = Some(Box::new(node1));
    head.as_mut().unwrap().next = Some(Box::new(node2));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(node3));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(node4));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(node5));

    println!("Original linked list:");
    traverse_list_recursive(&head);

    let target_value = 2;
    let new_head = reverse_linked_list(head);

    println!(
        "Linked list after removing nodes with value {}: ",
        target_value
    );
    traverse_list_recursive(&new_head);
    println!();

    println!("Linked list sum below");

    let linked_list_sum_rec = ll_sum_rec(new_head.clone());
    let linked_list_sum = ll_sum(new_head.clone());

    println!("ll rec sum: {linked_list_sum_rec}");
    println!("ll sum: {linked_list_sum}");

    println!("debug: {new_head:?}");

    println!("-----------------------------------------------------------------------");
}
