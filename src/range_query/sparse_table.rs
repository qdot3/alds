use std::ops::Range;

pub struct SparseTable<T> {
    table: Vec<Vec<T>>,
    op: Box<dyn Fn(&T, &T) -> T>,
}

impl<T> SparseTable<T> {
    pub fn from_vec(values: Vec<T>, op: impl Fn(&T, &T) -> T + 'static) -> Self {
        let n = values.len();
        let d = n.next_power_of_two().ilog2() as usize;
        let mut table = Vec::with_capacity(d);
        table.push(values);
        for i in 0..d {
            let w = 1 << i;
            let mut row = Vec::with_capacity(n - w + 1);
            for j in w..table[i].len() {
                row.push(op(&table[i][j - w], &table[i][j]));
            }
            table.push(row);
        }

        Self {
            table,
            op: Box::new(op),
        }
    }

    pub fn query(&self, range: Range<usize>) -> Option<T> {
        let Range { start, end } = range;

        if start < end && end <= self.table[0].len() {
            let d = (end - start).ilog2() as usize;
            let res = (self.op)(&self.table[d][start], &self.table[d][end - (1 << d)]);
            Some(res)
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
        let st = {
            let values = Vec::from_iter(0..6);
            SparseTable::from_vec(values, |l, r| *l.max(r))
        };

        assert_eq!(
            st.table,
            vec![
                Vec::from_iter(0..6),
                Vec::from_iter(1..6),
                Vec::from_iter(3..6)
            ]
        );
    }

    #[test]
    fn test_query() {
        let st = {
            let values = Vec::from_iter(0..100);
            SparseTable::from_vec(values, |l, r| *l.min(r))
        };

        assert_eq!(st.query(0..1), Some(0));
        assert_eq!(st.query(0..64), Some(0));
        assert_eq!(st.query(15..99), Some(15));
        assert_eq!(st.query(50..50), None);
        assert_eq!(st.query(50..30), None);
        assert_eq!(st.query(0..1000), None);

        let st = {
            let values = Vec::from_iter(0..100);
            SparseTable::from_vec(values, |l, r| *l.max(r))
        };

        assert_eq!(st.query(0..1), Some(0));
        assert_eq!(st.query(0..64), Some(63));
        assert_eq!(st.query(15..100), Some(99));
        assert_eq!(st.query(50..50), None);
        assert_eq!(st.query(50..30), None);
        assert_eq!(st.query(0..1000), None);
    }
}
