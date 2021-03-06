use crate::prelude::*;

#[derive(PartialEq, Eq)]
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

impl<N, E> std::fmt::Debug for Next<N, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "Next {{outgoing: {:?}, incoming: {:?}}}",
        //     self.outgoing, self.incoming
        // )
        write!(f, "Next {{\n")?;
        write!(f, "\toutgoing: {}\n", self.outgoing.debug())?;
        write!(f, "\tincoming: {},\n", self.incoming.debug())?;
        write!(f, "\t}}\n")
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

impl<N, E> std::ops::IndexMut<Direction> for Next<N, E> {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::Outgoing => &mut self.outgoing,
            Direction::Incoming => &mut self.incoming,
        }
    }
}
