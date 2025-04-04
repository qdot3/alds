use std::ops::RangeBounds;

use math_traits::{marker::Commutative, Group};

/// A data structure which efficiently performs point updates and range queries.
pub struct FenwickTree<T: Group + Commutative> {
    /// one-based indexing internally (`data[0]` is the identity element for simple implementation)
    data: Vec<T>,
}

impl<T: Group + Commutative> FenwickTree<T> {
    /// Creates a new instance initialized with [Group::identity].
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            data: Vec::from_iter(std::iter::repeat_with(T::identity).take(n + 1)),
        }
    }

    /// Updates `i`-th element using [Group::bin_op].
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
            // add LSSB
            i += i & i.wrapping_neg()
        }
    }

    /// Returns the result of combining elements over the [0, i).
    ///
    /// # Panics
    ///
    /// Panics if the given index is out of bounds.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    #[must_use]
    pub fn prefix_query(&self, mut i: usize) -> T {
        // avoid boundary check in while loop
        assert!(i < self.data.len(), "index out of bounds");

        let mut res = T::identity();
        while i > 0 {
            res = res.bin_op(&self.data[i]);
            // remove LSSB
            i &= i.wrapping_sub(1)
        }

        res
    }

    /// Returns the result of combining elements over the given range.
    ///
    /// If the given range is empty, then returns [Group::identity].
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    #[must_use]
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

        if l >= r {
            return T::identity();
        }
        // avoid boundary check in while loop
        assert!(r < self.data.len(), "index out of bounds");

        let mut res = T::identity();
        // skip common prefix (net zero)
        let mask = !0 >> (l ^ r).leading_zeros();
        while l & mask != 0 {
            res = res.bin_op(&self.data[l]);
            l &= l.wrapping_sub(1)
        }
        res = res.inverse();
        while r & mask != 0 {
            res = res.bin_op(&self.data[r]);
            r &= r.wrapping_sub(1)
        }

        res

        // let (mut res_l, mut res_r) = (T::identity(), T::identity());
        // // if l == r, then the result of remaining operations is net zero.
        // while l != r {
        //     if l > r {
        //         res_l = res_l.bin_op(&self.data[l]);
        //         // remove LSSB
        //         l &= l.wrapping_sub(1);
        //     } else {
        //         res_r = res_r.bin_op(&self.data[r]);
        //         r &= r.wrapping_sub(1);
        //     }
        // }

        // res_l.inverse().bin_op(&res_r)
    }

    /// Returns minimum `i` which satisfies `pred(prefix_query(i)) = true` if sorted.
    ///
    /// # Time complexity
    ///
    /// *Θ*(log *N*)
    #[must_use]
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
    #[must_use]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut data = vec![T::identity()];
        data.extend(iter);
        for i in 1..data.len() - 1 {
            // add LSSB
            data[i + 1] = data[i + 1].bin_op(&data[i])
        }

        Self { data }
    }
}
