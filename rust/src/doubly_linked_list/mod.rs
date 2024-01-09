use std::{cell::RefCell, rc::Rc};

type ListItem<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    value: T,
    prev: ListItem<T>,
    next: ListItem<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Clone)]
struct DoublyLinkedList<T> {
    head: ListItem<T>,
    tail: ListItem<T>,
    len: usize,
}

impl<T: Copy> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn prepend(&mut self, value: T) {
        let node = Node::new(value);
        self.len += 1;

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(node.clone());
                node.borrow_mut().next = Some(old_head);
                self.head = Some(node);
            }
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }
    }
}
