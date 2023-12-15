use std::{cell::RefCell, rc::Rc};

type StackItem<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    value: T,
    prev: StackItem<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { value, prev: None }))
    }
}

#[derive(Clone)]
struct Stack<T> {
    head: StackItem<T>,
    len: usize,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { head: None, len: 0 }
    }

    /// push an element onto the stack
    pub fn push(&mut self, value: T) {
        let node = Node::new(value);
        match self.head.take() {
            Some(old) => {
                node.borrow_mut().prev = Some(old);
                self.head = Some(node);
            }
            None => self.head = Some(node),
        }
        self.len += 1;
    }

    /// Remove an element from the stack
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len = self.len.saturating_sub(1);
        match self.len {
            0 => {
                let head = self.head.take();
                head.map(|head| head.borrow_mut().value)
            }
            _ => {
                let head = self.head.take();
                self.head = head
                    .as_ref()
                    .and_then(|head| head.borrow_mut().prev.clone());
                head.map(|head| head.borrow_mut().value)
            }
        }
    }

    /// Return the length of the stack
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return the next element in the stack
    pub fn peek(&self) -> Option<T> {
        self.head.as_ref().map(|head| head.borrow().value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        assert_eq!(s.len(), 0, "Expected length to be 0");

        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.len(), 3, "Expected length to be 3");
        assert_eq!(s.peek().unwrap(), 3, "Expected top of stack to be 3");

        assert_eq!(s.pop().unwrap(), 3, "Expected popped value to be 3");
        assert_eq!(s.pop().unwrap(), 2, "Expected popped value to be 2");
        assert_eq!(s.pop().unwrap(), 1, "Expected popped value to be 1");
        assert_eq!(s.len(), 0, "Expected length to be 0");
        assert!(
            s.pop().is_none(),
            "Expected pop to return None on empty stack"
        );
    }
}
