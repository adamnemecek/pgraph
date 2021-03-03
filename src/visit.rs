pub trait VisitMap<N> {
    /// Mark `a` as visited.
    ///
    /// Return **true** if this is the first visit, false otherwise.
    fn visit(&mut self, a: N) -> bool;

    /// Return whether `a` has been visited before.
    fn is_visited(&self, a: &N) -> bool;
}