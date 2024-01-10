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

impl<T: Copy + PartialEq> DoublyLinkedList<T> {
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

    pub fn append(&mut self, value: T) {
        let node = Node::new(value);
        self.len += 1;
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(node);
            }
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        let mut current = self.head.clone();
        for _ in 0..index {
            current = current.unwrap().borrow().next.clone();
        }

        current.map(|node| node.borrow().value)
    }

    pub fn insert_at(&mut self, index: usize, value: T) {
        if index > self.len {
            panic!("Index out of bounds");
        }

        if index == 0 {
            self.prepend(value);
            return;
        } else if index == self.len {
            self.append(value);
            return;
        }

        let new_node = Node::new(value);
        let mut current = self.head.as_ref().unwrap().clone();

        for _ in 0..index {
            let next = current.borrow().next.clone();
            current = next.unwrap();
        }

        let prev = current.borrow().prev.clone();

        new_node.borrow_mut().prev = prev.clone();
        new_node.borrow_mut().next = Some(current.clone());

        if let Some(prev_node) = prev {
            prev_node.borrow_mut().next = Some(new_node.clone());
        }

        current.borrow_mut().prev = Some(new_node);

        self.len += 1;
    }

    pub fn remove_by_value(&mut self, value: T) -> Option<T> {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().value == value {
                return Some(self.remove_node(node));
            }
            current = node.borrow().next.clone();
        }
        None
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        let mut current = self.head.clone();
        for _ in 0..index {
            current = current.unwrap().borrow().next.clone();
        }
        current.map(|node| self.remove_node(node))
    }

    fn remove_node(&mut self, node: Rc<RefCell<Node<T>>>) -> T {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        if let Some(prev_node) = &prev {
            prev_node.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next_node) = &next {
            next_node.borrow_mut().prev = prev;
        } else {
            self.tail = prev;
        }

        self.len -= 1;
        Rc::try_unwrap(node).ok().unwrap().into_inner().value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend_single_element() {
        let mut list = DoublyLinkedList::new();
        list.prepend(10);

        assert_eq!(list.len, 1);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 10);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 10);
    }

    #[test]
    fn test_prepend_multiple_elements() {
        let mut list = DoublyLinkedList::new();
        list.prepend(10);
        list.prepend(20);

        assert_eq!(list.len, 2);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 20);
        assert_eq!(
            list.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            10
        );
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 10);
    }

    #[test]
    fn test_append_single_element() {
        let mut list = DoublyLinkedList::new();
        list.append(10);

        assert_eq!(list.len, 1);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 10);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 10);
    }

    #[test]
    fn test_append_multiple_elements() {
        let mut list = DoublyLinkedList::new();
        list.append(10);
        list.append(20);

        assert_eq!(list.len, 2);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 10);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 20);
        assert_eq!(
            list.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            20
        );
    }

    #[test]
    fn test_insert_at_beginning() {
        let mut list = DoublyLinkedList::new();
        list.insert_at(0, 10);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 10);
        assert_eq!(list.len, 1);
    }

    #[test]
    fn test_insert_at_end() {
        let mut list = DoublyLinkedList::new();
        list.append(10);
        list.insert_at(1, 20);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 20);
        assert_eq!(list.len, 2);
    }

    #[test]
    fn test_insert_at_middle() {
        let mut list = DoublyLinkedList::new();
        list.append(10);
        list.append(30);
        list.insert_at(1, 20);
        let middle_value = list
            .head
            .as_ref()
            .unwrap()
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow()
            .value;
        assert_eq!(middle_value, 20);
        assert_eq!(list.len, 3);
    }

    #[test]
    fn test_insert_in_empty_list() {
        let mut list = DoublyLinkedList::new();
        list.insert_at(0, 10);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 10);
        assert_eq!(list.len, 1);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_insert_at_out_of_bounds() {
        let mut list = DoublyLinkedList::new();
        list.insert_at(1, 10); // This should panic
    }

    #[test]
    fn test_remove_by_value_existing_item() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        let removed = list.remove_by_value(2);
        assert_eq!(removed, Some(2));
        assert_eq!(list.len, 2);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 1);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 3);
    }

    #[test]
    fn test_remove_by_value_non_existing_item() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(3);

        let removed = list.remove_by_value(2);
        assert_eq!(removed, None);
        assert_eq!(list.len, 2);
    }

    #[test]
    fn test_remove_by_value_single_item() {
        let mut list = DoublyLinkedList::new();
        list.append(1);

        let removed = list.remove_by_value(1);
        assert_eq!(removed, Some(1));
        assert_eq!(list.len, 0);
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_remove_by_value_head_and_tail() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        let removed_head = list.remove_by_value(1);
        assert_eq!(removed_head, Some(1));
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 2);

        let removed_tail = list.remove_by_value(3);
        assert_eq!(removed_tail, Some(3));
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 2);
    }

    #[test]
    fn test_remove_at_beginning() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        let removed = list.remove_at(0);
        assert_eq!(removed, Some(1));
        assert_eq!(list.len, 2);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 2);
    }

    #[test]
    fn test_remove_at_middle() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        let removed = list.remove_at(1);
        assert_eq!(removed, Some(2));
        assert_eq!(list.len, 2);
        assert_eq!(
            list.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            3
        );
    }

    #[test]
    fn test_remove_at_end() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        let removed = list.remove_at(2);
        assert_eq!(removed, Some(3));
        assert_eq!(list.len, 2);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 2);
    }

    #[test]
    fn test_remove_at_out_of_bounds() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);

        let removed = list.remove_at(3);
        assert_eq!(removed, None);
        assert_eq!(list.len, 2);
    }

    #[test]
    fn test_remove_at_single_element_list() {
        let mut list = DoublyLinkedList::new();
        list.append(1);

        let removed = list.remove_at(0);
        assert_eq!(removed, Some(1));
        assert_eq!(list.len, 0);
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_get_beginning() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.get(0), Some(1));
    }

    #[test]
    fn test_get_middle() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.get(1), Some(2));
    }

    #[test]
    fn test_get_end() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.get(2), Some(3));
    }

    #[test]
    fn test_get_out_of_bounds() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);

        assert_eq!(list.get(3), None);
    }

    #[test]
    fn test_get_single_element_list() {
        let mut list = DoublyLinkedList::new();
        list.append(1);

        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), None);
    }
}
