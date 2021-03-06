use crate::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GraphError<N, E> {
    WouldCycle(NodeIndex<N, E>),
}

// impl Error for GraphError {

// }
