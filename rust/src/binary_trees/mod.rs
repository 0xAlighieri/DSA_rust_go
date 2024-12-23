pub enum BST<T: Ord + std::fmt::Debug + Clone> {
    Leaf {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Empty,
}

impl<T: Ord + std::fmt::Debug + Clone> BST<T> {
    pub fn insert(&mut self, new_value: T) {
        match self {
            BST::Empty => {
                *self = BST::Leaf {
                    value: new_value,
                    left: Box::new(BST::Empty),
                    right: Box::new(BST::Empty),
                };
            }
            BST::Leaf { value, left, right } => {
                if new_value <= *value {
                    left.insert(new_value);
                } else {
                    right.insert(new_value);
                }
            }
        }
    }

    pub fn inorder_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.inorder_collect(&mut result);
        result
    }

    fn inorder_collect(&self, result: &mut Vec<T>) {
        match self {
            BST::Empty => {}
            BST::Leaf {
                ref value,
                ref left,
                ref right,
            } => {
                left.inorder_collect(result);
                result.push(value.clone());
                right.inorder_collect(result);
            }
        }
    }

    pub fn preorder_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.preorder_collect(&mut result);
        result
    }

    fn preorder_collect(&self, result: &mut Vec<T>) {
        match self {
            BST::Empty => {}
            BST::Leaf {
                ref value,
                ref left,
                ref right,
            } => {
                result.push(value.clone());
                left.preorder_collect(result);
                right.preorder_collect(result);
            }
        }
    }

    pub fn postorder_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.postorder_collect(&mut result);
        result
    }

    fn postorder_collect(&self, result: &mut Vec<T>) {
        match self {
            BST::Empty => {}
            BST::Leaf {
                ref value,
                ref left,
                ref right,
            } => {
                left.postorder_collect(result);
                right.postorder_collect(result);
                result.push(value.clone());
            }
        }
    }

    pub fn inorder_print(&self) {
        for value in self.inorder_traversal() {
            println!("{:?}", value);
        }
    }

    pub fn preorder_print(&self) {
        for value in self.preorder_traversal() {
            println!("{:?}", value);
        }
    }

    pub fn postorder_print(&self) {
        for value in self.postorder_traversal() {
            println!("{:?}", value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traversals() {
        let mut tree = BST::Empty;
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);

        assert_eq!(tree.inorder_traversal(), vec![3, 5, 7]);
        assert_eq!(tree.preorder_traversal(), vec![5, 3, 7]);
        assert_eq!(tree.postorder_traversal(), vec![3, 7, 5]);
    }

    #[test]
    fn test_traversals_with_duplicates() {
        let mut tree = BST::Empty;
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);

        assert_eq!(
            tree.inorder_traversal(),
            vec![3, 3, 5, 5, 7, 7],
            "Inorder should show all values in ascending order, including duplicates"
        );

        assert_eq!(
            tree.preorder_traversal(),
            vec![5, 3, 3, 5, 7, 7],
            "Preorder should show root first, then left subtree, then right subtree"
        );

        assert_eq!(
            tree.postorder_traversal(),
            vec![3, 5, 3, 7, 7, 5],
            "Postorder should show leaves first, then their parents"
        );
    }
}
