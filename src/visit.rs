use fixedbitset::FixedBitSet;
use generational_arena::TypedIndex;

// pub trait VisitMap<N> {
//     /// Mark `a` as visited.
//     ///
//     /// Return **true** if this is the first visit, false otherwise.
//     fn visit(&mut self, a: TypedIndex<N>) -> bool;

//     /// Return whether `a` has been visited before.
//     fn is_visited(&self, a: TypedIndex<N>) -> bool;
// }

// impl<T> VisitMap<T> for fixedbitset::FixedBitSet {
//     fn visit(&mut self, x: TypedIndex<T>) -> bool {
//         !self.put(x.index())
//     }

//     fn is_visited(&self, x: TypedIndex<T>) -> bool {
//         self.contains(x.index())
//     }
// }

#[derive(Clone, Debug)]
pub struct VisitMap<N> {
    /// Mark `a` as visited.
    ///
    /// Return **true** if this is the first visit, false otherwise.
    inner: fixedbitset::FixedBitSet,
    ph: std::marker::PhantomData<N>,
}

impl<T: std::fmt::Debug> VisitMap<TypedIndex<T>> {
    pub fn with_capacity(bits: usize) -> Self {
        Self {
            inner: fixedbitset::FixedBitSet::with_capacity(bits),
            ph: Default::default(),
        }
    }

    pub fn visit(&mut self, x: TypedIndex<T>) -> bool {
        !self.inner.put(x.index())
    }

    pub fn is_visited(&self, x: TypedIndex<T>) -> bool {
        self.inner.contains(x.index())
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn grow(&mut self, bits: usize) {
        self.inner.grow(bits);
    }
}
