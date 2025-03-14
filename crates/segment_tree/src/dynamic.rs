use std::{
    ops::{Range, RangeBounds},
    usize,
};

use crate::Monoid;

#[derive(Debug, Clone)]
pub struct DynamicSegmentTree<T: Monoid> {
    arena: Vec<Node<T>>,
    range: Range<isize>,
    /// save allocation cost. O(log |range|)
    reusable_buf: Vec<usize>,
}

impl<T: Monoid + Clone> DynamicSegmentTree<T> {
    pub fn with_capacity(capacity: usize, range: Range<isize>) -> Self {
        Self {
            arena: Vec::with_capacity(capacity),
            reusable_buf: Vec::with_capacity(range.len().max(2).ilog2() as usize * 2),
            range,
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
                // index of left child should be less than that of parent
                if i > arena[p].index {
                    // `product` will be broken but recalculated after
                    std::mem::swap(&mut i, &mut arena[p].index);
                    std::mem::swap(&mut value, &mut arena[p].value);
                }

                if let Some(l) = arena[p].get_left() {
                    p = l;
                    end = mid;
                    continue;
                } else {
                    let n = arena.len();
                    arena[p].set_left(n);
                    arena.push(Node::new(i, value));
                    break;
                }
            } else {
                if i < arena[p].index {
                    std::mem::swap(&mut i, &mut arena[p].index);
                    std::mem::swap(&mut value, &mut arena[p].value);
                }

                if let Some(r) = arena[p].get_right() {
                    p = r;
                    start = mid;
                    continue;
                } else {
                    let n = arena.len();
                    arena[p].set_right(n);
                    arena.push(Node::new(i, value));
                    break;
                }
            }
        }

        // recalculate `product`
        while let Some(i) = reusable_buf.pop() {
            arena[i].product = match (arena[i].get_left(), arena[i].get_right()) {
                (None, Some(r)) => arena[i].value.binary_operation(&arena[r].product),
                (Some(l), None) => arena[l].product.binary_operation(&arena[i].value),
                (Some(l), Some(r)) => (arena[l].product)
                    .binary_operation(&arena[i].value)
                    .binary_operation(&arena[r].product),
                (None, None) => arena[i].value.clone(),
            };
        }
    }

    pub fn range_query<R>(&mut self, range: R) -> T
    where
        R: RangeBounds<isize>,
    {
        if self.arena.is_empty() {
            return T::identity();
        }

        let Range { mut start, mut end } = self.range;
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

        // recursive version
        // return self.rec_query(0, l, r, start, end);

        // non-recursive version
        #[allow(unreachable_code)]
        {
            let mut p = 0;
            let mut mid = 0;
            while let Some(node) = self.arena.get(p) {
                mid = (start + end) >> 1;
                if l >= mid {
                    if let Some(c) = node.get_right() {
                        if (l..r).contains(&self.arena[p].index) {
                            self.reusable_buf.push(p);
                        }
                        p = c;
                        start = mid;
                        continue;
                    }

                    let mut res = if (l..r).contains(&node.index) {
                        node.value.clone()
                    } else {
                        T::identity()
                    };
                    while let Some(p) = self.reusable_buf.pop() {
                        if p < usize::MAX / 2 {
                            res = self.arena[p].value.binary_operation(&res)
                        } else {
                            res = res.binary_operation(&self.arena[!p].value)
                        }
                    }
                    return res;
                } else if r <= mid {
                    if let Some(c) = node.get_left() {
                        if (l..r).contains(&self.arena[p].index) {
                            // Since maximum size of [Vec] is [isize::MAX], `!p` > [usize::MAX] / 2 >= 'p'
                            self.reusable_buf.push(!p);
                        }
                        p = c;
                        end = mid;
                        continue;
                    }

                    let mut res = if (l..r).contains(&node.index) {
                        node.value.clone()
                    } else {
                        T::identity()
                    };
                    while let Some(p) = self.reusable_buf.pop() {
                        if p < usize::MAX / 2 {
                            res = self.arena[p].value.binary_operation(&res)
                        } else {
                            res = res.binary_operation(&self.arena[!p].value)
                        }
                    }
                    return res;
                } else {
                    break;
                }
            }

            // start <= l < mid < r <= end
            let mut res_l = if let Some(mut p) = self.arena[p].get_left() {
                let mut res_l = T::identity();
                let mut end = mid;
                while let Some(node) = self.arena.get(p) {
                    if l <= start && end <= r {
                        res_l = node.product.binary_operation(&res_l);
                        break;
                    }

                    let mid = (start + end) >> 1;
                    if l < mid {
                        if let Some(c) = node.get_right() {
                            res_l = self.arena[c].product.binary_operation(&res_l)
                        }
                        if (l..r).contains(&node.index) {
                            res_l = node.value.binary_operation(&res_l)
                        }

                        if let Some(c) = node.get_left() {
                            p = c;
                            end = mid;
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        if (l..r).contains(&node.index) {
                            self.reusable_buf.push(p);
                        }

                        if let Some(c) = node.get_right() {
                            p = c;
                            start = mid;
                            continue;
                        } else {
                            break;
                        }
                    }
                }

                res_l
            } else {
                T::identity()
            };

            if (l..r).contains(&self.arena[p].index) {
                res_l = res_l.binary_operation(&self.arena[p].value)
            }

            let mut res_r = if let Some(mut p) = self.arena[p].get_right() {
                let mut res_r = T::identity();
                start = mid;
                while let Some(node) = self.arena.get(p) {
                    if l <= start && end <= r {
                        res_r = res_r.binary_operation(&node.product);
                        break;
                    }

                    mid = (start + end) >> 1;
                    if r <= mid {
                        if (l..r).contains(&node.index) {
                            self.reusable_buf.push(!p);
                        }

                        if let Some(c) = node.get_left() {
                            p = c;
                            end = mid
                        } else {
                            break;
                        }
                    } else {
                        if let Some(c) = node.get_left() {
                            res_r = res_r.binary_operation(&self.arena[c].product)
                        }
                        if (l..r).contains(&node.index) {
                            res_r = res_r.binary_operation(&node.value)
                        }

                        if let Some(c) = node.get_right() {
                            p = c;
                            start = mid;
                            continue;
                        } else {
                            break;
                        }
                    }
                }

                res_r
            } else {
                T::identity()
            };

            while let Some(p) = self.reusable_buf.pop() {
                if p < usize::MAX / 2 {
                    res_l = self.arena[p].value.binary_operation(&res_l)
                } else {
                    res_r = res_r.binary_operation(&self.arena[!p].value)
                }
            }

            res_l.binary_operation(&res_r)
        }
    }

    // recursive version
    #[allow(dead_code)]
    fn rec_query(&self, i: usize, l: isize, r: isize, start: isize, end: isize) -> T {
        if l >= end || r <= start {
            return T::identity();
        }

        if let Some(node) = self.arena.get(i) {
            if l <= start && end <= r {
                return node.product.clone();
            }

            let mid = (start + end) >> 1;
            let mut res = self.rec_query(node.get_left().unwrap_or(usize::MAX), l, r, start, mid);
            if (l..r).contains(&node.index) {
                res = res.binary_operation(&node.value)
            }
            res.binary_operation(
                &(self.rec_query(node.get_right().unwrap_or(usize::MAX), l, r, mid, end)),
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
    left: usize,
    right: usize,
}

impl<T: Clone> Node<T> {
    /// Since maximum capacity of [Vec] is [isize::MAX], [usize::MAX] can be used as `None`
    const NULL_CHILD: usize = usize::MAX;

    #[inline]
    fn new(index: isize, value: T) -> Self {
        Self {
            index,
            product: value.clone(),
            value,
            left: Self::NULL_CHILD,
            right: Self::NULL_CHILD,
        }
    }

    #[inline]
    fn get_left(&self) -> Option<usize> {
        if self.left == Self::NULL_CHILD {
            None
        } else {
            Some(self.left)
        }
    }

    #[inline]
    fn get_right(&self) -> Option<usize> {
        if self.right == Self::NULL_CHILD {
            None
        } else {
            Some(self.right)
        }
    }

    #[inline]
    fn set_left(&mut self, left: usize) {
        self.left = left
    }

    #[inline]
    fn set_right(&mut self, right: usize) {
        self.right = right
    }
}
