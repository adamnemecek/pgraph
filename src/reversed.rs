use crate::prelude::*;

pub struct Reversed<'a, N, E> {
    inner: &'a Graph<N, E>,
}

impl<'a, N, E> Reversed<'a, N, E> {
    pub fn new(inner: &'a Graph<N, E>) -> Self {
        Self { inner }
    }
}

impl<'a, N, E> GraphKind<N, E> for Reversed<'a, N, E> {
    fn neighbors(&self, node: NodeIndex<N, E>) -> Neighbors<N, E> {
        todo!()
    }
}
