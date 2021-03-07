use crate::prelude::*;
pub trait GraphKind<N, E> {
    fn neighbors(&self, node: NodeIndex<N, E>) -> Neighbors<N, E>;
}
