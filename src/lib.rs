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

// #[derive(Debug)]
pub struct Node<N, E> {
    /// Associated node data.
    pub weight: N,
    /// Next edge in outgoing and incoming edge lists.
    next: [EdgeIndex<N, E>; 2],
}

pub struct Graph<N, E> {
    nodes: generational_arena::Arena<Node<N, E>>,
}
