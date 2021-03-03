use generational_arena::TypedIndex;

pub trait VisitMap<N> {
    /// Mark `a` as visited.
    ///
    /// Return **true** if this is the first visit, false otherwise.
    fn visit(&mut self, a: TypedIndex<N>) -> bool;

    /// Return whether `a` has been visited before.
    fn is_visited(&self, a: TypedIndex<N>) -> bool;
}

impl<T> VisitMap<T> for fixedbitset::FixedBitSet
// where
    // Ix: IndexType,
{
    fn visit(&mut self, x: TypedIndex<T>) -> bool {
        !self.put(x.index())
    }

    fn is_visited(&self, x: TypedIndex<T>) -> bool {
        self.contains(x.index())
    }
}