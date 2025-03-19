use std::ops::RangeBounds;

use crate::{Monoid, MonoidAct};

/// A segment tree that supports range updates and range queries.
///
/// # Size dependent operations
///
/// If
pub struct LazySegmentTree<F: MonoidAct + Clone> {
    data: Box<[<F as MonoidAct>::Arg]>,
    /// store pending actions
    lazy: Box<[F]>,
    len: usize,
    buf_len: usize,
    height: u32,
}

impl<F: MonoidAct + Clone> LazySegmentTree<F> {
    pub fn new(n: usize) -> Self {
        assert!(n > 0 && n < usize::MAX);

        let buf_len = n.next_power_of_two(); // non-commutative monoid
        let height = buf_len.trailing_zeros() + 1;
        let data = Vec::from_iter(
            std::iter::repeat_with(|| <F as MonoidAct>::Arg::identity()).take(n + n % 2 + buf_len),
        ) // save space
        .into_boxed_slice();
        let lazy = Vec::from_iter(std::iter::repeat_with(|| F::identity()).take(buf_len))
            .into_boxed_slice();

        Self {
            data,
            lazy,
            len: n,
            buf_len,
            height,
        }
    }

    const fn inner_index(&self, i: usize) -> usize {
        self.buf_len + i
    }

    /// Returns `[l, r)`
    fn inner_range<R>(&self, range: R) -> (usize, usize)
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.len,
        };

        (self.inner_index(l), self.inner_index(r))
    }

    fn update_node(&mut self, i: usize) {
        self.data[i] = self.data[i * 2].binary_operation(&self.data[i * 2 + 1])
    }

    fn apply_action(&mut self, i: usize, action: F) {
        self.data[i] = action.apply(&self.data[i]);
        if i < self.buf_len {
            // apply `action` after `lazy[i]`
            self.lazy[i] = action.composite(&self.lazy[i])
        }
    }

    fn apply_pending_action(&mut self, i: usize) {
        self.apply_action(i * 2, self.lazy[i].clone());
        self.apply_action(i * 2 + 1, self.lazy[i].clone());
        self.lazy[i] = F::identity()
    }

    pub fn get(&mut self, i: usize) -> &<F as MonoidAct>::Arg {
        let i = self.inner_index(i);

        // apply pending actions
        for d in (1..=self.height).rev() {
            self.apply_pending_action(i >> d);
        }

        &self.data[i]
    }

    pub fn set(&mut self, i: usize, value: <F as MonoidAct>::Arg) {
        // apply pending actions
        self.get(i);

        // update data
        let i = self.inner_index(i);
        self.data[i] = value;
        for d in 1..=self.height {
            self.update_node(i >> d);
        }
    }

    pub fn apply<R>(&mut self, range: R, action: F)
    where
        R: RangeBounds<usize>,
    {
        let (l, r) = self.inner_range(range);

        // apply pending actions
        for d in (1..=self.height).rev() {
            // avoid unnecessary propagation
            if (l >> d) << d != l {
                self.apply_pending_action(l >> d);
            }
            if (r >> d) << d != r {
                self.apply_pending_action((r - 1) >> d);
            }
        }

        // apply `action` in a lazy way
        {
            let (mut l, mut r) = (l, r);
            while l < r {
                if l % 2 == 1 {
                    self.apply_action(l, action.clone());
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    self.apply_action(r, action.clone());
                }

                l /= 2;
                r /= 2;
            }
        }

        // update parents of modified nodes
        for d in 1..=self.height {
            // avoid updating node with children which has not been updated
            if (l >> d) << d != l {
                self.update_node(l >> d);
            }
            if (r >> d) << d != r {
                self.update_node((r - 1) >> d);
            }
        }
    }

    /// Propagates all pending updates and brings all nodes to their latest state.
    ///
    /// This is useful in scenarios where lazy propagation might be expensive,
    /// such as in [Range Set Range Composite (Library Checker)](https://judge.yosupo.jp/problem/range_set_range_composite).
    /// Periodically applying all updates helps maintain efficiency by reducing memory overhead.
    pub fn apply_all(&mut self) {
        for i in 1..self.data.len() / 2 {
            self.apply_pending_action(i);
        }
        for i in (1..self.data.len() / 2).rev() {
            self.update_node(i);
        }
    }

    pub fn eval<R>(&mut self, range: R) -> <F as MonoidAct>::Arg
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.inner_range(range);

        if l >= r {
            return <F as MonoidAct>::Arg::identity();
        }

        // apply pending actions
        for d in (1..=self.height).rev() {
            // avoid unnecessary propagation
            if (l >> d) << d != l {
                self.apply_pending_action(l >> d);
            }
            if (r >> d) << d != r {
                self.apply_pending_action(r >> d); // `(r >> d) % 2 = 1`
            }
        }

        // calculate result
        let (mut res_l, mut res_r) = (
            <F as MonoidAct>::Arg::identity(),
            <F as MonoidAct>::Arg::identity(),
        );
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

impl<F: MonoidAct + Clone> From<Vec<<F as MonoidAct>::Arg>> for LazySegmentTree<F> {
    fn from(values: Vec<<F as MonoidAct>::Arg>) -> Self {
        let len = values.len();
        let buf_len = len.next_power_of_two(); // non-commutative monoid
        let height = buf_len.trailing_zeros() + 1;
        let mut data = Vec::from_iter(
            std::iter::repeat_with(|| <F as MonoidAct>::Arg::identity())
                .take(buf_len)
                .chain(values)
                .chain(std::iter::repeat_with(|| <F as MonoidAct>::Arg::identity()).take(len % 2)), // save space
        )
        .into_boxed_slice();
        for i in (1..data.len() / 2).rev() {
            data[i] = data[i * 2].binary_operation(&data[i * 2 + 1])
        }

        let lazy = Vec::from_iter(std::iter::repeat_with(|| F::identity()).take(buf_len))
            .into_boxed_slice();

        Self {
            data,
            lazy,
            len,
            buf_len,
            height,
        }
    }
}
