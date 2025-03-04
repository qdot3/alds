/// A priority queue implemented with a (lazy) binomial heap, which supports efficient `push` operation.
///
/// This is a max heap.
#[derive(Debug, Clone)]
pub struct BinomialHeap<T> {
    // `arena[0]` is the root
    arena: Vec<Box<BinomialTree<T>>>,
    size: usize,
}

impl<T> Default for BinomialHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinomialHeap<T> {
    /// Creates an empty binomial heap.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::heap::BinomialHeap;
    ///
    /// let heap0 = BinomialHeap::<()>::new();
    /// assert!(heap0.is_empty());
    ///
    /// // more efficient way
    /// let mut heap1 = BinomialHeap::from(vec![2, 3, 5, 7, 11]);
    /// assert_eq!(heap1.pop(), Some(11));
    ///
    /// let heap2 = BinomialHeap::from_iter(0..100);
    /// assert_eq!(heap2.size(), 100);
    /// ```
    pub const fn new() -> Self {
        Self {
            arena: vec![],
            size: 0,
        }
    }

    /// Returns the number of elements in the binomial heap.
    ///
    /// Binomial Heap is a forest, so the name `len` is not suitable.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::heap::BinomialHeap;
    ///
    /// let mut heap = BinomialHeap::new();
    /// assert_eq!(heap.size(), 0);
    ///
    /// heap.push(100);
    /// heap.push(200);
    ///
    /// assert_eq!(heap.size(), 2);
    /// ```
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Checks if the binomial heap is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::heap::BinomialHeap;
    ///
    /// let mut heap = BinomialHeap::new();
    /// assert!(heap.is_empty());
    ///
    /// heap.extend(vec![2, 3, 5]);
    /// assert!(!heap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    /// Returns the minimum element, or `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::heap::BinomialHeap;
    ///
    /// let mut heap = BinomialHeap::new();
    ///
    /// assert!(heap.peek().is_none());
    ///
    /// heap.extend(0..10);
    ///
    /// assert_eq!(heap.peek(), Some(&9));
    /// assert_eq!(heap.peek(), Some(&9));
    /// assert_eq!(heap.peek(), Some(&9));
    /// ```
    /// # Time complexity
    ///
    /// *O*(1)
    pub fn peek(&self) -> Option<&T> {
        self.arena.first().map(|node| node.peek())
    }
}

impl<T: Ord> BinomialHeap<T> {
    /// Pushes an item onto the binomial heap.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::heap::BinomialHeap;
    ///
    /// let mut heap = BinomialHeap::new();
    /// assert!(heap.is_empty());
    ///
    /// heap.push(100);
    /// assert_eq!(heap.peek(), Some(&100));
    ///
    /// heap.push(200);
    /// assert_eq!(heap.pop(), Some(200));
    /// assert_eq!(heap.pop(), Some(100));
    /// assert!(heap.is_empty());
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    pub fn push(&mut self, value: T) {
        let Self { arena, size } = self;

        // lazy implementation
        arena.push(Box::new(BinomialTree::new(value)));
        *size += 1;

        // `arena[0]` is the root
        if arena.len() >= 2 {
            let n = arena.len() - 1;
            if arena[0].peek() < arena[n].peek() {
                arena.swap(0, n);
            }
        }
    }

    /// Removes the maximum element, or `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::heap::BinomialHeap;
    ///
    /// let mut heap = BinomialHeap::from_iter(5..15);
    /// assert_eq!(heap.pop(), Some(14));
    ///
    /// heap.push(100);
    /// assert_eq!(heap.peek(), Some(&100));
    /// assert_eq!(heap.pop(), Some(100));
    /// assert_eq!(heap.pop(), Some(13));
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(log *n*), amortized.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let (root, siblings) = self.arena.swap_remove(0).pop();
        self.arena.extend(siblings);
        self.size -= 1;

        if self.is_empty() {
            return Some(root);
        }

        // melding
        let mut new_arena = Vec::from_iter(
            std::iter::repeat_with(|| None::<Box<BinomialTree<T>>>)
                .take(self.size().ilog2() as usize + 1),
        );
        for mut one in self.arena.drain(..) {
            loop {
                let i = one.order();
                if let Some(other) = std::mem::take(&mut new_arena[i]) {
                    assert!(one.merge(*other).is_ok())
                } else {
                    new_arena[i] = Some(one);
                    break;
                }
            }
        }

        assert!(self.arena.is_empty());
        let mut new_arena = new_arena.into_iter().skip_while(|v| v.is_none());
        if let Some(mut max_v) = new_arena.next().and_then(|v| v) {
            for mut node in new_arena.flatten() {
                if node.peek() > max_v.peek() {
                    std::mem::swap(&mut node, &mut max_v);
                }
                self.arena.push(node);
            }

            // `self.arena[0]` is the root.
            let i = self.arena.len();
            self.arena.push(max_v);
            self.arena.swap(i, 0);
        }

        Some(root)
    }
}

impl<T: Ord> Extend<T> for BinomialHeap<T> {
    fn extend<U: IntoIterator<Item = T>>(&mut self, iter: U) {
        let Self { arena, size } = self;

        let n = arena.len();
        arena.extend(
            iter.into_iter()
                .map(|value| Box::new(BinomialTree::new(value))),
        );
        *size += arena.len() - n;

        // `self.arena[0]` is the root.
        if !arena.is_empty() {
            let mut i = 0;
            for j in n..arena.len() {
                if arena[j].peek() > arena[i].peek() {
                    i = j
                }
            }
            arena.swap(i, 0);
        }
    }
}

impl<U: Ord> FromIterator<U> for BinomialHeap<U> {
    fn from_iter<T: IntoIterator<Item = U>>(iter: T) -> Self {
        let mut heap = Self::new();
        heap.extend(iter);

        heap
    }
}

impl<T: Ord> From<Vec<T>> for BinomialHeap<T> {
    fn from(value: Vec<T>) -> Self {
        Self::from_iter(value)
    }
}

/// Prioritized binomial tree.
#[derive(Debug, Clone)]
struct BinomialTree<T> {
    value: T,
    order: usize,
    child: Option<Box<BinomialTree<T>>>,
    sibling: Option<Box<BinomialTree<T>>>,
}

impl<T> BinomialTree<T> {
    /// Returns singleton.
    const fn new(value: T) -> Self {
        Self {
            value,
            order: 0,
            child: None,
            sibling: None,
        }
    }

    const fn order(&self) -> usize {
        self.order
    }

    const fn peek(&self) -> &T {
        &self.value
    }
}

impl<T: Ord> BinomialTree<T> {
    /// Returns the root and children.
    ///
    /// # Panics
    ///
    /// Panics if given nodes is invalid.
    fn pop(self) -> (T, Vec<Box<Self>>) {
        let Self {
            value,
            order,
            mut child,
            sibling,
        } = self;

        assert!(sibling.is_none());

        let mut children = Vec::with_capacity(order);
        while let Some(mut c) = child {
            let sibling = std::mem::take(&mut c.sibling);
            children.push(c);
            child = sibling
        }

        (value, children)
    }

    /// Merge two
    ///
    /// # Panics
    ///
    /// Panics if given nodes is invalid.
    fn merge(&mut self, mut other: Self) -> Result<(), Self> {
        if self.order != other.order {
            return Err(other);
        }

        assert!(self.sibling.is_none());
        assert!(other.sibling.is_none());

        if self.value < other.value {
            std::mem::swap(self, &mut other);
        }

        // `self` takes priority over `other`.
        self.order += 1;

        let child = std::mem::take(&mut self.child);
        other.sibling = child;
        self.child = Some(Box::new(other));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordering() {
        let mut heap = BinomialHeap::from_iter(0..100);

        assert_eq!(
            Vec::from_iter(std::iter::repeat_with(|| heap.pop().unwrap()).take(100)),
            Vec::from_iter((0..100).rev())
        );
        assert!(heap.is_empty())
    }

    #[test]
    fn test_merge_node() {
        const BIT: usize = 10;

        let mut heap = BinomialHeap::from_iter(0..1 << BIT);
        while heap.pop().is_some() {
            assert!(heap.arena.len() <= BIT);
        }
    }
}
