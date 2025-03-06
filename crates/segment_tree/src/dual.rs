use std::ops::RangeBounds;

use crate::MonoidAction;

/// A data structure that efficiently applies (non-commutative) functions (or actions) to consecutive elements
/// and retrieves a single element.
///
/// # Multiple Functions
///
/// If multiple different functions can be composed, you can use [`DualSegmentTree`].
#[derive(Debug, Clone)]
pub struct DualSegmentTree<T, F: MonoidAction<T>> {
    data: Box<[T]>,
    /// one-based indexing buffer for pending actions.
    action: Box<[F]>,
    buf_len: usize,
}

impl<T, F: MonoidAction<T>> DualSegmentTree<T, F> {
    /// Converts index of `data` to the corresponding index of `action`.
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
        let action = std::mem::replace(&mut self.action[i], F::identity());
        self.action[2 * i] = action.composite(&self.action[2 * i]);
        self.action[2 * i + 1] = action.composite(&self.action[2 * i + 1]);
    }

    /// Applies `action` to elements in the given `range`.
    pub fn apply<R>(&mut self, range: R, action: F)
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.inner_range(range);
        if l >= r {
            return;
        }

        if !F::IS_COMMUTATIVE {
            // propagate pending operations
            for d in (1..=self.buf_len.trailing_zeros()).rev() {
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
                self.action[l] = action.composite(&self.action[l]);
                l += 1
            }
            if r % 2 == 1 {
                r -= 1;
                self.action[r] = action.composite(&self.action[r])
            }

            l /= 2;
            r /= 2;
        }
    }

    /// Returns `i`-th element.
    pub fn get(&self, i: usize) -> T {
        // action may be non-commutative
        let (mut i, mut res) = {
            let arg = &self.data[i];
            let i = self.inner_index(i);

            (i, self.action[i].apply(arg))
        };
        while i > 1 {
            i /= 2;
            res = self.action[i].apply(&res)
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
        std::mem::replace(&mut self.action[ii], F::identity())
            .apply(&std::mem::replace(&mut self.data[i], value))
    }
}

impl<T, F: MonoidAction<T>> From<Vec<T>> for DualSegmentTree<T, F> {
    fn from(data: Vec<T>) -> Self {
        let data = data.into_boxed_slice();
        let buf_len: usize = data.len().next_power_of_two();
        let action = Vec::from_iter(
            std::iter::repeat_with(|| F::identity()).take(buf_len + (data.len() + 1) / 2 * 2),
        )
        .into_boxed_slice();

        Self {
            data,
            action,
            buf_len,
        }
    }
}
