use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Next<N, E> {
    pub(crate) outgoing: Option<EdgeIndex<N, E>>,
    pub(crate) incoming: Option<EdgeIndex<N, E>>,
}

impl<N, E> Clone for Next<N, E> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<N, E> Copy for Next<N, E> {}

impl<N, E> Default for Next<N, E> {
    fn default() -> Self {
        Self {
            outgoing: None,
            incoming: None,
        }
    }
}