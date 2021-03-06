#[deny(unused_must_use)]
mod algo;
pub use algo::*;

mod edge;
pub use edge::*;

mod edges;
pub use edges::*;

mod edges_mut;
pub use edges_mut::*;

mod error;
pub use error::*;

mod graph_kind;
pub use graph_kind::*;

mod stack;
pub use stack::*;

mod graph_ref;
pub use graph_ref::*;

mod incoming_neighbors;
pub use incoming_neighbors::*;

mod index;
pub use index::*;

mod graph;
pub use graph::*;

mod next;
pub use next::*;

mod node;
pub use node::*;

mod outgoing_neighbors;
pub use outgoing_neighbors::*;

mod traversal;
pub use traversal::*;

mod visit;
pub use visit::*;

mod reversed;
pub use reversed::*;

mod tree;
pub use tree::*;

pub mod prelude;

// Index into the NodeIndex and EdgeIndex arrays
/// Edge direction.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    /// An `Outgoing` edge is an outward edge *from* the current node.
    Outgoing = 0,
    /// An `Incoming` edge is an inbound edge *to* the current node.
    Incoming = 1,
}
