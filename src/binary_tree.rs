#[derive(Debug, PartialEq, Eq)]
pub enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> BinaryTree<T> {
    // Create an empty node.
    pub fn new() -> BinaryTree<T> {
        BinaryTree::Empty
    }

    // Create a node containing the specified 'value'.
    pub fn new_node(value: T) -> BinaryTree<T> {
        BinaryTree::Node {
            value,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }
    }

    // Create a node with the specified 'value', having the specified 'left' and 'right' children.
    pub fn new_node_with_children(
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    ) -> BinaryTree<T> {
        BinaryTree::Node { value, left, right }
    }

    // Return true if this node is empty.
    pub fn is_empty(&self) -> bool {
        match *self {
            BinaryTree::Empty => true,
            _ => false,
        }
    }

    // Set the left child of this node to the specified 'node'.
    pub fn set_left(&mut self, node: Box<BinaryTree<T>>) {
        match *self {
            BinaryTree::Empty => {
                panic!("Cannot set left on empty node");
            }
            BinaryTree::Node { ref mut left, .. } => {
                *left = node;
            }
        };
    }

    // Set the right child of this node to the specified 'node'.
    pub fn set_right(&mut self, node: Box<BinaryTree<T>>) {
        match *self {
            BinaryTree::Empty => {
                panic!("Cannot set right on empty node");
            }
            BinaryTree::Node { ref mut right, .. } => {
                *right = node;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        let root: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(root, BinaryTree::Empty);
    }

    #[test]
    fn new_node() {
        let root = BinaryTree::new_node(42i32);

        match root {
            BinaryTree::Node { value, left, right } => {
                assert_eq!(value, 42i32);
                assert!(left.is_empty());
                assert!(right.is_empty());
            }
            _ => panic!("Expected Node, got something else"),
        }
    }

    #[test]
    fn new_node_with_children() {
        let root = BinaryTree::new_node_with_children(
            42i32,
            Box::new(BinaryTree::new_node(1i32)),
            Box::new(BinaryTree::<i32>::new()),
        );

        match root {
            BinaryTree::Node { value, left, right } => {
                assert_eq!(value, 42);
                assert_eq!(*left, BinaryTree::new_node(1i32));
                assert!(right.is_empty());
            }
            _ => panic!("Expected Node with specific left and right children, got something else"),
        }
    }

    #[test]
    fn is_empty() {
        let root: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(root.is_empty(), true)
    }

    #[test]
    fn set_left() {
        let mut root = BinaryTree::new_node(42i32);
        root.set_left(Box::new(BinaryTree::new_node(1i32)));

        match &root {
            &BinaryTree::Node {
                ref value,
                ref left,
                ref right,
            } => {
                assert_eq!(*value, 42);

                match &**left {
                    &BinaryTree::Node {
                        ref value,
                        ref left,
                        ref right,
                    } => {
                        assert_eq!(*value, 1i32);
                        assert_eq!(**left, BinaryTree::Empty);
                        assert_eq!(**right, BinaryTree::Empty);
                    }
                    _ => panic!("Unexpected variant for left child"),
                };

                if let BinaryTree::Empty = **right {
                    /* okay */
                } else {
                    panic!("Unexpected variant for right child")
                }
            }
            _ => panic!("Unexpected variant"),
        }
    }

    #[test]
    fn set_right() {
        let mut root = BinaryTree::new_node(42i32);
        root.set_right(Box::new(BinaryTree::new_node(1i32)));

        match &root {
            &BinaryTree::Node {
                ref value,
                ref left,
                ref right,
            } => {
                assert_eq!(*value, 42i32);

                if let BinaryTree::Empty = **left {
                    /* okay */
                } else {
                    panic!("Unexpected variant for left child")
                }

                match &**right {
                    &BinaryTree::Node {
                        ref value,
                        ref left,
                        ref right,
                    } => {
                        assert_eq!(*value, 1);
                        assert_eq!(**left, BinaryTree::Empty);
                        assert_eq!(**right, BinaryTree::Empty);
                    }
                    _ => panic!("Unexpected variant for right child"),
                };
            }
            _ => panic!("Unexpected variant"),
        }
    }
}
