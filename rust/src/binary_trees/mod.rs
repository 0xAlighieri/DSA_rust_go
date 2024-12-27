use crate::queue::Queue;

pub enum BST<T: Ord + std::fmt::Debug + Clone + Copy> {
    Leaf {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Empty,
}

impl<T: Ord + std::fmt::Debug + Clone + Copy> BST<T> {
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

    pub fn compare(&self, other: &BST<T>) -> bool
    where
        T: Ord + PartialEq,
    {
        match (self, other) {
            (BST::Empty, BST::Empty) => true,
            (
                BST::Leaf {
                    value: v1,
                    left: l1,
                    right: r1,
                },
                BST::Leaf {
                    value: v2,
                    left: l2,
                    right: r2,
                },
            ) => {
                // First check values to short-circuit if they're different
                if v1 != v2 {
                    return false;
                }
                // Then recursively check subtrees
                l1.compare(l2) && r1.compare(r2)
            }
            _ => false,
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

    pub fn bf_search(&self, needle: T) -> bool {
        match self {
            BST::Empty => false,
            BST::Leaf {
                ref value,
                ref left,
                ref right,
            } => {
                let mut q = Queue::new();
                q.enqueue(*value);

                let mut nodes_to_check = vec![(left, right)];

                while q.len() > 0 {
                    if let Some(curr) = q.deque() {
                        if curr == needle {
                            return true; // Actually return when we find the value
                        }

                        if let Some((left_child, right_child)) = nodes_to_check.pop() {
                            match left_child.as_ref() {
                                BST::Leaf {
                                    value: v,
                                    left: l,
                                    right: r,
                                } => {
                                    q.enqueue(*v);
                                    nodes_to_check.push((l, r));
                                }
                                BST::Empty => {}
                            }

                            match right_child.as_ref() {
                                BST::Leaf {
                                    value: v,
                                    left: l,
                                    right: r,
                                } => {
                                    q.enqueue(*v);
                                    nodes_to_check.push((l, r));
                                }
                                BST::Empty => {}
                            }
                        }
                    }
                }
                false
            }
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

    #[test]
    fn test_bf_search_empty_tree() {
        // First, let's test the simplest case - an empty tree
        let empty_tree: BST<i32> = BST::Empty;
        assert!(
            !empty_tree.bf_search(5),
            "Searching in an empty tree should always return false"
        );
    }

    #[test]
    fn test_bf_search_single_node() {
        // Next, test a tree with just one node
        let mut single_node = BST::Empty;
        single_node.insert(5);

        assert!(
            single_node.bf_search(5),
            "Should find the value in a single-node tree"
        );
        assert!(
            !single_node.bf_search(3),
            "Should not find a non-existent value in single-node tree"
        );
    }

    #[test]
    fn test_bf_search_complete_tree() {
        // Create a more complete tree structure:
        //       5
        //      / \
        //     3   7
        //    /     \
        //   1       9
        let mut tree = BST::Empty;
        tree.insert(5); // Root
        tree.insert(3); // Left child
        tree.insert(7); // Right child
        tree.insert(1); // Left-left child
        tree.insert(9); // Right-right child

        // Test finding values at different levels of the tree
        assert!(tree.bf_search(5), "Should find root value");
        assert!(tree.bf_search(3), "Should find value in left subtree");
        assert!(tree.bf_search(7), "Should find value in right subtree");
        assert!(tree.bf_search(1), "Should find value in deep left position");
        assert!(
            tree.bf_search(9),
            "Should find value in deep right position"
        );

        // Test searching for non-existent values
        assert!(
            !tree.bf_search(4),
            "Should not find value between existing nodes"
        );
        assert!(
            !tree.bf_search(0),
            "Should not find value less than minimum"
        );
        assert!(
            !tree.bf_search(10),
            "Should not find value greater than maximum"
        );
    }

    #[test]
    fn test_bf_search_with_duplicates() {
        // Create a tree with duplicate values:
        //       5
        //      / \
        //     3   5
        //    /     \
        //   3       5
        let mut tree = BST::Empty;
        tree.insert(5);
        tree.insert(3);
        tree.insert(5); // Duplicate of root
        tree.insert(3); // Duplicate of left child
        tree.insert(5); // Another duplicate

        assert!(
            tree.bf_search(5),
            "Should find value that appears multiple times"
        );
        assert!(
            tree.bf_search(3),
            "Should find duplicate value in left subtree"
        );
        assert!(
            !tree.bf_search(7),
            "Should not find non-existent value in tree with duplicates"
        );
    }

    // Helper function to create test trees more easily
    fn create_test_tree(values: &[i32]) -> BST<i32> {
        let mut tree = BST::Empty;
        for &value in values {
            if let BST::Empty = tree {
                tree = BST::Leaf {
                    value,
                    left: Box::new(BST::Empty),
                    right: Box::new(BST::Empty),
                };
            } else {
                tree.insert(value);
            }
        }
        tree
    }

    #[test]
    fn test_compare_empty_trees() {
        // Two empty trees should be considered equal
        let tree1: BST<i32> = BST::Empty;
        let tree2: BST<i32> = BST::Empty;
        assert!(tree1.compare(&tree2), "Empty trees should be equal");
    }

    #[test]
    fn test_compare_empty_vs_nonempty() {
        // An empty tree should not equal a non-empty tree
        let tree1: BST<i32> = BST::Empty;
        let tree2 = create_test_tree(&[5]);
        assert!(
            !tree1.compare(&tree2),
            "Empty tree should not equal non-empty tree"
        );
        assert!(
            !tree2.compare(&tree1),
            "Non-empty tree should not equal empty tree"
        );
    }

    #[test]
    fn test_compare_single_node() {
        // Test trees with just one node
        let tree1 = create_test_tree(&[5]);
        let tree2 = create_test_tree(&[5]);
        let tree3 = create_test_tree(&[7]);

        assert!(
            tree1.compare(&tree2),
            "Identical single-node trees should be equal"
        );
        assert!(
            !tree1.compare(&tree3),
            "Different single-node trees should not be equal"
        );
    }

    #[test]
    fn test_compare_complex_identical_trees() {
        // Test more complex trees with multiple levels
        // Creates a tree structure:
        //       5
        //      / \
        //     3   7
        //    /     \
        //   1       9
        let tree1 = create_test_tree(&[5, 3, 7, 1, 9]);
        let tree2 = create_test_tree(&[5, 3, 7, 1, 9]);

        assert!(
            tree1.compare(&tree2),
            "Identical complex trees should be equal"
        );
    }

    #[test]
    fn test_compare_different_structures() {
        // Test trees with different structures
        // First tree:         Second tree:
        //     5                    5
        //    /                    /
        //   3                    4
        //    \                  /
        //     4                3
        let tree1 = create_test_tree(&[5, 3, 4]);
        let tree2 = create_test_tree(&[5, 4, 3]);

        assert!(
            !tree1.compare(&tree2),
            "Trees with different structures should not be equal"
        );
    }

    #[test]
    fn test_compare_with_duplicates() {
        // Test how comparison handles duplicate values
        let tree1 = create_test_tree(&[5, 3, 7, 3, 5]);
        let tree2 = create_test_tree(&[5, 3, 7, 3, 5]);
        let tree3 = create_test_tree(&[5, 3, 7, 3, 7]); // Different duplicate

        assert!(
            tree1.compare(&tree2),
            "Trees with identical structure and duplicates should be equal"
        );
        assert!(
            !tree1.compare(&tree3),
            "Trees with different duplicate values should not be equal"
        );
    }

    #[test]
    fn test_compare_different_depths() {
        // Test trees of different depths
        // Tree1:     Tree2:
        //   5          5
        //  / \        /
        // 3   7      3
        let tree1 = create_test_tree(&[5, 3, 7]);
        let tree2 = create_test_tree(&[5, 3]);

        assert!(
            !tree1.compare(&tree2),
            "Trees with different depths should not be equal"
        );
    }

    #[test]
    fn test_compare_ownership() {
        // Test that comparison doesn't consume the trees
        let tree1 = create_test_tree(&[5, 3, 7]);
        let tree2 = create_test_tree(&[5, 3, 7]);

        // First comparison
        assert!(tree1.compare(&tree2));

        // We should be able to use the trees again
        assert!(tree1.compare(&tree2));
    }
}
