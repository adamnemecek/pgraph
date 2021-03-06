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
        for (a, b) in self.nodes.iter().zip(other.nodes.iter()) {
            if a.0 != b.0 {
                return false;
            }
        }

        if self.edges.len() != other.edges.len() {
            return false;
        }
        for (a, b) in self.edges.iter().zip(other.edges.iter()) {
            if a.0 != b.0 {
                return false;
            }
        }
        true
    }
}

impl<N: Eq, E: Eq> Eq for Graph<N, E> { }

// impl<N: Clone, E: Clone> Clone for Graph<N, E> {
//     fn clone(&self) -> Self {
//         Self {
//             edges: self.edges.clone(),
//             nodes: self.nodes.clone(),
//         }
//     }
// }

impl<N: std::fmt::Debug, E: std::fmt::Debug> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            edges: Default::default(),
        }
    }

    pub fn add_node(&mut self, weight: N) -> NodeIndex<N, E> {
        let node = Node {
            weight,
            // next: Default::default(),
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

    pub fn visit_map(&self) -> fixedbitset::FixedBitSet {
        fixedbitset::FixedBitSet::with_capacity(self.node_count())
    }

    pub fn reset_map(&self, map: &mut fixedbitset::FixedBitSet) {
        map.clear();
        map.grow(self.node_count());
    }

    pub fn add_edge(
        &mut self,
        a: NodeIndex<N, E>,
        b: NodeIndex<N, E>,
        weight: E,
    ) -> EdgeIndex<N, E> {
        assert!(a != b);

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
        edge_index
    }

    fn change_edge_link(&mut self, node: NodeIndex<N, E>, dir: Direction) {
        let fst = &self.nodes[node];
        if false {
            //todo!()
        } else {
            // let mut edges = EdgesMut::new(Direction::Incoming, fst, dir)
            // while let Some(cur_edge) = edges.next() {
            //     // if cur_edge.next[dir] == e {
            //         todo!()
            //     // }
            // }
        }
    }

    // iterate through the list of
    // fn replace_outgoing_edge_links_of_node(
    //     &mut self,
    //     node: NodeIndex<N, E>,
    //     replace: EdgeIndex<N, E>,
    //     with: EdgeIndex<N, E>,
    //     dir: Direction,
    // ) {
    //     let node = &mut self[node];
    //     loop {
    //         if let Some(next) = node.next.outgoing {
    //             if next == replace {
    //                 node.next.outgoing = Some(with);
    //             }
    //         } else {
    //         }
    //         // let edge = node.next[dir];
    //         // let fst = node.next[dir].expect("fdas");

    //         // if fst == replace {
    //         //
    //         // } else {
    //         // let edges = EdgesMut::new(dir, node.next, &mut self.edges);
    //         // }
    //     }

    //     todo!()
    // }

    fn replace_edge_links_of_node(
        &mut self,
        node: NodeIndex<N, E>,
        replace: EdgeIndex<N, E>,
        with: Option<EdgeIndex<N, E>>,
        dir: Direction,
    ) {
        println!("replace: {} ", replace.debug());
        println!("with {:?} ", with);
        // println!("next {:?} ", next);
        println!("dir {:?}", dir);

        let next = { self.nodes[node].next };
        if next[dir] == Some(replace) {
            self.nodes[node].next[dir] = with;
        } else {
            let mut edges = match dir {
                Direction::Outgoing => self.outgoing_edges_mut(node),
                Direction::Incoming => self.incoming_edges_mut(node),
            };

            while let Some((index, cur_edge)) = edges.next() {
                //
                println!("cur_edge index {:?}", index);
                println!("cur_edge {:?}", cur_edge);
                println!("cur_edge.next[dir] {:?}", cur_edge.next[dir]);
                println!("replace {:?}", replace);
                println!("with {:?}", with);
                println!("next[dir] {:?}", next[dir]);
                if index == replace {
                    // assert!(cur_edge.next[dir] != next[dir]);
                    cur_edge.next[dir] = next[dir];
                    break;
                }
            }
        }
    }

    pub fn remove_edge(&mut self, e: EdgeIndex<N, E>) -> Option<E> {
        //
        let t = if let Some(edge) = self.edges.typed_get(e) {
            Some((edge.from(), edge.to(), edge.next))
        } else {
            None
        };

        if let Some((from, to, next)) = t {
            self.replace_edge_links_of_node(from, e, next.outgoing, Direction::Outgoing);
            self.replace_edge_links_of_node(to, e, next.incoming, Direction::Incoming);

            Some(self.edges.typed_remove(e).unwrap().weight)
        } else {
            None
        }
    }

    pub fn remove_node(&mut self, n: NodeIndex<N, E>) -> Option<N> {
        if let Some(node) = self.nodes.typed_remove(n) {
            // self.nodes.get(a.index())?;
            // for d in &DIRECTIONS {
            //     let k = d.index();

            //     // Remove all edges from and to this node.
            let next = node.next;
            loop {
                if let Some(outgoing) = next.outgoing {
                    self.remove_edge(outgoing);
                } else {
                    break;
                }
            }
            loop {
                if let Some(incoming) = next.incoming {
                    self.remove_edge(incoming);
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
