use std::ops::{Index, RangeBounds};


use crate::Monoid;

/// A data structure that supports point updates and range queries.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```
/// use segment_tree::{Monoid, SegmentTree};
///
/// // range minimum query
/// struct RMQ(i32);
///
/// impl Monoid for RMQ {
///     fn identity() -> Self {
///         RMQ(i32::MAX)
///     }
///
///     fn binary_operation(&self, rhs: &Self) -> Self {
///         RMQ(self.0.min(rhs.0))
///     }
/// }
///
/// let mut seg_tree = SegmentTree::from(Vec::from_iter((0..6).map(|i| RMQ(i))));
/// // [(0, 1, 2, 3, 4, 5)]
/// assert_eq!(seg_tree.range_query(..).0, 0);
/// assert_eq!(seg_tree.range_query(2..6).0, 2);
///
/// seg_tree.point_update(4, RMQ(-10));
/// // [0, 1, 2, 3, -10, 5]
/// assert_eq!(seg_tree.range_query(..).0, -10);
/// ```
///
/// ## Multiple range queries
///
/// When handling multiple types of range queries on the same data sequence,
/// using multiple segment trees is an option.
/// However, incorporating set of elements into a single segment tree
/// generally yields better performance.
///
/// ```
/// use segment_tree::{Monoid, SegmentTree};
///
/// struct MinMax(i32, i32);
///
/// impl Monoid for MinMax {
///     fn identity() -> Self {
///         Self(i32::MAX, i32::MIN)
///     }
///
///     fn binary_operation(&self, rhs: &Self) -> Self {
///         Self(self.0.min(rhs.0), self.1.max(rhs.1))
///     }
/// }
///
/// let mut seg_tree = SegmentTree::from(Vec::from_iter((0..6).map(|i| MinMax(5 - i, i))));
/// // [(5, 0), (4, 1), (3, 2), (2, 3), (1, 4), (0, 5)]
/// assert_eq!(seg_tree.range_query(..).0, 0);
/// assert_eq!(seg_tree.range_query(2..3).1, 2);
///
/// seg_tree.point_update(4, MinMax(100, 100));
/// // [(5, 0), (4, 1), (3, 2), (2, 3), (-100, -100), (0, 5)]
/// assert_eq!(seg_tree.range_query(3..).0, 0);
/// assert_eq!(seg_tree.range_query(3..).1, 100);
/// ```
#[derive(Clone, Debug)]
pub struct SegmentTree<T: Monoid> {
    data: Box<[T]>,
}

impl<T: Monoid> SegmentTree<T> {
    #[inline]
    const fn inner_index(&self, i: usize) -> usize {
        self.data.len() / 2 + i
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
            std::ops::Bound::Unbounded => self.data.len() / 2,
        };

        (self.inner_index(l), self.inner_index(r))
    }

    /// Returns a reference to a single element.
    ///
    /// # Panics
    ///
    /// Panics if given `i` is out of bounds.
    #[inline]
    pub fn point_query(&self, i: usize) -> &T {
        let i = self.inner_index(i);
        &self.data[i]
    }

    /// Returns the result of combining elements over the 'given' range.
    ///
    /// # Panics
    ///
    /// Panics if given `range` is out of bounds.
    pub fn range_query<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.inner_range(range);

        if l >= r {
            return T::identity();
        }

        // calculate result over [l, r)
        l >>= l.trailing_zeros();
        r >>= r.trailing_zeros();
        let (mut res_l, mut res_r) = (T::identity(), T::identity());
        loop {
            if l >= r {
                res_l = res_l.binary_operation(&self.data[l]);
                l += 1;
                l >>= l.trailing_zeros()
            } else {
                r -= 1;
                res_r = self.data[r].binary_operation(&res_r);
                r >>= r.trailing_zeros()
            }

            if l == r {
                break;
            }
        }

        res_l.binary_operation(&res_r)
    }

    /// Replace the `i`-th element with the given one.
    ///
    /// # Panics
    ///
    /// Panics if given index is out of bounds.
    pub fn point_update(&mut self, i: usize, element: T) -> T {
        let mut i = self.inner_index(i);
        let old = std::mem::replace(&mut self.data[i], element);
        // TODO: remove updates on invalid nodes
        while i > 1 {
            i >>= 1;
            self.data[i] = self.data[i * 2].binary_operation(&self.data[i * 2 + 1])
        }

        old
    }

    // TODO: impl max_right() & max_left()
}

impl<T: Monoid> SegmentTree<T> {
    pub fn new(n: usize) -> Self {
        let data = Vec::from_iter(std::iter::repeat_with( T::identity).take(n << 1))
            .into_boxed_slice();

        Self { data }
    }

    pub fn into_vec(self) -> Vec<T> {
        let n = self.data.len() >> 1;

        self.data.into_vec().split_off(n)
    }

    #[allow(dead_code)]
    fn fill<R>(&mut self, range: R, value: T)
    where
        T: Clone,
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.inner_range(range);
        self.data[l..r].fill(value);

        (l, r) = (l / 2, r / 2);
        while l < r {
            for i in l..r {
                self.data[i] = self.data[i * 2].binary_operation(&self.data[i * 2 + 1])
            }
            (l, r) = (l / 2, r / 2);
        }

        assert_eq!(l, r);
        while l > 1 {
            l /= 2;
            self.data[l] = self.data[l * 2].binary_operation(&self.data[l * 2 + 1])
        }
    }
}

impl<T: Monoid> From<Vec<T>> for SegmentTree<T> {
    /// Creates a new segment tree with the given initial `elements` in *O*(*N*) time,
    /// where *N* is the number of elements in `elements`.
    fn from(elements: Vec<T>) -> Self {
        // this space optimization is valid even in commutative operation cases.
        let mut data = Vec::from_iter(
            std::iter::repeat_with( T::identity)
                .take(elements.len())
                .chain(elements),
        )
        .into_boxed_slice();
        for i in (1..data.len() / 2).rev() {
            data[i] = data[2 * i].binary_operation(&data[2 * i + 1])
        }

        Self { data }
    }
}

impl<T: Monoid> FromIterator<T> for SegmentTree<T> {
    /// Creates a new segment tree with the given initial elements in *O*(*N*) time,
    /// where *N* is the number of elements.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        if Some(min) == max {
            // same as `from()`
            let mut data = Vec::from_iter(
                std::iter::repeat_with(T::identity)
                    .take(min)
                    .chain(iter),
            )
            .into_boxed_slice();
            for i in (1..min).rev() {
                data[i] = data[2 * i].binary_operation(&data[2 * i + 1])
            }

            Self { data }
        } else {
            Self::from(Vec::from_iter(iter))
        }
    }
}

impl<T: Monoid> IntoIterator for SegmentTree<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<T: Monoid> Index<usize> for SegmentTree<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let i = self.inner_index(index);
        &self.data[i]
    }
}
