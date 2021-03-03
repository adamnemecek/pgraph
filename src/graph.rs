pub type EdgeIndex<N, E> = generational_arena::TypedIndex<Edge<N, E>>;
pub type NodeIndex<N, E> = generational_arena::TypedIndex<Node<N, E>>;

/// The graph's edge type.
// #[derive(Debug)]
pub struct Edge<N, E> {
    /// Associated edge data.
    pub weight: E,
    /// Next edge in outgoing and incoming edge lists.
    next: [EdgeIndex<N, E>; 2],
    /// Start and End node index
    node: [NodeIndex<N, E>; 2],
}

impl<N, E> Edge<N, E> {
    pub fn source(&self) -> NodeIndex<N, E> {
        self.node[0]
    }

    pub fn target(&self) -> NodeIndex<N, E> {
        self.node[1]
    }
}

// #[derive(Debug)]
pub struct Node<N, E> {
    /// Associated node data.
    pub weight: N,
    /// Next edge in outgoing and incoming edge lists.
    next: [EdgeIndex<N, E>; 2],
}

pub struct Graph<N, E> {
    nodes: generational_arena::Arena<Node<N, E>>,
    edges: generational_arena::Arena<Edge<N, E>>,
}

impl<N, E> Graph<N, E> {
    pub fn add_node(&mut self, weight: N) -> NodeIndex<N, E> {
        // let node = Node {
        //     weight,
        //     next: [EdgeIndex::end(), EdgeIndex::end()],
        // };
        // let node_idx = NodeIndex::new(self.nodes.len());
        // // check for max capacity, except if we use usize
        // assert!(<Ix as IndexType>::max().index() == !0 || NodeIndex::end() != node_idx);
        // self.nodes.push(node);
        // node_idx
        todo!()
    }
}
