use std::{fmt::Debug, ops::RangeBounds};

/// Defines a set of elements which forms a monoid
pub trait IdempotentSemigroup {
    /// Performs the associative binary operation on the monoid.
    fn binary_operation(&self, rhs: &Self) -> Self;
}

#[derive(Clone)]
pub struct SparseTable<T: IdempotentSemigroup> {
    table: Box<[T]>,
    partition: Box<[usize]>,
}

impl<T: IdempotentSemigroup> SparseTable<T> {
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
            std::ops::Bound::Unbounded => self.partition[1],
        };

        if l >= r {
            return None;
        }

        let w = (r - l).ilog2() as usize;
        Some(
            self.table[self.partition[w] + l]
                .binary_operation(&self.table[self.partition[w] + r - (1 << w)]),
        )
    }
}

impl<T: IdempotentSemigroup + Debug> Debug for SparseTable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SparseTable")
            .field(
                "table",
                &Vec::from_iter(
                    self.partition
                        .windows(2)
                        .map(|lr| &self.table[lr[0]..lr[1]]),
                ),
            )
            .finish()
    }
}

impl<T: IdempotentSemigroup> FromIterator<T> for SparseTable<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        let (height, mut table) = if Some(min) == max {
            let height = min.next_power_of_two().trailing_zeros() as usize;
            let mut table = Vec::with_capacity(min * (height + 1));
            table.extend(iter);

            (height, table)
        } else {
            let mut table = Vec::from_iter(iter);
            let height = table.len().next_power_of_two().trailing_zeros() as usize;
            table.reserve(table.len() * height);

            (height, table)
        };

        let mut partition = Vec::with_capacity(height + 1);
        partition.extend_from_slice(&[0, table.len()]);

        for i in 1..height {
            for j in (partition[i - 1]..partition[i]).skip(1 << i - 1) {
                table.push(table[j - (1 << i - 1)].binary_operation(&table[j]));
            }
            partition.push(table.len());
        }

        Self {
            table: table.into_boxed_slice(),
            partition: partition.into_boxed_slice(),
        }
    }
}

impl<T: IdempotentSemigroup> From<Vec<T>> for SparseTable<T> {
    fn from(value: Vec<T>) -> Self {
        Self::from_iter(value)
    }
}
