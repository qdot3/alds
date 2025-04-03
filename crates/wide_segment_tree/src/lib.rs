use std::ops::RangeBounds;

use math_traits::Monoid;

#[repr(align(64))]
pub struct WideSegmentTree<T: Monoid> {
    data: Box<[T]>,
    /// Partitions between layers
    partition: Box<[usize]>,
}

impl<T: Monoid> WideSegmentTree<T> {
    const BITS: u32 = {
        assert!(
            std::mem::size_of::<T>().is_power_of_two(),
            "data does NOT fit cache line"
        );
        // cache line size is assumed to be 64 bytes.
        let n = 64 / std::mem::size_of::<T>();
        n.ilog2()
    };
    /// Number of elements in single cash line.
    const N: usize = 1 << Self::BITS;

    #[inline]
    const fn round_up(i: usize) -> usize {
        Self::round_down(i + Self::N - 1)
    }

    #[inline]
    const fn round_down(i: usize) -> usize {
        i & !(Self::N - 1)
    }

    pub fn point_update(&mut self, mut i: usize, elem: T) {
        let Self { data, partition } = self;

        for p in partition.iter() {
            data[p + i] = elem.bin_op(&data[p + i]);
            i >>= Self::BITS;
        }
    }

    pub fn range_query<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => todo!(),
        };
        if l >= r {
            return T::identity();
        }

        let (mut res_l, mut res_r) = (T::identity(), T::identity());
        let Self { data, partition } = self;
        for p in partition.iter() {
            if l / Self::N == r / Self::N {
                return data[p + l..p + r]
                    .iter()
                    .fold(res_l, |acc, v| acc.bin_op(v))
                    .bin_op(&res_r);
            } else {
                if l % Self::N != 0 {
                    res_l = data[p + l..Self::round_up(p + l)]
                        .iter()
                        .fold(res_l, |acc, v| acc.bin_op(v));
                    l += Self::N;
                }
                if r & Self::N != 0 {
                    res_r = data[Self::round_down(p + l)..p + l]
                        .iter()
                        .fold(res_r, |acc, v| v.bin_op(&acc));
                }

                l >>= Self::BITS;
                r >>= Self::BITS;
            }
        }
        unreachable!()
    }
}

impl<T: Monoid> FromIterator<T> for WideSegmentTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}
