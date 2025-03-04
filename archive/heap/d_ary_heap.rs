use itertools::Itertools;

/// A priority queue implemented with implicit simple D-ary heap.
///
/// This is a max heap.
#[derive(Debug, Clone)]
pub struct DAryHeap<T, const D: usize> {
    /// data[0] is the root node.
    data: Vec<T>,
}

impl<T: Ord, const D: usize> Default for DAryHeap<T, D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord, const D: usize> DAryHeap<T, D> {
    const fn assert_branching_factor() {
        assert!(D > 0, "branching factor `D` should be positive.");
    }

    /// See [`Vec::new`].
    ///
    /// # Panics
    ///
    /// Panics if `D = 0`.
    pub const fn new() -> Self {
        Self::assert_branching_factor();

        Self { data: Vec::new() }
    }

    /// See [`Vec::with_capacity`].
    ///
    /// # Panics
    ///
    /// Panics if `D = 0`.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::assert_branching_factor();

        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// See [`Vec::shrink_to`]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity);
    }

    /// See [`Vec::shrink_to_fit`]
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// See [`Vec::reserve_exact`].
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// See [`Vec::reserve_exact`].
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    /// See [`Vec::is_empty`].
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// See [`Vec::len`].
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// See [`Vec::as_slice`]
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Consumes the `QuadHeap` and returns the underlying vector in arbitrary order.
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Returns an iterator visiting all values in the underlying vector, in arbitrary order.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    /// See [`Vec::drain`]
    pub fn drain(&mut self) -> std::vec::Drain<'_, T> {
        self.data.drain(..)
    }

    /// See [`Vec::clear`]
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T: Ord, const D: usize> DAryHeap<T, D> {
    /// # Example
    ///
    /// ```
    /// use alds::heap::DAryHeap;
    ///
    /// let mut heap = DAryHeap::<_, 8>::new();
    ///
    /// heap.push(100);
    /// heap.push(200);
    /// heap.push(300);
    ///
    /// assert_eq!(heap.pop(), Some(300));
    /// assert_eq!(heap.pop(), Some(200));
    /// assert_eq!(heap.pop(), Some(100));
    /// assert!(heap.pop().is_none());
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(log_D *n*)
    pub fn push(&mut self, item: T) {
        self.data.push(item);

        // maintain consistency
        let mut c = self.data.len() - 1;
        while c > 0 {
            let p = (c - 1) / D;

            if self.data[p] >= self.data[c] {
                break;
            }

            self.data.swap(p, c);
            c = p;
        }
    }

    /// # Example
    ///
    /// ```
    /// use alds::heap::DAryHeap;
    ///
    /// let mut heap = DAryHeap::<_, 8>::with_capacity(3);
    ///
    /// heap.push(100);
    /// heap.push(200);
    /// heap.push(300);
    ///
    /// assert_eq!(heap.peek(), Some(&300));
    /// assert_eq!(heap.peek(), Some(&300));
    /// assert_eq!(heap.peek(), Some(&300));
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    /// # Example
    ///
    /// ```
    /// use alds::heap::DAryHeap;
    ///
    /// let mut heap = DAryHeap::<_, 8>::from(vec![1, 3, 5, 7, 9, -8, -6, -4, -2, 0]);
    ///
    /// assert_eq!(
    ///     Vec::from_iter(std::iter::from_fn(|| heap.pop())),
    ///     vec![9, 7, 5, 3, 1, 0, -2, -4, -6, -8],
    /// );
    /// assert!(heap.is_empty());
    /// ```
    /// # Time complexity
    ///
    /// O(*D* log_D *n*)
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let res = self.data.swap_remove(0);
        // maintain consistency
        self.shift_down(0);

        Some(res)
    }

    /// If *i* is out of bounds, do nothing.
    ///
    /// # Time complexity
    ///
    /// O(*D* log_D *n*)
    fn shift_down(&mut self, i: usize) {
        let mut p = i;

        while let Some(max_c) = self
            .data
            .get(D * p + 1..)
            .and_then(|children| children.iter().take(D).position_max())
        {
            let c = D * p + 1 + max_c;

            if self.data[p] >= self.data[c] {
                break;
            }

            self.data.swap(p, c);
            p = c
        }
    }
}

impl<T: Ord, const D: usize> From<Vec<T>> for DAryHeap<T, D> {
    /// # Time complexity
    ///
    /// *O*(*n* / *D*)
    fn from(vec: Vec<T>) -> Self {
        assert!(D > 0, "`D` should be positive");

        let mut heap = Self { data: vec };

        // since sum_(k=0)^d k D^(d - k) ~ D^(d-1), where d := ilog_D(n),
        // time complexity is *O*(*n* / *D*) for D-ary heap.
        for i in (0..heap.len() / D).rev() {
            heap.shift_down(i);
        }

        heap
    }
}
