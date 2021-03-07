use crate::prelude::*;

pub struct Reversed<'a, N, E> {
    inner: &'a Graph<N, E>
}

impl<'a, N, E> Reversed<'a, N, E> {
    pub fn new(g: &Graph<N,E>) -> Self {
        todo!()
    }
}

impl<'a, N,E> GraphKind<N, E> for Reversed<'a, N, E> {
    fn neighbors(&self, node: NodeIndex<N, E>) -> Neighbors<N, E> {
        todo!()
    }   
}