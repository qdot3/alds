use std::ops::Range;

use itertools::Itertools;

pub struct DisjointSparseTable<T> {
    table: Vec<Vec<T>>,
    op: Box<dyn Fn(&T, &T) -> T>,
}

impl<T: Clone> DisjointSparseTable<T> {
    pub fn from_vec(values: Vec<T>, op: impl Fn(&T, &T) -> T + 'static) -> Self {
        let n = values.len();
        let exp = n.next_power_of_two().ilog2() as usize;
        let mut table = Vec::with_capacity(exp);
        for i in (1..exp).rev() {
            let w = 1 << i;

            let mut row = values.clone();
            for (index, reversed) in (0..n)
                .chunks(w)
                .into_iter()
                .zip([true, false].into_iter().cycle())
            {
                if reversed {
                    for i in index.into_iter().collect_vec().into_iter().rev().skip(1) {
                        row[i] = op(&row[i], &row[i + 1])
                    }
                } else {
                    for i in index.into_iter().skip(1) {
                        row[i] = op(&row[i], &row[i - 1])
                    }
                }
            }
            table.push(row);
        }
        table.push(values);

        Self {
            table,
            op: Box::new(op),
        }
    }

    pub fn query(&self, range: Range<usize>) -> Option<T> {
        let Range { start, end } = range;

        if start < end && end <= self.table[0].len() {
            let (l, r) = (start, end - 1);

            let d = self.table.len();
            if l == r {
                Some(self.table[d - 1][l].clone())
            } else {
                let i = d - (l ^ r).ilog2() as usize;
                Some((self.op)(&self.table[i - 1][l], &self.table[i - 1][r]))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let dst = DisjointSparseTable::from_vec(vec![1; 10], |l, r| l + r);

        assert_eq!(
            dst.table,
            vec![
                vec![8, 7, 6, 5, 4, 3, 2, 1, 1, 2],
                vec![4, 3, 2, 1, 1, 2, 3, 4, 2, 1],
                vec![2, 1, 1, 2, 2, 1, 1, 2, 2, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ]
        )
    }

    #[test]
    fn test_query() {
        let dst = DisjointSparseTable::from_vec((0..10).collect_vec(), |l, r| l + r);

        assert_eq!(dst.query(0..1), Some(0));
        assert_eq!(dst.query(0..10), Some(45));
        assert_eq!(dst.query(0..8), Some(28));
        assert_eq!(dst.query(0..0), None);
        assert_eq!(dst.query(10..0), None);
        assert_eq!(dst.query(0..100), None);
    }
}
