use crate::prelude::*;

pub struct Edges<'a, N, E> {
    edges: &'a generational_arena::Arena<Edge<N, E>>,
    direction: Direction,
    next: Next<N, E>,
}

impl<'a, N, E> Edges<'a, N, E> {
    pub(crate) fn new(
        direction: Direction,
        next: Next<N, E>,
        edges: &'a generational_arena::Arena<Edge<N, E>>,
    ) -> Self {
        Self {
            direction,
            next,
            edges,
        }
    }
}

impl<'a, N, E> Iterator for Edges<'a, N, E> {
    type Item = N;
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.edges.len(), None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
