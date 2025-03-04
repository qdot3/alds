use std::{cmp::Ordering, ops::Range};

use crate::Monoid;

/// Fixed-sized segment tree.
#[derive(Clone, Debug)]
pub struct SegmentTree<T: Monoid> {
    data: Box<[T]>,
    buf_len: usize,
}

impl<T: Monoid> SegmentTree<T> {
    /// Creates a new fixed-size segment tree initialized with the identity element.
    ///
    /// Use [`from`](Self::from) if you have initial values.
    pub fn new(n: usize) -> Self {
        let buf_len = n.saturating_sub(1);
        let data = Vec::from_iter(std::iter::repeat_with(|| T::identity()).take(n + buf_len))
            .into_boxed_slice();

        Self { data, buf_len }
    }

    const fn inner_index(&self, i: usize) -> usize {
        self.buf_len + i
    }

    fn inner_range(&self, range: Range<usize>) -> Range<usize> {
        let Range { mut start, mut end } = range;
        start = self.inner_index(start).min(self.data.len());
        end = self.inner_index(end).min(self.data.len());

        start..end
    }

    pub fn update(&mut self, i: usize, value: T) {
        let mut i = self.inner_index(i);
        self.data[i] = value;
        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = self.data[i * 2 + 1].binary_operation(&self.data[i * 2 + 2])
        }
    }

    pub fn fill(&mut self, range: Range<usize>, value: T)
    where
        T: Clone,
    {
        let Range {
            start: mut l,
            end: mut r,
        } = range;
        self.data[l..r].fill(value);

        (l, r) = ((l - 1) / 2, (r - 1) / 2);
        while l < r {
            for i in l..r {
                self.data[i] = self.data[i * 2 + 1].binary_operation(&self.data[i * 2 + 2])
            }
            (l, r) = ((l - 1) / 2, (r - 1) / 2);
        }

        assert_eq!(l, r);
        while l > 0 {
            l = (l - 1) / 2;
            self.data[l] = self.data[l * 2 + 1].binary_operation(&self.data[l * 2 + 2])
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        let i = self.inner_index(i);
        self.data.get(i)
    }

    pub fn query(&self, range: Range<usize>) -> T {
        let Range {
            start: mut l,
            end: mut r,
        } = self.inner_range(range);

        // calculate result on [l, r)
        let mut res = T::identity();
        while l < r {
            if l % 2 == 0 {
                res = res.binary_operation(&self.data[l]);
                l += 1;
            }
            if r % 2 == 0 {
                r -= 1;
                res = res.binary_operation(&self.data[r]);
            }

            l /= 2; // = (l - 1) / 2 because `l` is odd
            r /= 2; // = (r - 1) / 2
        }

        res
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
                buf_len: 0,
            };
        }

        let buf_len = values.len() - 1;
        let mut data = Vec::from_iter(
            std::iter::repeat_with(|| T::identity())
                .take(buf_len)
                .chain(values),
        )
        .into_boxed_slice();
        for i in (0..buf_len).rev() {
            data[i] = data[2 * i + 1].binary_operation(&data[2 * i + 2])
        }

        Self { data, buf_len }
    }
}
