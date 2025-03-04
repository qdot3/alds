use std::ops::Range;

use crate::{Map, Monoid};

pub struct LazySegmentTree<T: Monoid, F: Map<T>> {
    data: Box<[T]>,
    /// store pending operations
    lazy: Box<[F]>,
    buf_len: usize,
    height: u32,
}

impl<T: Monoid, F: Map<T>> LazySegmentTree<T, F> {
    pub fn new(n: usize) -> Self {
        assert!(n > 0 && n < usize::MAX);

        let buf_len = n; // one-based indexing
        let data = Vec::from_iter(std::iter::repeat_with(|| T::identity()).take(n + buf_len))
            .into_boxed_slice();
        let lazy = Vec::from_iter(std::iter::repeat_with(|| F::identity()).take(buf_len))
            .into_boxed_slice();
        let height = (n + buf_len).next_power_of_two().trailing_zeros();

        Self {
            data,
            lazy,
            buf_len,
            height,
        }
    }

    const fn inner_index(&self, i: usize) -> usize {
        self.buf_len + i
    }

    fn update_node(&mut self, i: usize) {
        self.data[i] = self.data[i * 2].binary_operation(&self.data[i * 2 + 1])
    }

    fn apply_map(&mut self, i: usize, map: F) {
        let size = 1 << (self.height - i.ilog2());
        self.data[i] = map.apply(&self.data[i], size);
        if i < self.buf_len {
            // apply `map` after `lazy[i]`
            self.lazy[i] = map.compose(&self.lazy[i])
        }
    }

    fn apply_pending_map(&mut self, i: usize) {
        self.apply_map(i * 2, self.lazy[i].clone());
        self.apply_map(i * 2 + 1, self.lazy[i].clone());
        self.lazy[i] = F::identity()
    }

    pub fn get(&mut self, i: usize) -> &T {
        let i = self.inner_index(i);

        // apply pending operations
        for d in (1..self.height).rev() {
            let i = i >> d;
            if i > 0 {
                self.apply_pending_map(i);
            }
        }

        &self.data[i]
    }

    pub fn set(&mut self, i: usize, value: T) {
        // apply pending operations
        self.get(i);

        // update data
        let mut i = self.inner_index(i);
        self.data[i] = value;
        for _ in 1..self.height {
            i /= 2;
            self.data[i] = self.data[i * 2].binary_operation(&self.data[i * 2 + 1])
        }
    }

    fn inner_range(&self, range: Range<usize>) -> Range<usize> {
        let Range { mut start, mut end } = range;
        start = self.inner_index(start).min(self.data.len() - 1);
        end = self.inner_index(end).min(self.data.len());

        start..end
    }

    pub fn update(&mut self, range: Range<usize>, map: F) {
        let Range { start: l, end: r } = self.inner_range(range);

        if l == r {
            return;
        }

        // apply pending operations
        for d in (1..self.height).rev() {
            if l >> d > 0 {
                self.apply_pending_map(l >> d);
            }
            if (r - 1) >> d > 0 {
                self.apply_pending_map((r - 1) >> d);
            }
        }

        // apply `map` in lazy way
        {
            let (mut l, mut r) = (l, r);
            while l < r {
                if l % 2 == 1 {
                    self.apply_map(l, map.clone());
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    self.apply_map(r, map.clone());
                }

                l /= 2;
                r /= 2;
            }
        }

        // update parents of modified nodes
        for d in 1..self.height {
            self.update_node(l >> d);
            self.update_node(r >> d);
        }
    }

    pub fn query(&mut self, range: Range<usize>) -> T {
        let Range {
            start: mut l,
            end: mut r,
        } = self.inner_range(range);

        if l == r {
            return T::identity();
        }

        // apply pending operations
        for d in (1..self.height).rev() {
            if l >> d > 0 {
                self.apply_pending_map(l >> d);
            }
            if (r - 1) >> d > 0 {
                self.apply_pending_map((r - 1) >> d);
            }
        }

        // calculate result
        let (mut res_l, mut res_r) = (T::identity(), T::identity());
        while l < r {
            if l % 2 == 1 {
                res_l = res_l.binary_operation(&self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res_r = self.data[r].binary_operation(&res_r);
            }

            l /= 2;
            r /= 2;
        }

        res_l.binary_operation(&res_r)
    }
}

impl<T: Monoid, F: Map<T>> From<Vec<T>> for LazySegmentTree<T, F> {
    fn from(values: Vec<T>) -> Self {
        let buf_len = values.len(); // one-based indexing

        let mut data = Vec::from_iter(
            std::iter::repeat_with(|| T::identity())
                .take(buf_len)
                .chain(values),
        )
        .into_boxed_slice();
        for i in (0..buf_len).rev() {
            data[i] = data[i * 2].binary_operation(&data[i * 2 + 1])
        }

        let lazy = Vec::from_iter(std::iter::repeat_with(|| F::identity()).take(buf_len))
            .into_boxed_slice();

        let height = data.len().next_power_of_two().trailing_zeros();

        Self {
            data,
            lazy,
            buf_len,
            height,
        }
    }
}
