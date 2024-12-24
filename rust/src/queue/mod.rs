use std::{cell::RefCell, rc::Rc};

type QueueItem<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: QueueItem<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Clone)]
pub struct Queue<T> {
    head: QueueItem<T>,
    tail: QueueItem<T>,
    len: usize,
}

impl<T: Copy> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Add an element to the end of the queue
    pub fn enqueue(&mut self, value: T) {
        let node = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(node.clone()),
            None => self.head = Some(node.clone()),
        }
        self.len += 1;
        self.tail = Some(node);
    }

    /// Remove an element from the front of the queue
    pub fn deque(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.len -= 1;
            match Rc::try_unwrap(head) {
                Ok(cell) => cell.into_inner().value,
                Err(_) => panic!("Unexpected multiple references to head node"),
            }
        })
    }

    /// Return the length of the queue
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return the first element of the queue
    pub fn peek(&self) -> Option<T> {
        self.head.as_ref().map(|head| head.borrow().value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q: Queue<i32> = Queue::new();

        assert_eq!(q.peek(), None);
        for i in 1..=10 {
            q.enqueue(i);
        }
        println!("len: {}\n", q.len());

        assert_eq!(q.len(), 10);

        assert_eq!(q.peek(), Some(1));
        assert_eq!(q.deque(), Some(1));

        assert_eq!(q.deque(), Some(2));
        assert_eq!(q.deque(), Some(3));
        assert_eq!(q.deque(), Some(4));
        assert_eq!(q.deque(), Some(5));
        assert_eq!(q.deque(), Some(6));
        assert_eq!(q.deque(), Some(7));
        assert_eq!(q.deque(), Some(8));
        assert_eq!(q.deque(), Some(9));
        assert_eq!(q.deque(), Some(10));

        assert_eq!(q.len(), 0);
        assert_eq!(q.deque(), None);
        assert_eq!(q.peek(), None);
    }
}
