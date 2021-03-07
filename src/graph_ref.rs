use crate::prelude::*;
pub struct GraphRef<'a, N, E> {
    inner: &'a Graph<N, E>,
}

impl<'a, N, E> GraphRef<'a, N, E> {
    pub fn new(inner: &'a Graph<N, E>) -> Self {
        Self { inner }
    }
}

impl<'a, N: std::fmt::Debug, E: std::fmt::Debug> GraphKind<N, E> for GraphRef<'a, N, E> {
    type Neighbors = OutgoingNeighbors<'a, N, E>;
    fn neighbors(&self, node: NodeIndex<N, E>) -> Self::Neighbors {
        self.inner.outgoing_neighbors(node)
    }
}
