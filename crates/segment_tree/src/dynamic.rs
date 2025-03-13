use std::{
    ops::{Range, RangeBounds},
    usize,
};

use crate::Monoid;

#[derive(Debug, Clone)]
pub struct DynamicSegmentTree<T: Monoid> {
    arena: Vec<Node<T>>,
    range: Range<isize>,
    reusable_buf: Vec<usize>,
}

impl<T: Monoid + Clone> DynamicSegmentTree<T> {
    pub fn with_capacity(capacity: usize, range: Range<isize>) -> Self {
        Self {
            arena: Vec::with_capacity(capacity),
            range,
            reusable_buf: Vec::new(),
        }
    }

    pub fn point_set(&mut self, mut i: isize, mut value: T) {
        if self.arena.is_empty() {
            self.arena.push(Node::new(i, value));
            return;
        }

        let Self {
            arena,
            range,
            reusable_buf,
        } = self;

        let mut p = 0;
        let Range { mut start, mut end } = range;
        loop {
            reusable_buf.push(p);

            if arena[p].index == i {
                arena[p].value = value;
                break;
            }

            let mid = (start + end) >> 1;
            if i < mid {
                if i > arena[p].index {
                    // `product` will be broken but recalculated after
                    std::mem::swap(&mut i, &mut arena[p].index);
                    std::mem::swap(&mut value, &mut arena[p].value);
                }

                if let Some(l) = arena[p].left {
                    p = l;
                    end = mid;
                    continue;
                } else {
                    let n = arena.len();
                    arena[p].left.replace(n);
                    arena.push(Node::new(i, value));
                    break;
                }
            } else {
                if i < arena[p].index {
                    std::mem::swap(&mut i, &mut arena[p].index);
                    std::mem::swap(&mut value, &mut arena[p].value);
                }

                if let Some(r) = arena[p].right {
                    p = r;
                    start = mid;
                    continue;
                } else {
                    let n = arena.len();
                    arena[p].right.replace(n);
                    arena.push(Node::new(i, value));
                    break;
                }
            }
        }

        // recalculate product
        for i in reusable_buf.drain(..).rev() {
            arena[i].product = match (arena[i].left, arena[i].right) {
                (None, Some(r)) => arena[i].value.binary_operation(&arena[r].product),
                (Some(l), None) => arena[l].product.binary_operation(&arena[i].value),
                (Some(l), Some(r)) => (arena[l].product)
                    .binary_operation(&arena[i].value)
                    .binary_operation(&arena[r].product),
                (None, None) => arena[i].value.clone(),
            };
        }
    }

    pub fn range_query<R>(&self, range: R) -> T
    where
        R: RangeBounds<isize>,
    {
        if self.arena.is_empty() {
            return T::identity();
        }

        let Range { start, end } = self.range;
        let l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => start,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => end,
        };

        if l == start && r == end {
            return self.arena[0].product.clone();
        }
        if l >= r {
            return T::identity();
        }

        self.rec_query(0, l, r, start, end)
    }

    fn rec_query(&self, i: usize, l: isize, r: isize, start: isize, end: isize) -> T {
        if l >= end || r < start {
            return T::identity();
        }

        if let Some(node) = self.arena.get(i) {
            if l == start && r == end {
                return node.product.clone();
            }

            let mid = (start + end) >> 1;
            let mut res = self.rec_query(node.left.unwrap_or(usize::MAX), l, mid, start, mid);
            if (l..r).contains(&node.index) {
                res = res.binary_operation(&node.value)
            }
            res.binary_operation(
                &(self.rec_query(node.right.unwrap_or(usize::MAX), mid, r, mid, end)),
            )
        } else {
            T::identity()
        }
    }
}

#[derive(Debug, Clone)]
struct Node<T> {
    index: isize,
    value: T,
    product: T,
    left: Option<usize>,
    right: Option<usize>,
}

impl<T: Clone> Node<T> {
    fn new(index: isize, value: T) -> Self {
        Self {
            index,
            product: value.clone(),
            value,
            left: None,
            right: None,
        }
    }
}
