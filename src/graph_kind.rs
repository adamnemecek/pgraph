use crate::prelude::*;
pub trait GraphKind<N, E> {
    type Neighbors: Iterator<Item = NodeIndex<N, E>>;
    fn neighbors(&self, node: NodeIndex<N, E>) -> Self::Neighbors;
}
