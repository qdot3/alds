use std::ops::RangeBounds;

use crate::Monoid;

/// A segment tree that supports range updates and point queries.
///
/// # Performs binary operations in reversed order.
///
/// Define [Monoid::binary_operation] in reversed order: `rhs ∘ self` instead of `self ∘ rhs`.
///
/// # Multiple operations
///
#[derive(Debug, Clone)]
pub struct DualSegmentTree<T: Monoid> {
    /// one-based indexing
    lazy: Box<[T]>,
    /// true size
    len: usize,
    /// size of buffer for lazy propagation
    buf_len: usize,
}

impl<T: Monoid> DualSegmentTree<T> {
    /// Converts index of `data` to the corresponding index of `lazy`.
    const fn inner_index(&self, i: usize) -> usize {
        self.buf_len + i
    }

    /// Returns `[l, r)`.
    fn inner_range<R>(&self, range: R) -> (usize, usize)
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.len,
        };

        (self.inner_index(l), self.inner_index(r))
    }

    /// Propagates pending operations to the children.
    ///
    /// # Panics
    ///
    /// Assumes two children exist.
    fn propagate(&mut self, i: usize) {
        let lazy = std::mem::replace(&mut self.lazy[i], T::identity());
        self.lazy[i << 1] = lazy.binary_operation(&self.lazy[i << 1]);
        self.lazy[(i << 1) | 1] = lazy.binary_operation(&self.lazy[(i << 1) | 1]);
    }

    /// Updates elements in the given `range` using the binary operation defined in the [Monoid] trait.
    /// More precisely, performs `a[i] <- elem ∘ a[i]` for each `i` in the range.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_update<R>(&mut self, range: R, elem: T)
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.inner_range(range);
        if l >= r {
            return;
        }

        // In the case of a commutative binary operation, the order of computation can be arbitrary
        // without affecting the result.
        // So we can skip propagation.
        if !T::IS_COMMUTATIVE {
            // propagate pending operations
            for d in (1..=self.buf_len.trailing_zeros()).rev() {
                if (l >> d) << d != l {
                    self.propagate(l >> d);
                }
                if (r >> d) << d != r {
                    self.propagate((r - 1) >> d);
                }
            }
        }

        while l < r {
            if l % 2 == 1 {
                self.lazy[l] = elem.binary_operation(&self.lazy[l]);
                l += 1
            }
            if r % 2 == 1 {
                r -= 1;
                self.lazy[r] = elem.binary_operation(&self.lazy[r]);
            }

            l >>= 1;
            r >>= 1;
        }
    }

    /// Returns `i`-th element.
    ///
    /// # Panics
    ///
    /// Panics if given index is out of bounds.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_query(&self, i: usize) -> T {
        debug_assert!(i < self.len);

        let mut res = T::identity();
        // operation may be non-commutative
        let mut i = self.inner_index(i);
        while i >= 1 {
            res = self.lazy[i].binary_operation(&res);
            i /= 2;
        }

        res
    }

    /// Update `i`-th element using the binary operation defined in the [Monoid] trait.
    /// More precisely, performs `a[i] <- elem ∘ a[i]`.
    ///
    /// # Panics
    ///
    /// Panics if given index is out of bounds.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_update(&mut self, i: usize, elem: T) -> T {
        debug_assert!(i < self.len);

        // propagate pending operations
        let i = self.inner_index(i);
        for d in (1..=self.buf_len.trailing_zeros()).rev() {
            self.propagate(i >> d);
        }

        // override and return previous one
        std::mem::replace(&mut self.lazy[i], elem)
    }
}

impl<T: Monoid> DualSegmentTree<T> {
    /// Creates a new [DualSegmentTree] instance for the given number of elements.
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    pub fn new(n: usize) -> Self {
        let buf_len: usize = n.next_power_of_two();
        let lazy =
            Vec::from_iter(std::iter::repeat_with(|| T::identity()).take(buf_len + n + n % 2))
                .into_boxed_slice();

        Self {
            len: n,
            lazy,
            buf_len,
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the results of updates.
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    pub fn into_vec(mut self) -> Vec<T> {
        for i in 1..self.lazy.len() >> 1 {
            self.propagate(i);
        }

        self.lazy.into_vec().split_off(self.buf_len)
    }
}

impl<T: Monoid> IntoIterator for DualSegmentTree<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}
