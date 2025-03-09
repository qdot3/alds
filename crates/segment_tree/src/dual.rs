use std::ops::RangeBounds;

use crate::MonoidAct;

/// A data structure that efficiently applies (non-commutative) functions (or acts) to consecutive elements
/// and retrieves a single element.
///
/// # Multiple Functions
///
/// If multiple different functions can be composed, you can use [`DualSegmentTree`].
#[derive(Debug, Clone)]
pub struct DualSegmentTree<F: MonoidAct> {
    // data: Box<[T]>,
    len: usize,
    /// one-based indexing buffer for pending actions.
    action: Box<[F]>,
    buf_len: usize,
}

impl<F: MonoidAct> DualSegmentTree<F> {
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
        let action = std::mem::replace(&mut self.action[i], F::identity());
        self.action[2 * i] = action.composite(&self.action[2 * i]);
        self.action[2 * i + 1] = action.composite(&self.action[2 * i + 1]);
    }

    pub fn new(n: usize) -> Self {
        let buf_len: usize = n.next_power_of_two();
        let action = Vec::from_iter(
            std::iter::repeat_with(|| F::identity()).take(buf_len + (n + 1) / 2 * 2),
        )
        .into_boxed_slice();

        Self {
            len: n,
            action,
            buf_len,
        }
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
                    self.propagate(l >> d);
                }
                if (r >> d) << d != r {
                    self.propagate((r - 1) >> d);
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
    pub fn get(&self, i: usize) -> F {
        let mut res = F::identity();
        // action may be non-commutative
        let mut i = self.inner_index(i);
        while i >= 1 {
            res = self.action[i].composite(&res);
            i /= 2;
        }

        res
    }

    /// Set `i`-th act with the given `act` and returns the previous one.
    pub fn set(&mut self, i: usize, act: F) -> F {
        // propagate pending operations
        let i = self.inner_index(i);
        for d in (1..=self.buf_len.trailing_zeros()).rev() {
            self.propagate(i >> d);
        }

        // override and return previous one
        std::mem::replace(&mut self.action[i], act)
    }
}

// impl<T, F: MonoidAction<T>> From<Vec<T>> for DualSegmentTree<T, F> {
//     fn from(data: Vec<T>) -> Self {
//         let data = data.into_boxed_slice();
//         let buf_len: usize = data.len().next_power_of_two();
//         let action = Vec::from_iter(
//             std::iter::repeat_with(|| F::identity()).take(buf_len + (data.len() + 1) / 2 * 2),
//         )
//         .into_boxed_slice();

//         Self {
//             data,
//             action,
//             buf_len,
//         }
//     }
// }
