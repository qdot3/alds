use std::ops::RangeBounds;

use super::Semigroup;

#[derive(Debug, Clone)]
pub struct DisjointSparseTable<T: Semigroup + Clone> {
    table: Box<[T]>,
    len: usize,
}

impl<T: Semigroup + Clone> DisjointSparseTable<T> {
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

        // result over [l, r)
        if l + 1 == r {
            Some(self.table[l].clone())
        } else {
            let level = (l ^ (r - 1)).ilog2() as usize;
            Some(
                self.table[level * self.len + l]
                    .binary_operation(&self.table[level * self.len + (r - 1)]),
            )
        }
    }
}

impl<T: Semigroup + Clone> DisjointSparseTable<T> {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn into_vec(self) -> Vec<T> {
        let mut res = self.table.into_vec();
        res.truncate(self.len);

        res
    }
}

impl<T: Semigroup + Clone> FromIterator<T> for DisjointSparseTable<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        let (height, mut table) = if Some(min) == max {
            let height = min.next_power_of_two().trailing_zeros() as usize;
            let mut table = Vec::with_capacity(min * height);
            table.extend(iter);

            (height, table)
        } else {
            let mut table = Vec::from_iter(iter);
            let height = table.len().next_power_of_two().trailing_zeros() as usize;
            table.reserve(table.len() * height.saturating_sub(1));

            (height, table)
        };

        let n = table.len();
        let mut stack = Vec::with_capacity(n);
        for b in (1..height).map(|i| 1 << i) {
            // TODO: use div_ceil()
            for i in 0..(n + b - 1) / b {
                if i & 1 == 1 {
                    stack.push(table[i * b].clone());
                    for v in &table[i * b + 1..n.min(i * b + b)] {
                        stack.push(stack.last().unwrap().binary_operation(v));
                    }
                    for i in 0..stack.len() >> 1 {
                        table.push(stack.swap_remove(i));
                    }
                    while let Some(v) = stack.pop() {
                        table.push(v);
                    }
                } else {
                    stack.push(table[n.min(i * b + b) - 1].clone()); // n > 0 and b > 0
                    for v in table[i * b..n.min(i * b + b) - 1].iter().rev() {
                        stack.push(stack.last().unwrap().binary_operation(v));
                    }
                    while let Some(v) = stack.pop() {
                        table.push(v);
                    }
                }
            }
        }

        Self {
            table: table.into_boxed_slice(),
            len: n,
        }
    }
}

impl<T: Semigroup + Clone> From<Vec<T>> for DisjointSparseTable<T> {
    fn from(value: Vec<T>) -> Self {
        Self::from_iter(value)
    }
}
