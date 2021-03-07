use crate::prelude::*;

pub struct Neighbors<'a, N, E> {
    // inner: NodeIndex<N, E>,
    inner: &'a Graph<N, E>,
}

impl<'a, N, E> Neighbors<'a, N, E> {
    pub fn new(inner: &'a Graph<N, E>) -> Self {
        Self { inner }
    }
}

impl<'a, N, E> Iterator for Neighbors<'a, N, E> {
    type Item = NodeIndex<N, E>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
