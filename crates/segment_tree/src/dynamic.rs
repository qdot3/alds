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

        let mut p = 0;
        let n = self.arena.len();
        let Range { mut start, mut end } = self.range;
        while let Some(node) = self.arena.get_mut(p) {
            self.reusable_buf.push(p);

            if node.index == i {
                node.value = value;
                break;
            }

            if i < (start + end) >> 1 {
                if i > node.index {
                    // `product` will be broken but recalculated after
                    std::mem::swap(&mut i, &mut node.index);
                    std::mem::swap(&mut value, &mut node.value);
                }

                if let Some(c) = node.left {
                    p = c;
                    end = (start + end) >> 1;
                } else {
                    node.left.replace(n);
                    self.arena.push(Node::new(i, value));
                    break;
                }
            } else {
                if i < node.index {
                    std::mem::swap(&mut i, &mut node.index);
                    std::mem::swap(&mut value, &mut node.value);
                }

                if let Some(c) = node.right {
                    p = c;
                    start = (start + end) >> 1;
                } else {
                    node.right.replace(n);
                    self.arena.push(Node::new(i, value));
                    break;
                }
            }
        }

        // recalculate product
        for i in self.reusable_buf.drain(..).rev() {
            self.arena[i].product = match (self.arena[i].left, self.arena[i].right) {
                (None, Some(r)) => self.arena[i].value.binary_operation(&self.arena[r].product),
                (Some(l), None) => self.arena[l].product.binary_operation(&self.arena[i].value),
                (Some(l), Some(r)) => (self.arena[l].product)
                    .binary_operation(&self.arena[i].value)
                    .binary_operation(&self.arena[r].product),
                (None, None) => self.arena[i].value.clone(),
            };
        }
    }

    pub fn range_query<R>(&self, range: R) -> T
    where
        R: RangeBounds<isize>,
    {
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
        if let Some(node) = self.arena.get(i) {
            if l == start && r == end {
                return node.product.clone();
            }

            let mid = (start + end) >> 1;
            if r <= mid {
                if let Some(i) = node.left {
                    return self.rec_query(i, l, r, start, mid);
                } else if (l..r).contains(&node.index) {
                    return node.value.clone();
                } else {
                    return T::identity();
                }
            } else if l >= mid {
                if let Some(i) = node.right {
                    return self.rec_query(i, l, r, mid, end);
                } else if (l..r).contains(&node.index) {
                    return node.value.clone();
                } else {
                    return T::identity();
                }
            }

            (self.rec_query(node.left.unwrap_or(usize::MAX), l, mid, start, mid))
                .binary_operation(&node.value)
                .binary_operation(&self.rec_query(
                    node.right.unwrap_or(usize::MAX),
                    mid,
                    r,
                    mid,
                    end,
                ))
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
