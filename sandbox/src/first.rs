#![allow(dead_code, unused)]

use std::mem;

#[derive(Debug)]
pub struct Node {
    el: i32,
    next: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    fn new() -> Self {
        Self { head: Link::Empty }
    }

    fn push(&mut self, el: i32) {
        let new_node = Box::new(Node {
            el,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.el)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = curr {
            curr = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_test() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn push_test() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
    }
}
