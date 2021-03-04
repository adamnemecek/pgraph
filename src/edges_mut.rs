use crate::prelude::*;

pub struct EdgesMut<'a, N: 'a, E: 'a> {
    edges: &'a mut generational_arena::Arena<Edge<N, E>>,
    direction: Direction,
    next: Next<N, E>,
}

impl<'a, N: 'a, E: 'a> EdgesMut<'a, N, E> {
    pub(crate) fn new(
        direction: Direction,
        next: Next<N, E>,
        edges: &'a mut generational_arena::Arena<Edge<N, E>>,
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

    pub fn next(&'a mut self) -> Option<&'a mut Edge<N, E>> {
        // todo!()

        if let Some(idx) = self.next[self.direction] {
            let edge = &mut self.edges[idx];
            self.next = edge.next;
            Some(edge)
        } else {
            None
        }
    }
}
