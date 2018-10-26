#[derive(Debug)]
pub struct NodeId(usize);

// TODO: Rename to binary tree to reflect the two child restriction.
#[derive(Debug)]
struct BinaryNode<T> {
    data: T,
    left: Option<NodeId>,
    right: Option<NodeId>,
}

#[derive(Debug)]
pub struct NodeArena<T> {
    nodes: Vec<BinaryNode<T>>,
}

impl<T> NodeArena<T> {
    pub fn new() -> NodeArena<T> {
        NodeArena { nodes: Vec::new() }
    }

    pub fn new_node(&mut self, data: T, left: Option<NodeId>, right: Option<NodeId>) -> NodeId {
        let new_node_id = self.nodes.len();

        self.nodes.push(BinaryNode {
            data: data,
            left: left,
            right: right,
        });

        NodeId(new_node_id)
    }
}
