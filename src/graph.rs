pub type EdgeIndex<N, E> = generational_arena::TypedIndex<Edge<N, E>>;
pub type NodeIndex<N, E> = generational_arena::TypedIndex<Node<N, E>>;

/// The graph's edge type.
// #[derive(Debug)]
pub struct Edge<N, E> {
    /// Associated edge data.
    pub weight: E,
    /// Next edge in outgoing and incoming edge lists.
    next: [Option<EdgeIndex<N, E>>; 2],
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
    next: [Option<EdgeIndex<N, E>>; 2],
}

pub struct Graph<N, E> {
    nodes: generational_arena::Arena<Node<N, E>>,
    edges: generational_arena::Arena<Edge<N, E>>,
}

impl<N, E> Graph<N, E> {
    pub fn add_node(&mut self, weight: N) -> NodeIndex<N, E> {
        let node = Node {
            weight,
            next: Default::default(),
        };

        // let node_idx = NodeIndex::new(self.nodes.len());
        // // check for max capacity, except if we use usize
        // assert!(<Ix as IndexType>::max().index() == !0 || NodeIndex::end() != node_idx);
        // self.nodes.push(node);
        // node_idx
        // todo!()
        self.nodes.typed_insert(node)
    }

    pub fn add_edge(
        &mut self,
        a: NodeIndex<N, E>,
        b: NodeIndex<N, E>,
        weight: E,
    ) -> EdgeIndex<N, E> {
        assert!(a != b);
        // let edge_idx = EdgeIndex::new(self.edges.len());
        // assert!(<Ix as IndexType>::max().index() == !0 || EdgeIndex::end() != edge_idx);
        // let mut edge = Edge {
        //     weight,
        //     node: [a, b],
        //     next: [EdgeIndex::end(); 2],
        // };
        // match index_twice(&mut self.nodes, a.index(), b.index()) {
        //     Pair::None => panic!("Graph::add_edge: node indices out of bounds"),
        //     Pair::One(an) => {
        //         edge.next = an.next;
        //         an.next[0] = edge_idx;
        //         an.next[1] = edge_idx;
        //     }
        //     Pair::Both(an, bn) => {
        //         // a and b are different indices
        //         edge.next = [an.next[0], bn.next[1]];
        //         an.next[0] = edge_idx;
        //         bn.next[1] = edge_idx;
        //     }
        // }
        // self.edges.push(edge);
        // edge_idx
        let mut edge = Edge {
            weight,
            node: [a, b],
            next: Default::default(),
        };
        // match index_twice(&mut self.nodes, a.index(), b.index()) {
        //     Pair::None => panic!("Graph::add_edge: node indices out of bounds"),
        //     Pair::One(an) => {
        //         edge.next = an.next;
        //         an.next[0] = edge_idx;
        //         an.next[1] = edge_idx;
        //     }
        //     Pair::Both(an, bn) => {
        //         // a and b are different indices
        //         edge.next = [an.next[0], bn.next[1]];
        //         an.next[0] = edge_idx;
        //         bn.next[1] = edge_idx;
        //     }
        // }
        todo!()
    }
}
