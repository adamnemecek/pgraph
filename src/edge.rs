use crate::prelude::*;

/// The graph's edge type.
#[derive(Debug)]
pub struct Edge<N, E> {
    /// Associated edge data.
    pub weight: E,
    /// Next edge in outgoing and incoming edge lists.
    // next: [Option<EdgeIndex<N, E>>; 2],
    // next_outgoing: Option<EdgeIndex<N, E>>,
    // next_incoming: Option<EdgeIndex<N, E>>,
    pub(crate) next: Next<N, E>,
    /// Start and End node index
    // node: [NodeIndex<N, E>; 2],
    pub(crate) from: NodeIndex<N, E>,
    pub(crate) to: NodeIndex<N, E>,
}

impl<N, E> Edge<N, E> {
    pub fn from(&self) -> NodeIndex<N, E> {
        self.from
    }

    pub fn to(&self) -> NodeIndex<N, E> {
        self.to
    }
}

// impl<N, E: std::fmt::Debug> std::fmt::Debug for Edge<N, E> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Edge {{weight: {:?}, next: {:?}, from: {:?}, to: {:?}}}", self.weight, self.next, self.from, self.to)
//     }
// }

impl<N, E> std::ops::Index<Direction> for Edge<N, E> {
    type Output = NodeIndex<N, E>;
    fn index(&self, dir: Direction) -> &Self::Output {
        match dir {
            Direction::Outgoing => &self.to,
            Direction::Incoming => &self.from,
        }
    }
}

// impl<N, E> std::ops::IndexMut<Direction> for Edge<N, E> {
//     fn index_mut(&mut self, dir: Direction) -> &mut Self::Output {
//         match dir {
//             Direction::Outgoing => &mut self.to,
//             Direction::Incoming => &mut self.from,
//         }
//     }
// }
