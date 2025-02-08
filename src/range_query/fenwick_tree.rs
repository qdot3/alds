use std::ops::Range;

pub struct FenwickTree {
    // one-based indexing internally (`data[0]` is an identity element or 0 for simpler implementation)
    data: Vec<i64>,
}

impl FenwickTree {
    /// Creates new fixed-size Fenwick tree.
    ///
    /// # Panic
    ///
    /// Panics if `size` is [`usize::MAX`].
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size + 1],
        }
    }

    /// Add `value` to `i`-th element.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::range_query::FenwickTree;
    ///
    /// let mut ft = FenwickTree::new(5);
    ///
    /// ft.add(0, 1);
    /// assert_eq!(ft.query(0..1), 1);
    ///
    /// ft.add(2, 2);
    /// ft.add(4, 3);
    /// assert_eq!(ft.prefix_sum(4), 1 + 2 + 3);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(log *n*)
    pub fn add(&mut self, mut i: usize, value: i64) {
        i += 1;

        while let Some(data) = self.data.get_mut(i) {
            *data += value;
            i += 1 << i.trailing_zeros();
        }
    }

    /// Calculate the sum of elements in `0..i`.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::range_query::FenwickTree;
    ///
    /// let ft = FenwickTree::from(Vec::from_iter(0..100));
    /// assert_eq!(ft.prefix_sum(0), 0);
    /// assert_eq!(ft.prefix_sum(10), (0 + 9) * 10 / 2);
    /// assert_eq!(ft.prefix_sum(100), (0 + 99) * 100 / 2);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(log *n*)
    pub fn prefix_sum(&self, mut i: usize) -> i64 {
        i = i.min(self.data.len() - 1);

        let mut res = self.data[i];
        while i > 0 {
            i -= 1 << i.trailing_zeros();
            res += self.data[i];
        }

        res
    }

    /// Calculate the sum of the range.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::range_query::FenwickTree;
    ///
    /// let ft = FenwickTree::from(Vec::from_iter(0..100));
    ///
    /// assert_eq!(ft.range_sum(0..100), (0 + 99) * 100 / 2);
    /// assert_eq!(ft.range_sum(0..usize::MAX), ft.range_sum(0..100));
    /// assert_eq!(ft.range_sum(1_000..2_000), 0);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(log *n*)
    /// *O*(1) if the range is empty or the length is 1.
    pub fn range_sum(&self, range: Range<usize>) -> i64 {
        if range.is_empty() {
            return 0;
        }

        // including `end`, but excluding `start`
        let Range { mut start, mut end } = range;
        start = start.min(self.data.len() - 1);
        end = end.min(self.data.len() - 1);

        let mut res = 0;
        // if start == end, then the result of remaining operations is net zero.
        while start != end {
            let tz_s = start.trailing_zeros();
            let tz_e = end.trailing_zeros();

            if tz_s <= tz_e {
                res -= self.data[start];
                start ^= 1 << tz_s;
            }
            if tz_e <= tz_s {
                res += self.data[end];
                end ^= 1 << tz_e;
            }
        }

        res
    }

    /// See [`std::slice::partition_point`](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point).
    ///
    /// ```
    /// use alds::range_query::FenwickTree;
    ///
    /// let mut ft = FenwickTree::from(vec![1; 100]);
    /// assert_eq!(ft.partition_point(|v| v < 50), 49);
    ///
    /// ft.add(0, -50);
    /// assert_eq!(ft.partition_point(|v| v < 50), 99);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(log *n*)
    pub fn partition_point(&self, pred: impl Fn(i64) -> bool) -> usize {
        let mut res = 0;
        let mut sum = 0;

        // start from the largest block.
        for d in (0..=self.data.len().ilog2()).rev() {
            if let Some(block) = self.data.get(res + (1 << d)) {
                if pred(sum + block) {
                    res += 1 << d;
                    sum += block
                }
            }
        }

        res
    }

    /// # Example
    ///
    /// ```
    /// use alds::range_query::FenwickTree;
    ///
    /// let mut ft = FenwickTree::from(vec![0; 10]);
    /// for i in 0..5 {
    ///     ft.add(i, i as i64 * 2)
    /// }
    /// assert_eq!(ft.to_vec(), vec![0, 2, 4, 6, 8, 0, 0, 0, 0, 0])
    /// ```
    ///
    /// # Time Complexity
    ///
    /// *O*(*n* log *n*)
    pub fn to_vec(self) -> Vec<i64> {
        let Self { mut data } = self;

        // (n log n) / 2
        for mut i in 1..data.len() {
            let value = data[i];

            // reverse operation of `add` method
            i += 1 << i.trailing_zeros();
            while let Some(data) = data.get_mut(i) {
                *data -= value;
                i += 1 << i.trailing_zeros();
            }
        }

        data.split_off(1)
    }
}

impl From<Vec<i64>> for FenwickTree {
    fn from(value: Vec<i64>) -> Self {
        let mut ft = Self::new(value.len());
        for (i, value) in value.into_iter().enumerate() {
            ft.add(i, value);
        }

        ft
    }
}
