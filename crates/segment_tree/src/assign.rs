use std::ops::RangeBounds;

use crate::MonoidAct;

/// A segment tree specialized for efficiently assigning functions to consecutive elements
/// and composing them over a range.
///
/// More precisely, the set of functions with the assignment operation `(F, =)`, where `F`
/// forms a semigroup, naturally extends to a monoid structure when lifted to `Option<F>`.
/// [`AssignSegmentTree`] encapsulates this behavior, providing a structured way to
/// efficiently manage function assignments.
///
/// # Comparison with Other Segment Tree Variants
///
/// - [`LazySegmentTree`](crate::LazySegmentTree): While a [`LazySegmentTree`](crate::LazySegmentTree) supports
///   more general range updates, [`AssignSegmentTree`] offers a simpler API and can be more efficient
///   when the cost of repeated function composition is high.
/// - [`DualSegmentTree`](crate::DualSegmentTree): Unlike [`AssignSegmentTree`], which ensures that
///   newer assignments override older ones, [`DualSegmentTree`](crate::DualSegmentTree) applies
///   function composition in chronological order.
#[derive(Debug, Clone)]
pub struct AssignSegmentTree<F: MonoidAct + Copy> {
    /// `data.len()` will be even for simplicity.
    /// If ths size of given `data` is odd, `F::identity()` will be attached.
    data: Box<[F]>,
    /// `lazy[i] = lazy_pow[lazy_map[i]]`.
    /// Ths size will be `len.next_power_of_two()`.
    lazy_map: Box<[usize]>,
    /// `[(T, 0), (T, 1), .., (T, d)|(U, 0), .., (U, d)|..|(V, 0), .., (V, d)]`, where `(T, n)` represents `T^(2^n)`
    lazy_pow: Vec<F>,
    /// Number of `data`, excluding at most one extended identity element.
    len: usize,
    /// Height of `lazy_map`
    height: u32,
}

impl<F: MonoidAct + Copy> AssignSegmentTree<F> {
    const NULL_ID: usize = !0;

    const fn inner_index(&self, i: usize) -> usize {
        // `self.lazy_map.len()` = 2^d >= i
        self.lazy_map.len() + i
    }

    /// Returns `[l, r)`
    fn inner_range<R>(&self, range: R) -> (usize, usize)
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

        (self.inner_index(l), self.inner_index(r))
    }

    /// Updates `data[i]`.
    fn update(&mut self, i: usize) {
        self.data[i] = self.data[i << 1].composite(&self.data[(i << 1) | 1])
    }

    /// Updates all `data` **without** pending operations.
    fn update_all(&mut self) {
        for i in (1..self.data.len() >> 1).rev() {
            self.update(i);
        }
    }

    /// Assign `lazy_pow[lazy_map[a]]` to `data[i]` and puts propagation toward bottom on hold.
    fn push(&mut self, i: usize, act_id: usize) {
        if act_id != Self::NULL_ID {
            self.data[i] = self.lazy_pow[act_id];
            if let Some(prev) = self.lazy_map.get_mut(i) {
                *prev = act_id
            }
        }
    }

    /// Propagates pending operation stored in `lazy[i]`.
    fn propagate(&mut self, i: usize) {
        let act_id = std::mem::replace(&mut self.lazy_map[i], Self::NULL_ID);
        if act_id != Self::NULL_ID {
            self.push(i << 1, act_id - 1);
            self.push((i << 1) | 1, act_id - 1);
        }
    }

    /// Propagates all pending operations but updates **no** `data`.
    fn propagate_all(&mut self) {
        for i in 1..self.data.len() >> 1 {
            self.propagate(i);
        }
    }

    pub fn get(&mut self, i: usize) -> F {
        let i = self.inner_index(i);

        // propagate pending updates if necessary.
        for d in (1..=self.height).rev() {
            self.propagate(i >> d);
        }

        self.data[i]
    }

    pub fn composite<R>(&mut self, range: R) -> F
    where
        R: RangeBounds<usize>,
    {
        let (l, r) = self.inner_range(range);

        if l + 1 == r {
            return self.get(l - self.lazy_map.len());
        }
        if [l, r] == [self.lazy_map.len(), self.lazy_map.len() + self.len] {
            return self.data[1];
        }
        if l >= r {
            return F::identity();
        }

        // propagate pending updates if necessary.
        let each = (l ^ (r - 1)).ilog2(); // no panic
        for d in (each + 1..=self.height).rev() {
            if (l >> d) << d != l || (r >> d) << d != r {
                self.propagate(l >> d);
            }
        }
        for d in (1..=each).rev() {
            if (l >> d) << d != l {
                self.propagate(l >> d);
            }
            if (r >> d) << d != r {
                self.propagate((r - 1) >> d);
            }
        }

        let (mut l, mut r) = (l, r);
        let (mut res_l, mut res_r) = (F::identity(), F::identity());
        while l < r {
            if l & 1 == 1 {
                res_l = res_l.composite(&self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r ^= 1;
                res_r = self.data[r].composite(&res_r);
            }
            l >>= 1;
            r >>= 1;
        }
        res_l.composite(&res_r)
    }

    pub fn set(&mut self, i: usize, act: F) -> F {
        let i = self.inner_index(i);

        // propagate pending updates if necessary.
        for d in (1..=self.height).rev() {
            self.propagate(i >> d);
        }

        let prev = std::mem::replace(&mut self.data[i], act);

        // updates data
        for d in 1..=self.height {
            self.update(i >> d);
        }

        prev
    }

    pub fn assign<R>(&mut self, range: R, act: F)
    where
        R: RangeBounds<usize>,
    {
        let (l, r) = self.inner_range(range);
        if l + 1 == r {
            self.set(l - self.lazy_map.len(), act);
            return;
        }

        // 1. propagate pending updates if necessary.
        // 2. calculate `act.pow(block_size)`
        let mut id = self.lazy_pow.len();
        let mut pow_act = act;
        let each = (l ^ (r - 1)).ilog2(); // no panic
        for d in (each + 1..=self.height).rev() {
            if (l >> d) << d != l || (r >> d) << d != r {
                self.propagate(l >> d);
            }

            self.lazy_pow.push(pow_act);
            pow_act = pow_act.composite(&pow_act)
        }
        for d in (1..=each).rev() {
            if (l >> d) << d != l {
                self.propagate(l >> d);
            }
            if (r >> d) << d != r {
                self.propagate((r - 1) >> d);
            }

            self.lazy_pow.push(pow_act);
            pow_act = pow_act.composite(&pow_act)
        }
        self.lazy_pow.push(pow_act);

        // put correct `act_id` to `lazy_map`
        {
            let (mut l, mut r) = (l, r);
            while l < r {
                if l & 1 == 1 {
                    self.push(l, id);
                    l += 1;
                }
                if r & 1 == 1 {
                    r ^= 1;
                    self.push(r, id);
                }
                l >>= 1;
                r >>= 1;
                id += 1;
            }
        }

        // update `data`
        if self.lazy_pow.len() < self.data.len() {
            for d in 1..=self.height {
                if (l >> d) << d != l {
                    self.update(l >> d);
                }
                if (r >> d) << d != r {
                    self.update((r - 1) >> d);
                }
            }
        } else {
            self.propagate_all();
            self.update_all();
            self.lazy_pow.clear();
        }
    }
}

impl<F: MonoidAct + Copy> From<Vec<F>> for AssignSegmentTree<F> {
    fn from(values: Vec<F>) -> Self {
        let len = values.len();
        let buf_len = len.next_power_of_two();
        let data = Vec::from_iter(
            std::iter::repeat_with(|| F::identity())
                .take(buf_len)
                .chain(values)
                .chain(std::iter::repeat_with(|| F::identity()).take(len % 2)),
        )
        .into_boxed_slice();

        let mut res = Self {
            data,
            lazy_map: vec![Self::NULL_ID; buf_len].into_boxed_slice(),
            lazy_pow: Vec::with_capacity(buf_len + len),
            len,
            height: buf_len.trailing_zeros(),
        };
        res.update_all();

        res
    }
}
