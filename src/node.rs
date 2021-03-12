use crate::prelude::*;

// #[derive(Debug)]
#[derive(Clone, PartialEq, Eq)]
pub struct Node<N, E> {
    /// Associated node data.
    pub weight: N,
    /// Next edge in outgoing and incoming edge lists.
    pub(crate) next: Next<N, E>,
}

impl<N: std::fmt::Debug, E: std::fmt::Debug> std::fmt::Debug for Node<N, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{\n")?;
        write!(f, "\tweight: {:?}\n", self.weight)?;
        write!(f, "\t  next: {:?},\n", self.next)?;
        write!(f, "}}\n")
    }
}

// impl<N, E> std::ops::Deref for Node<N, E> {
//     type Target = N;
//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.weight
//     }
// }

// impl<N, E> std::ops::DerefMut for Node<N, E> {
//     #[inline]
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.weight
//     }
// }
