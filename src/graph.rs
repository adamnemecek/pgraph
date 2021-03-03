pub type NodeIndex<N, E> = generational_arena::TypedIndex<Node<N, E>>;
pub type EdgeIndex<N, E> = generational_arena::TypedIndex<Edge<N, E>>;

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
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            edges: Default::default(),
        }
    }

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
        let edge_index = self.edges.typed_insert(edge);
        let mut an = &mut self[a];
        an.next[0] = Some(edge_index);

        let mut bn = &mut self[b];
        bn.next[1] = Some(edge_index);
        edge_index
    }

    pub fn remove_node(&mut self, a: NodeIndex<N, E>) -> Option<N> {
        // self.nodes.get(a.index())?;
        // for d in &DIRECTIONS {
        //     let k = d.index();

        //     // Remove all edges from and to this node.
        //     loop {
        //         let next = self.nodes[a.index()].next[k];
        //         if next == EdgeIndex::end() {
        //             break;
        //         }
        //         let ret = self.remove_edge(next);
        //         debug_assert!(ret.is_some());
        //         let _ = ret;
        //     }
        // }

        // // Use swap_remove -- only the swapped-in node is going to change
        // // NodeIndex<Ix>, so we only have to walk its edges and update them.

        // let node = self.nodes.swap_remove(a.index());

        // // Find the edge lists of the node that had to relocate.
        // // It may be that no node had to relocate, then we are done already.
        // let swap_edges = match self.nodes.get(a.index()) {
        //     None => return Some(node.weight),
        //     Some(ed) => ed.next,
        // };

        // // The swapped element's old index
        // let old_index = NodeIndex::new(self.nodes.len());
        // let new_index = a;

        // // Adjust the starts of the out edges, and ends of the in edges.
        // for &d in &DIRECTIONS {
        //     let k = d.index();
        //     let mut edges = edges_walker_mut(&mut self.edges, swap_edges[k], d);
        //     while let Some(curedge) = edges.next_edge() {
        //         debug_assert!(curedge.node[k] == old_index);
        //         curedge.node[k] = new_index;
        //     }
        // }
        // Some(node.weight)
        todo!()
    }

    pub fn get(&mut self, index: NodeIndex<N, E>) -> Option<&Node<N, E>> {
        self.nodes.typed_get(index)
    }
}

impl<N, E> std::ops::Index<NodeIndex<N, E>> for Graph<N, E> {
    type Output = Node<N, E>;
    fn index(&self, index: NodeIndex<N, E>) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<N, E> std::ops::IndexMut<NodeIndex<N, E>> for Graph<N, E> {
    fn index_mut(&mut self, index: NodeIndex<N, E>) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
