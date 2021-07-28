use crate::prelude::*;
use generational_arena::TypedArena;

pub struct EdgesMut<'a, N: 'a, E: 'a> {
    edges: &'a mut TypedArena<Edge<N, E>>,
    next: Next<N, E>,
    direction: Direction,
}

impl<'a, N: 'a, E: 'a> EdgesMut<'a, N, E> {
    pub(crate) fn new(
        direction: Direction,
        next: Next<N, E>,
        edges: &'a mut TypedArena<Edge<N, E>>,
    ) -> Self {
        Self {
            direction,
            next,
            edges,
        }
    }
}

impl<'a, N: 'a, E: 'a> EdgesMut<'a, N, E> {
    // type Item = &'a mut Edge<N, E>;
    // fn size_hint(&self) -> (usize, Option<usize>) {
    //     (self.edges.len(), None)
    // }

    pub fn next(&mut self) -> Option<(EdgeIndex<N, E>, &mut Edge<N, E>)> {
        let next = self.next[self.direction];

        if let Some(idx) = next {
            let edge = &mut self.edges[idx];
            self.next = edge.next;
            Some((idx, edge))
        } else {
            None
        }
    }
}
