use crate::prelude::*;

pub struct Reversed<'a, N, E> {
    inner: &'a Graph<N, E>,
}

impl<'a, N, E> Reversed<'a, N, E> {
    pub fn new(inner: &'a Graph<N, E>) -> Self {
        Self { inner }
    }
}

impl<'a, N: std::fmt::Debug, E: std::fmt::Debug> GraphKind<N, E> for Reversed<'a, N, E> {
    type Neighbors = IncomingNeighbors<'a, N, E>;
    fn neighbors(&self, node: NodeIndex<N, E>) -> Self::Neighbors {
        Self::Neighbors::new(self.inner, node)
    }
}
