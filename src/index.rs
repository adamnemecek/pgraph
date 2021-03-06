use crate::prelude::*;
pub type NodeIndex<N, E> = generational_arena::TypedIndex<Node<N, E>>;
pub type EdgeIndex<N, E> = generational_arena::TypedIndex<Edge<N, E>>;

pub(crate) trait DebugExt {
    fn debug(&self) -> String;
}

impl<T> DebugExt for generational_arena::TypedIndex<T> {
    fn debug(&self) -> String {
        format!(
            "index: {:?}, generation: {:?}",
            self.index(),
            self.generation()
        )
    }
}

impl<T> DebugExt for Option<generational_arena::TypedIndex<T>> {
    fn debug(&self) -> String {
        if let Some(inner) = self {
            inner.debug()
        } else {
            "None".to_string()
        }
    }
}
