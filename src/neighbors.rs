use crate::prelude::*;

pub struct Neighbors<N, E> {
    inner: NodeIndex<N, E>,
}

impl<N, E> Iterator for Neighbors<N, E> {
    type Item = NodeIndex<N, E>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
