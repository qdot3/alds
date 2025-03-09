use std::ops::RangeBounds;

use crate::Monoid;

/// Fixed-sized segment tree.
#[derive(Clone, Debug)]
pub struct SegmentTree<T: Monoid> {
    data: Box<[T]>,
}

impl<T: Monoid> SegmentTree<T> {
    #[inline]
    const fn inner_index(&self, i: usize) -> usize {
        self.data.len() / 2 + i
    }

    ///`[l, r)`
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

    pub fn get(&self, i: usize) -> Option<&T> {
        let i = self.inner_index(i);
        self.data.get(i)
    }

    pub fn eval<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.inner_range(range);

        if l + 1 == r {
            return T::identity();
        }
        if l == self.data.len() / 2 && r == self.data.len() {
            return T::identity().binary_operation(&self.data[1]);
        }

        // calculate result on [l, r)
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

    pub fn set(&mut self, i: usize, value: T) {
        let mut i = self.inner_index(i);
        self.data[i] = value;
        while i > 1 {
            i = i / 2;
            self.data[i] = self.data[i * 2].binary_operation(&self.data[i * 2 + 1])
        }
    }

    pub fn fill<R>(&mut self, range: R, value: T)
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
    /// Creates a new segment tree with the given initial `values` in *O*(*N*) time,
    /// where *N* is the number of elements in `values`.
    fn from(values: Vec<T>) -> Self {
        let mut data = Vec::from_iter(
            std::iter::repeat_with(|| T::identity())
                .take(values.len())
                .chain(values),
        )
        .into_boxed_slice();
        for i in (1..data.len() / 2).rev() {
            data[i] = data[2 * i].binary_operation(&data[2 * i + 1])
        }

        Self { data }
    }
}
