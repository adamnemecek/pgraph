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

impl<N, E> std::ops::Index<Direction> for Next<N, E> {
    type Output = Option<EdgeIndex<N, E>>;
    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Outgoing => &self.outgoing,
            Direction::Incoming => &self.incoming,
        }
    }
}
