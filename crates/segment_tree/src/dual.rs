use std::ops::RangeBounds;

use crate::Monoid;

/// A data structure that supports range updates and point queries.
///
/// # Multiple Functions
///
/// If multiple different functions can be composed, you can use [`DualSegmentTree`].
#[derive(Debug, Clone)]
pub struct DualSegmentTree<T: Monoid> {
    /// one-based indexing buffer for pending operations.
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
        self.lazy[2 * i] = lazy.binary_operation(&self.lazy[2 * i]);
        self.lazy[2 * i + 1] = lazy.binary_operation(&self.lazy[2 * i + 1]);
    }

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

    /// Updates elements in the given `range` using the binary operation defined in the [Monoid] trait.
    /// More precisely, performs `a[i] = elem ∘ a[i]` for each `i` in the range.
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

            l /= 2;
            r /= 2;
        }
    }

    /// Returns `i`-th element.
    pub fn point_query(&self, i: usize) -> T {
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
    /// More precisely, performs `a[i] = elem ∘ a[i]`.
    pub fn point_update(&mut self, i: usize, elem: T) -> T {
        // propagate pending operations
        let i = self.inner_index(i);
        for d in (1..=self.buf_len.trailing_zeros()).rev() {
            self.propagate(i >> d);
        }

        // override and return previous one
        std::mem::replace(&mut self.lazy[i], elem)
    }
}