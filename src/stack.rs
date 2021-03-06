#[derive(Clone, Debug)]
pub struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn push(&mut self, item: T) {
        self.inner.push(item)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.last()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;
    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        assert!(s.peek() == Some(&2));
        assert!(s.pop() == Some(2));
        assert!(s.pop() == Some(1));
    }
}
