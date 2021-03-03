use crate::prelude::*;

pub struct Edges<'a, N, E> {
    inner: &'a generational_arena::Arena<Edge<N, E>>,
    direction: Direction,
    next: [Option<EdgeIndex<N, E>>; 2],
}

impl<'a, N, E> Edges<'a, N, E> {
    pub(crate) fn new(
        direction: Direction,
        next: [Option<EdgeIndex<N, E>>; 2],
        inner: &'a generational_arena::Arena<Edge<N, E>>,
    ) -> Self {
        Self {
            direction,
            next,
            inner,
        }
    }
}

impl<'a, N, E> Iterator for Edges<'a, N, E> {
    type Item = N;
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.len(), None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
