use crate::prelude::*;

// #[derive(Debug)]
#[derive(Clone)]
pub struct Graph<N, E> {
    nodes: generational_arena::Arena<Node<N, E>>,
    edges: generational_arena::Arena<Edge<N, E>>,
}

impl<N: PartialEq, E: PartialEq> PartialEq for Graph<N, E> {
    fn eq(&self, other: &Self) -> bool {
        if self.nodes.len() != other.nodes.len() {
            return false;
        }

        if self.edges.len() != other.edges.len() {
            return false;
        }

        for (a, b) in self.nodes.iter().zip(other.nodes.iter()) {
            if a.0 != b.0 {
                return false;
            }
        }

        for (a, b) in self.edges.iter().zip(other.edges.iter()) {
            if a.0 != b.0 {
                return false;
            }
        }

        true
    }
}

impl<N: Eq, E: Eq> Eq for Graph<N, E> {}

impl<N: std::fmt::Debug, E: std::fmt::Debug> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            edges: Default::default(),
        }
    }

    pub fn with_capacity(node_len: usize, edge_len: usize) -> Self {
        Self {
            nodes: generational_arena::Arena::with_capacity(node_len),
            edges: generational_arena::Arena::with_capacity(edge_len),
        }
    }

    pub fn add_node(&mut self, weight: N) -> NodeIndex<N, E> {
        let node = Node {
            weight,
            // next: Default::default(),
            next: Default::default(),
        };

        self.nodes.typed_insert(node)
    }

    pub fn incoming_edges(&self, index: NodeIndex<N, E>) -> Edges<'_, N, E> {
        let node = &self[index];
        Edges::new(Direction::Incoming, node.next, &self.edges)
    }

    pub fn outgoing_edges(&self, index: NodeIndex<N, E>) -> Edges<'_, N, E> {
        let node = &self[index];
        Edges::new(Direction::Outgoing, node.next, &self.edges)
    }

    pub fn incoming_edges_mut(&mut self, index: NodeIndex<N, E>) -> EdgesMut<'_, N, E> {
        let node = &self[index];
        EdgesMut::new(Direction::Incoming, node.next, &mut self.edges)
    }

    pub fn outgoing_edges_mut(&mut self, index: NodeIndex<N, E>) -> EdgesMut<'_, N, E> {
        let node = &self[index];
        EdgesMut::new(Direction::Outgoing, node.next, &mut self.edges)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn visit_map(&self) -> VisitMap<NodeIndex<N, E>> {
        VisitMap::with_capacity(self.node_count())
    }

    pub fn reset_map(&self, map: &mut VisitMap<NodeIndex<N, E>>) {
        map.clear();
        map.grow(self.node_count());
    }

    #[must_use]
    pub fn add_edge(
        &mut self,
        a: NodeIndex<N, E>,
        b: NodeIndex<N, E>,
        weight: E,
    ) -> Result<EdgeIndex<N, E>, GraphError<N, E>> {
        // assert!(a != b);
        if a == b {
            return Err(GraphError::WouldCycle(a));
        }

        let an = &self[a];
        let bn = &self[b];

        let edge = Edge {
            weight,
            from: a,
            to: b,
            next: Next {
                outgoing: an.next.outgoing,
                incoming: bn.next.incoming,
            },
        };

        let edge_index = self.edges.typed_insert(edge);

        let mut an = &mut self[a];
        an.next.outgoing = Some(edge_index);

        let mut bn = &mut self[b];
        bn.next.incoming = Some(edge_index);
        Ok(edge_index)
    }

    fn replace_edge_links_of_node(
        &mut self,
        node: NodeIndex<N, E>,
        replace: EdgeIndex<N, E>,
        with: Option<EdgeIndex<N, E>>,
        dir: Direction,
    ) {
        // let mut found = false;
        // println!("----");
        // println!("on node {}", node.debug());
        // println!("replace: {} ", replace.debug());

        // println!("with {} ", with.debug());
        // // println!("next {:?} ", next);
        // println!("dir {:?}", dir);

        let next = { self.nodes[node].next };
        // let nxt = next[dir];
        // println!("nxt {:?}", nxt);
        if next[dir] == Some(replace) {
            // found = true;
            self.nodes[node].next[dir] = with;
        } else {
            let mut edges = match dir {
                Direction::Outgoing => self.outgoing_edges_mut(node),
                Direction::Incoming => self.incoming_edges_mut(node),
            };

            while let Some((_, cur_edge)) = edges.next() {
                //
                // println!("cur_edge index {:?}", index);
                // println!("cur_edge {:?}", cur_edge);
                // println!("cur_edge.next[dir] {:?}", cur_edge.next[dir]);
                // println!("replace {:?}", replace);
                // println!("with {:?}", with);
                // println!("next[dir] {:?}", next[dir]);
                if cur_edge.next[dir] == Some(replace) {
                    debug_assert!(cur_edge.next[dir] != with);
                    // println!("replacing {} with {}", cur_edge.next[dir].debug(), with.debug());
                    // assert!(cur_edge.next[dir] != next[dir]);
                    cur_edge.next[dir] = with;
                    // found = true;
                    break;
                }
            }
        }
        // assert!(
        //     found,
        //     format!("failed to replace edge {:?} with {:?}", replace, with)
        // );
    }

    pub fn nodes(&self) -> &generational_arena::Arena<Node<N, E>> {
        &self.nodes
    }

    pub fn edges(&self) -> &generational_arena::Arena<Edge<N, E>> {
        &self.edges
    }

    pub fn incoming_neighbors(&self, node: NodeIndex<N, E>) -> IncomingNeighbors<N, E> {
        IncomingNeighbors::new(self, node)
    }

    pub fn outgoing_neighbors(&self, node: NodeIndex<N, E>) -> OutgoingNeighbors<N, E> {
        OutgoingNeighbors::new(self, node)
    }

    pub fn remove_edge(&mut self, e: EdgeIndex<N, E>) -> Option<E> {
        // println!("remove edge {}", e.debug());
        //
        let t = if let Some(edge) = self.edges.typed_get(e) {
            Some((edge.from(), edge.to(), edge.next))
        } else {
            None
        };

        if let Some((from, to, next)) = t {
            // println!(
            //     "removing edge from node {:?} to node {:?}",
            //     self.nodes[from].weight, self.nodes[to].weight
            // );
            self.replace_edge_links_of_node(from, e, next.outgoing, Direction::Outgoing);
            self.replace_edge_links_of_node(to, e, next.incoming, Direction::Incoming);

            Some(self.edges.typed_remove(e).unwrap().weight)
        } else {
            None
        }
    }

    pub fn remove_node(&mut self, n: NodeIndex<N, E>) -> Option<N> {
        if let Some(_) = self.nodes.typed_get(n) {
            // self.nodes.get(a.index())?;
            // for d in &DIRECTIONS {
            //     let k = d.index();

            //     // Remove all edges from and to this node.

            let mut i = 0;
            loop {
                let next = self.nodes[n].next;
                if let Some(outgoing) = next.outgoing {
                    // println!("removing outgoing edge {:?} {:?}", i, outgoing.debug());
                    let z = self.remove_edge(outgoing);
                    assert!(
                        z.is_some(),
                        format!(
                            "failed to remove outgoing edge {} of node {}",
                            outgoing.debug(),
                            n.debug()
                        )
                    );
                    // i += 1;
                    // if outgoing.index() == 8 {
                    //     break;
                    // }
                } else {
                    break;
                }
            }
            loop {
                let next = self.nodes[n].next;
                if let Some(incoming) = next.incoming {
                    // println!("removing incoming edge {:?}", incoming.debug());
                    let z = self.remove_edge(incoming);
                    assert!(
                        z.is_some(),
                        format!(
                            "failed to remove edge incoming {} of node {}",
                            incoming.debug(),
                            n.debug()
                        )
                    );
                } else {
                    break;
                }
            }
            // let next = self.nodes[n];
            // let next = self.nodes[a.index()].next[k];
            // if next == EdgeIndex::end() {
            //     break;
            // }
            // let ret = self.remove_edge(next);
            // debug_assert!(ret.is_some());
            // let _ = ret;
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
            let node = self.nodes.typed_remove(n).unwrap();
            Some(node.weight)
        } else {
            None
        }
    }

    pub fn get(&mut self, index: NodeIndex<N, E>) -> Option<&Node<N, E>> {
        self.nodes.typed_get(index)
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
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

impl<N, E> std::ops::Index<EdgeIndex<N, E>> for Graph<N, E> {
    type Output = Edge<N, E>;
    fn index(&self, index: EdgeIndex<N, E>) -> &Self::Output {
        &self.edges[index]
    }
}

impl<N, E> std::ops::IndexMut<EdgeIndex<N, E>> for Graph<N, E> {
    fn index_mut(&mut self, index: EdgeIndex<N, E>) -> &mut Self::Output {
        &mut self.edges[index]
    }
}

impl<N: std::fmt::Debug, E: std::fmt::Debug> std::fmt::Debug for Graph<N, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Graph {{\n")?;
        for (index, node) in self.nodes.iter() {
            write!(f, "{:?}\n\t{:?}\n", index, node)?;
        }

        for (index, edge) in self.edges.iter() {
            write!(f, "{:?}\n\t{:?}\n", index, edge)?;
        }
        write!(f, "}}")
    }
}
