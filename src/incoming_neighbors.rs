use crate::prelude::*;

pub struct IncomingNeighbors<'a, N, E> {
    index: NodeIndex<N, E>,
    inner: &'a Graph<N, E>,
    // next: Next<N, E>,
    edges: Edges<'a, N, E>,
}

impl<'a, N: std::fmt::Debug, E: std::fmt::Debug> IncomingNeighbors<'a, N, E> {
    pub fn new(inner: &'a Graph<N, E>, index: NodeIndex<N, E>) -> Self {
        Self {
            index,
            inner,
            // next: inner[index].next,
            edges: inner.incoming_edges(index)
        }
    }
}

impl<'a, N, E> Iterator for IncomingNeighbors<'a, N, E> {
    type Item = NodeIndex<N, E>;
    // fn next(&mut self) -> Option<Self::Item> {
    //     if let Some(index) = self.next.incoming {
    //         let node = self.inner[index].from();
    //         self.next = self.inner[node].next;
    //         Some(node)
    //     } else {
    //         None
    //     }
    // }
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, edge)) = self.edges.next() {
            Some(edge.from())
        } else {
            None
        }
    }
}
