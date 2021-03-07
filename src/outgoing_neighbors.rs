use crate::prelude::*;

pub struct OutgoingNeighbors<'a, N, E> {
    index: NodeIndex<N, E>,
    inner: &'a Graph<N, E>,
    next: Next<N, E>,
}

impl<'a, N, E> OutgoingNeighbors<'a, N, E> {
    pub fn new(inner: &'a Graph<N, E>, index: NodeIndex<N, E>) -> Self {
        Self {
            index,
            inner,
            next: inner[index].next,
        }
    }
}

impl<'a, N, E> Iterator for OutgoingNeighbors<'a, N, E> {
    type Item = NodeIndex<N, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.next.outgoing {
            let node = self.inner[index].to();
            self.next = self.inner[node].next;
            Some(node)
        } else {
            None
        }
    }
}

// match self.edges.get(self.next[0].index()) {
//     None => {}
//     Some(edge) => {
//         self.next[0] = edge.next[0];
//         return Some(edge.nodes[1]);
//     }
// }
// // Then incoming edges
// // For an "undirected" iterator (traverse both incoming
// // and outgoing edge lists), make sure we don't double
// // count selfloops by skipping them in the incoming list.
// while let Some(edge) = self.edges.get(self.next[1].index()) {
//     self.next[1] = edge.next[1];
//     if edge.nodes[0] != self.skip_start {
//         return Some(edge.nodes[0]);
//     }
// }
// None