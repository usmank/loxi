#[derive(Debug, PartialEq, Eq)]
pub enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T: Eq> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree::Empty
    }

    pub fn new_node(new_value: T) -> BinaryTree<T> {
        BinaryTree::Node { value: new_value, left: Box::new(BinaryTree::Empty), right: Box::new(BinaryTree::Empty)}
    }

    pub fn is_empty(&self) -> bool {
        match self {
            &BinaryTree::Empty => true,
            _ => false,
        }
    }

    pub fn set_left(&mut self, new_left: Box<BinaryTree<T>>) {
        match self {
            &mut BinaryTree::Empty => {
                panic!("Cannot set left lG empty node");
            },
            &mut BinaryTree::Node { ref mut left, .. } => {
                *left = new_left;
            }
        };
    }

    pub fn set_right(&mut self, new_right: Box<BinaryTree<T>>) {
        match self {
            &mut BinaryTree::Empty => {
                panic!("Cannot set right on empty node");
            },
            &mut BinaryTree::Node { ref mut right, .. }=> {
                *right = new_right;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        let root : BinaryTree<i32> = BinaryTree::new();
        assert_eq!(root, BinaryTree::Empty);
    }

    #[test]
    fn new_node() {
        let root = BinaryTree::new_node(42);

        match root {
            BinaryTree::Node { value, left, right } => {
                assert_eq!(value, 42);
                assert!(left.is_empty());
                assert!(right.is_empty());
            },
            _ => {
                panic!("Expected Node, got something else");
            }
        }
    }

    #[test]
    fn is_empty() {
        let root : BinaryTree<i32> = BinaryTree::new();
        assert_eq!(root.is_empty(), true)
    }

    #[test]
    fn set_left() {
        let mut root = BinaryTree::new_node(42);
        root.set_left(Box::new(BinaryTree::new_node(1)));

        match &root {
            &BinaryTree::Node { ref value, ref left, ref right } => {
                assert_eq!(*value, 42);

                match &**left {
                    &BinaryTree::Node { ref value, ref left, ref right } => {
                        assert_eq!(*value, 1);
                        assert_eq!(**left, BinaryTree::Empty);
                        assert_eq!(**right, BinaryTree::Empty);
                    },
                    _ => {
                        panic!("Unexpected variant for left child")
                    },
                };

                if let BinaryTree::Empty = **right {
                    /* okay */
                } else {
                    panic!("Unexpected variant for right child")
                }
            },
            _ => {
                panic!("Unexpected variant");
            },
        }
    }

    #[test]
    fn set_right() {
        let mut root = BinaryTree::new_node(42);
        root.set_right(Box::new(BinaryTree::new_node(1)));

        match &root {
            &BinaryTree::Node { ref value, ref left, ref right } => {
                assert_eq!(*value, 42);

                if let BinaryTree::Empty = **left {
                    /* okay */
                } else {
                    panic!("Unexpected variant for left child")
                }

                match &**right {
                    &BinaryTree::Node { ref value, ref left, ref right } => {
                        assert_eq!(*value, 1);
                        assert_eq!(**left, BinaryTree::Empty);
                        assert_eq!(**right, BinaryTree::Empty);
                    },
                    _ => {
                        panic!("Unexpected variant for right child")
                    },
                };
            },
            _ => {
                panic!("Unexpected variant");
            },
        }
    }
}