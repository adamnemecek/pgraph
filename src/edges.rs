use crate::prelude::*;

pub struct Edges<'a, T> {
    inner: &'a generational_arena::Arena<T>,
    direction: Direction,
}

impl<'a, T> Edges<'a, T> {
    fn new(direction: Direction, inner: &'a generational_arena::Arena<T>) -> Self {
        Self {
            direction,
            inner,
        }
    }
}

impl<'a, T> Iterator for Edges<'a, T> {
    type Item = T;
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.len(), None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

}