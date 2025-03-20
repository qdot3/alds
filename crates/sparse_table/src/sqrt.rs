use std::ops::RangeBounds;

use super::{DisjointSparseTable, Semigroup};

#[derive(Debug)]
pub struct SqrtTable<T: Semigroup + Clone> {
    large_table: DisjointSparseTable<T>,
    small_table: Box<[DisjointSparseTable<T>]>,
    block_size: usize,
    len: usize,
}

impl<T: Semigroup + Clone> SqrtTable<T> {
    pub fn range_query<R>(&self, range: R) -> Option<T>
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => self.len,
        };
        if l >= r {
            return None;
        }

        let (il, ir) = (l / self.block_size, r / self.block_size);

        if il == ir {
            return self.small_table[il]
                .range_query(l - il * self.block_size..r - ir * self.block_size);
        }

        let left = self.small_table[il].range_query(l - il * self.block_size..);
        let center = self.large_table.range_query(il + 1..ir);
        let right = self.small_table[ir].range_query(..r - ir * self.block_size);
        [center, right]
            .into_iter()
            .fold(left, |acc, v| match (acc, v) {
                (Some(acc), Some(v)) => Some(acc.binary_operation(&v)),
                (None, Some(v)) => Some(v),
                (Some(acc), None) => Some(acc),
                (None, None) => None,
            })
    }
}

impl<T: Semigroup + Clone> From<Vec<T>> for SqrtTable<T> {
    fn from(mut value: Vec<T>) -> Self {
        let len = value.len();
        let block_size = len.isqrt() + 1;
        let large_table = DisjointSparseTable::from_iter(value.chunks(block_size).map(|b| {
            if b.len() == 1 {
                b[0].clone()
            } else {
                b.iter()
                    .skip(2)
                    .fold(b[0].binary_operation(&b[1]), |acc, v| {
                        acc.binary_operation(v)
                    })
            }
        }));
        let mut small_table = Vec::from_iter(
            (0..len)
                .step_by(block_size)
                .rev()
                .map(|i| DisjointSparseTable::from(value.split_off(i))),
        );
        small_table.reverse();

        Self {
            large_table,
            small_table: small_table.into_boxed_slice(),
            block_size,
            len,
        }
    }
}
