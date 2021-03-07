use crate::prelude::*;

pub struct Neighbors<'a, N, E> {
    index: NodeIndex<N, E>,
    inner: &'a Graph<N, E>,
    next: Next<N, E>
}

impl<'a, N, E> Neighbors<'a, N, E> {
    pub fn new(inner: &'a Graph<N, E>, index: NodeIndex<N, E>) -> Self {
        Self { index, inner, next: inner[index].next }
    }
}

impl<'a, N, E> Iterator for Neighbors<'a, N, E> {
    type Item = NodeIndex<N, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.next.outgoing {
            None
        } else {
            None
        }
    }
}
