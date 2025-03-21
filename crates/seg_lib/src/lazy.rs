use std::ops::RangeBounds;

use super::{Monoid, MonoidAct};

/// A segment tree that supports range updates and range queries.
///
/// # Examples
///
/// - [range affine range sum](): size-dependent updates
///
/// If the cost of n-folding composition of acts is high, /TODO/ is more suitable.
#[derive(Clone)]
pub struct LazySegmentTree<F: MonoidAct + Clone> {
    /// Stores given elements with buffer. The size will be even for simplicity.
    data: Box<[<F as MonoidAct>::Arg]>,
    /// True size of data (without any buffer).
    len: usize,
    /// Stores pending acts. The size will be `len.next_power_of_two()`
    lazy: Box<[F]>,
    /// A shortcut to `lazy.len().trailing_zeros()`.
    lazy_height: u32,
}

impl<F: MonoidAct + Clone> LazySegmentTree<F> {
    #[inline]
    const fn inner_index(&self, i: usize) -> usize {
        self.lazy.len() + i
    }

    /// Returns `[l, r)`
    #[inline]
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

    #[inline]
    fn update(&mut self, i: usize) {
        self.data[i] = self.data[i << 1].binary_operation(&self.data[(i << 1) | 1])
    }

    #[inline]
    fn push(&mut self, i: usize, act: F) {
        self.data[i] = act.apply(&self.data[i]);
        if i < self.lazy.len() {
            // apply `act` after `lazy[i]`
            self.lazy[i] = act.composite(&self.lazy[i])
        }
    }

    #[inline]
    fn propagate(&mut self, i: usize) {
        let act = std::mem::replace(&mut self.lazy[i], F::identity());
        self.push(i << 1, act.clone());
        self.push((i << 1) | 1, act);
    }

    /// Returns a reference to a single element.
    ///
    /// # Panics
    ///
    /// Panics if given index is out of bounds.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_query(&mut self, i: usize) -> &<F as MonoidAct>::Arg {
        let i = self.inner_index(i);

        // apply pending acts
        for d in (1..=self.lazy_height).rev() {
            self.propagate(i >> d);
        }

        &self.data[i]
    }

    /// Returns the result of combining elements over the given `range`.
    /// If given `range` is empty, returns the identity element defined as [`Monoid::identity`].
    ///
    /// # Panics
    ///
    /// Panics if given `range` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_query<R>(&mut self, range: R) -> <F as MonoidAct>::Arg
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.inner_range(range);

        if l >= r {
            return <F as MonoidAct>::Arg::identity();
        }
        if l + 1 == r {
            return self.point_query(l - self.lazy.len()).clone();
        }

        // apply pending acts
        let common = (l ^ r).ilog2();
        for d in (common + 1..=self.lazy_height).rev() {
            if (l >> d) << d != l || (r >> d) << d != r {
                self.propagate(l >> d);
            }
        }
        for d in (1..=common).rev() {
            // avoid unnecessary propagation
            if (l >> d) << d != l {
                self.propagate(l >> d);
            }
            if (r >> d) << d != r {
                self.propagate(r >> d);
            }
        }

        // calculate result over [l, r)
        l >>= l.trailing_zeros();
        r >>= r.trailing_zeros();

        if l == r {
            return self.data[l].clone();
        }

        let (mut res_l, mut res_r) = (
            <F as MonoidAct>::Arg::identity(),
            <F as MonoidAct>::Arg::identity(),
        );
        while l != r {
            if l >= r {
                res_l = res_l.binary_operation(&self.data[l]);
                l += 1;
                l >>= l.trailing_zeros()
            } else {
                r -= 1;
                res_r = self.data[r].binary_operation(&res_r);
                r >>= r.trailing_zeros()
            }
        }

        res_l.binary_operation(&res_r)
    }

    /// Update `i`-th element using the operation defined as [MonoidAct::apply].
    /// More precisely, performs `a[i] <- act.apply(a[i])`.
    ///
    /// # Panics
    ///
    /// Panics if given index is out of bounds.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_update(&mut self, i: usize, act: F) {
        // apply pending acts
        let value = act.apply(self.point_query(i));

        // update data
        let i = self.inner_index(i);
        self.data[i] = value;
        for d in 1..=self.lazy_height {
            self.update(i >> d);
        }
    }

    /// Updates elements in the given `range` using the operation defined as [MonoidAct::apply].
    /// More precisely, performs `a[i] <- act.apply(a[i])` for each `i` in the range.
    ///
    /// # Panics
    ///
    /// Panics if given `range` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_update<R>(&mut self, range: R, act: F)
    where
        R: RangeBounds<usize>,
    {
        let (l, r) = self.inner_range(range);
        if l >= r {
            return;
        }
        if l + 1 == r {
            self.point_update(l - self.lazy.len(), act);
            return;
        }

        // apply pending acts
        let common = (l ^ (r - 1)).ilog2();
        for d in (common + 1..=self.lazy_height).rev() {
            if (l >> d) << d != l || (r >> d) << d != r {
                self.propagate(l >> d);
            }
        }
        for d in (1..=common).rev() {
            // avoid unnecessary propagation
            if (l >> d) << d != l {
                self.propagate(l >> d);
            }
            if (r >> d) << d != r {
                self.propagate((r - 1) >> d);
            }
        }

        // apply `act` in a lazy way
        {
            let (mut l, mut r) = (l, r);
            l >>= l.trailing_zeros();
            r >>= r.trailing_zeros();
            if l == r {
                self.push(l, act);
            } else {
                while l != r {
                    if l >= r {
                        self.push(l, act.clone());
                        l += 1;
                        l >>= l.trailing_zeros();
                    } else {
                        r -= 1;
                        self.push(r, act.clone());
                        r >>= r.trailing_zeros();
                    }
                }
            }
        }

        // update parents of modified nodes
        for d in 1..=self.lazy_height {
            // avoid updating node with children which has not been updated
            if (l >> d) << d != l {
                self.update(l >> d);
            }
            if (r >> d) << d != r {
                self.update((r - 1) >> d);
            }
        }
    }
}

impl<F: MonoidAct + Clone> LazySegmentTree<F> {
    /// Creates a new [LazySegmentTree] instance initialized with identity elements.
    ///
    /// Use [LazySegmentTree::from] for custom initial elements for better performance.
    ///
    /// # Time Complexity
    ///
    /// *O*(*N*)
    pub fn new(n: usize) -> Self {
        // the size will be determined (i.e. `n`)
        // See <https://doc.rust-lang.org/std/iter/struct.Take.html#impl-ExactSizeIterator-for-Take%3CRepeatWith%3CF%3E%3E>
        Self::from_iter(std::iter::repeat_with(<F as MonoidAct>::Arg::identity).take(n))
    }

    /// Returns the number of elements.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the results of updates.
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    pub fn into_vec(mut self) -> Vec<<F as MonoidAct>::Arg> {
        // propagate all pending acts
        for i in 1..self.data.len() >> 1 {
            self.propagate(i);
        }

        // discard buffer
        self.data.into_vec().split_off(self.lazy.len())
    }
}

impl<F: MonoidAct + Clone> IntoIterator for LazySegmentTree<F> {
    type Item = <F as MonoidAct>::Arg;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<F: MonoidAct + Clone> FromIterator<<F as MonoidAct>::Arg> for LazySegmentTree<F> {
    fn from_iter<T: IntoIterator<Item = <F as MonoidAct>::Arg>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();

        // avoid unnecessary `Vec::collect()`
        let (len, buf_len, mut data) = if Some(min) == max {
            let len = min;
            let buf_len = min.next_power_of_two();
            let data = Vec::from_iter(
                std::iter::repeat_with(<F as MonoidAct>::Arg::identity)
                    .take(buf_len)
                    .chain(iter)
                    .chain(std::iter::repeat_with(<F as MonoidAct>::Arg::identity).take(len % 2)), // save space
            )
            .into_boxed_slice();

            (len, buf_len, data)
        } else {
            let vec = Vec::from_iter(iter);
            let len = vec.len();
            let buf_len = min.next_power_of_two();
            let data = Vec::from_iter(
                std::iter::repeat_with(<F as MonoidAct>::Arg::identity)
                    .take(buf_len)
                    .chain(vec)
                    .chain(std::iter::repeat_with(<F as MonoidAct>::Arg::identity).take(len % 2)), // save space
            )
            .into_boxed_slice();

            (len, buf_len, data)
        };

        for i in (1..data.len() / 2).rev() {
            data[i] = data[i * 2].binary_operation(&data[i * 2 + 1])
        }
        let lazy =
            Vec::from_iter(std::iter::repeat_with(F::identity).take(buf_len)).into_boxed_slice();

        Self {
            data,
            len,
            lazy,
            lazy_height: buf_len.trailing_zeros(),
        }
    }
}

impl<F: MonoidAct + Clone> From<Vec<<F as MonoidAct>::Arg>> for LazySegmentTree<F> {
    fn from(values: Vec<<F as MonoidAct>::Arg>) -> Self {
        // the size will be fixed.
        // See <https://doc.rust-lang.org/std/vec/struct.IntoIter.html#impl-ExactSizeIterator-for-IntoIter%3CT,+A%3E>
        Self::from_iter(values)
    }
}
