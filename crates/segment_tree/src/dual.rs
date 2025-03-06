use std::ops::RangeBounds;

use crate::MapMonoid;

/// A data structure that efficiently applies (non-commutative) functions (or maps) to consecutive elements
/// and retrieves a single element.
///
/// # Multiple Functions
///
/// If multiple different functions can be composed, you can use [`DualSegmentTree`].
#[derive(Debug, Clone)]
pub struct DualSegmentTree<T, F: MapMonoid<T>> {
    data: Box<[T]>,
    /// one-based indexing buffer for pending maps.
    maps: Box<[F]>,
    buf_len: usize,
}

impl<T, F: MapMonoid<T>> DualSegmentTree<T, F> {
    /// Converts index of `data` to the corresponding index of `maps`.
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
            std::ops::Bound::Unbounded => self.data.len(),
        };

        (self.inner_index(l), self.inner_index(r))
    }

    /// Propagates pending operations to the children.
    ///
    /// # Panics
    ///
    /// Assume two children exist.
    fn push(&mut self, i: usize) {
        let map = std::mem::replace(&mut self.maps[i], F::identity());
        self.maps[2 * i] = map.composite(&self.maps[2 * i]);
        self.maps[2 * i + 1] = map.composite(&self.maps[2 * i + 1]);
    }

    /// Applies `map` to elements in the given `range`.
    pub fn apply<R>(&mut self, range: R, map: F)
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.inner_range(range);
        if l >= r {
            return;
        }

        if F::IS_COMMUTATIVE {
            // propagate pending operations
            for d in ((l | r).trailing_zeros().max(1)..=self.buf_len.trailing_zeros()).rev() {
                if (l >> d) << d != l {
                    self.push(l >> d);
                }
                if (r >> d) << d != r {
                    self.push((r - 1) >> d);
                }
            }
        }

        while l < r {
            if l % 2 == 1 {
                self.maps[l] = map.composite(&self.maps[l]);
                l += 1
            }
            if r % 2 == 1 {
                r -= 1;
                self.maps[r] = map.composite(&self.maps[r])
            }

            l /= 2;
            r /= 2;
        }
    }

    /// Returns `i`-th element.
    pub fn get(&self, i: usize) -> T {
        // maps may be non-commutative
        let (mut i, mut res) = {
            let arg = &self.data[i];
            let i = self.inner_index(i);

            (i, self.maps[i].apply(arg))
        };
        while i > 1 {
            i /= 2;
            res = self.maps[i].apply(&res)
        }

        res
    }

    /// Overrides `i`-th element with the given `value` and returns the previous result.
    pub fn set(&mut self, i: usize, value: T) -> T {
        // propagate pending operations
        let ii = self.inner_index(i);
        for d in (1..=self.buf_len.trailing_zeros()).rev() {
            self.push(ii >> d);
        }

        // override and return
        std::mem::replace(&mut self.maps[ii], F::identity())
            .apply(&std::mem::replace(&mut self.data[i], value))
    }
}

impl<T, F: MapMonoid<T>> From<Vec<T>> for DualSegmentTree<T, F> {
    fn from(data: Vec<T>) -> Self {
        let data = data.into_boxed_slice();
        let buf_len: usize = data.len().next_power_of_two();
        let maps = Vec::from_iter(
            std::iter::repeat_with(|| F::identity()).take(buf_len + (data.len() + 1) / 2 * 2),
        )
        .into_boxed_slice();

        Self {
            data,
            maps,
            buf_len,
        }
    }
}
