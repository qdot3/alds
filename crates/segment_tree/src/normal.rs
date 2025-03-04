use std::{
    cmp::Ordering,
    ops::{Range, RangeBounds},
};

use crate::Monoid;

/// Fixed-sized segment tree.
#[derive(Clone, Debug)]
pub struct SegmentTree<T: Monoid> {
    data: Box<[T]>,
    len: usize,
    buf_len: usize,
}

impl<T: Monoid> SegmentTree<T> {
    /// Creates a new fixed-size segment tree initialized with the identity element.
    ///
    /// Use [`from`](Self::from) if you have initial values.
    pub fn new(n: usize) -> Self {
        let buf_len = n.next_power_of_two(); // non-commutative monoid
        let data =
            Vec::from_iter(std::iter::repeat_with(|| T::identity()).take(n + n % 2 + buf_len))
                .into_boxed_slice();

        Self {
            data,
            len: n,
            buf_len,
        }
    }

    const fn inner_index(&self, i: usize) -> usize {
        self.buf_len + i
    }

    ///`[l, r)`
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

    pub fn update(&mut self, i: usize, value: T) {
        let mut i = self.inner_index(i);
        self.data[i] = value;
        while i > 1 {
            i = i / 2;
            self.data[i] = self.data[i * 2].binary_operation(&self.data[i * 2 + 1])
        }
    }

    pub fn fill(&mut self, range: Range<usize>, value: T)
    where
        T: Clone,
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

    pub fn get(&self, i: usize) -> Option<&T> {
        let i = self.inner_index(i);
        self.data.get(i)
    }

    pub fn query(&self, range: Range<usize>) -> T {
        let (mut l, mut r) = self.inner_range(range);

        if l == self.buf_len && r == self.data.len() {
            return T::identity().binary_operation(&self.data[1]);
        }

        // calculate result on [l, r)
        let (mut res_l, mut res_r) = (T::identity(), T::identity());
        while l < r {
            if l % 2 == 1 {
                res_l = res_l.binary_operation(&self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res_r = self.data[r].binary_operation(&res_r);
            }

            l /= 2;
            r /= 2;
        }

        res_l.binary_operation(&res_r)
    }

    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.data[self.buf_len..].binary_search(x)
    }

    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        self.data[self.buf_len..].binary_search_by(f)
    }

    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
    where
        B: Ord,
        F: FnMut(&'a T) -> B,
    {
        self.data[self.buf_len..].binary_search_by_key(b, f)
    }

    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        self.data[self.buf_len..].partition_point(pred)
    }
}

impl<T: Monoid> From<Vec<T>> for SegmentTree<T> {
    /// Creates a new segment tree with the given initial `values` in *O*(*N*) time,
    /// where *N* is the number of elements in `values`.
    fn from(values: Vec<T>) -> Self {
        if values.is_empty() {
            return Self {
                data: Box::new([]),
                len: 0,
                buf_len: 0,
            };
        }

        let len = values.len();
        let buf_len = values.len().next_power_of_two(); // non-commutative monoid
        let mut data = Vec::from_iter(
            std::iter::repeat_with(|| T::identity())
                .take(buf_len)
                .chain(values)
                .chain(std::iter::repeat_with(|| T::identity()).take(len % 2)),
        )
        .into_boxed_slice();
        for i in (1..buf_len).rev() {
            if i * 2 + 1 < len + buf_len {
                data[i] = data[2 * i].binary_operation(&data[2 * i + 1])
            }
        }

        Self { data, len, buf_len }
    }
}
