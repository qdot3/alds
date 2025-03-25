use std::ops::RangeBounds;

use math_traits::{marker::Commutative, Group};

pub struct FenwickTree<T: Group + Commutative> {
    /// one-based indexing internally (`data[0]` is the identity element 0 for simple implementation)
    data: Vec<T>,
}

impl<T: Group + Commutative> FenwickTree<T> {
    /// Creates new fixed-size Fenwick tree.
    ///
    /// # Panics
    ///
    /// Panics if `size` is [`usize::MAX`].
    pub fn new(n: usize) -> Self {
        Self {
            data: Vec::from_iter(std::iter::repeat_with(T::identity).take(n + 1)),
        }
    }

    /// Updates `i`-th element using the binary operation defined in the [Group] trait.
    /// More precisely, performs `a[i] <- elem ∘ a[i]`.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_update(&mut self, mut i: usize, elem: T) {
        // one-based indexing
        i += 1;

        while let Some(data) = self.data.get_mut(i) {
            *data = elem.bin_op(data);
            i += 1 << i.trailing_zeros();
        }
    }

    /// Calculates the product of elements over [0, i).
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn prefix_query(&self, mut i: usize) -> T {
        i = i.min(self.data.len() - 1);

        let mut res = T::identity();
        while i > 0 {
            res = res.bin_op(&self.data[i]);
            i -= 1 << i.trailing_zeros();
        }

        res
    }

    /// Calculate the sum of the range.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_query<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        // (l, r] due to one-based indexing
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => self.data.len() - 1,
        };

        let mut res = T::identity();
        // if l = r, then the result of remaining operations is net zero.
        while l != r {
            if l > r {
                res = res.bin_op(&self.data[l].inverse());
                l &= l.wrapping_sub(1);
            } else {
                res = res.bin_op(&self.data[r]);
                r &= r.wrapping_sub(1);
            }
        }

        res
    }

    /// See [`std::slice::partition_point`](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point).
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn partition_point(&self, pred: impl Fn(T) -> bool) -> usize {
        let mut res = 0;
        let mut sum = T::identity();

        // start from the largest block.
        for d in (0..=self.data.len().ilog2()).rev() {
            if let Some(block) = self.data.get(res + (1 << d)) {
                if pred(sum.bin_op(&block)) {
                    res += 1 << d;
                    sum = sum.bin_op(&block)
                }
            }
        }

        res
    }
}

impl<T: Group + Commutative> FromIterator<T> for FenwickTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();

        if Some(min) == max {
            let mut ft = Self::new(min);
            for (i, v) in iter.enumerate() {
                ft.point_update(i, v)
            }

            ft
        } else {
            let mut data = Vec::from_iter(std::iter::once(T::identity()).chain(iter));
            for i in 1..data.len() {
                let (prefix, suffix) = data.split_at_mut(i + 1);
                let mut j = i + (1 << i.trailing_zeros());
                while let Some(acc) = suffix.get_mut(j - i - 1) {
                    *acc = prefix[i].bin_op(acc);
                    j += 1 << j.trailing_zeros()
                }
            }

            Self { data }
        }
    }
}
