// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::{fastout, input};
use segment_tree::{LazySegmentTree, Map, Monoid};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    // test `from`
    let mut seg_tree = LazySegmentTree::<_, F>::from(Vec::from_iter(a.into_iter().map(|a| M(a))));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: usize, x: u64, }

            let value = M(x + seg_tree.get(p).0);
            seg_tree.set(p, value);
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", seg_tree.query(l..r).0)
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone)]
struct M(u64);

impl Monoid for M {
    fn identity() -> Self {
        Self(0)
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

#[derive(Clone)]
struct F(u64);

impl Map<M> for F {
    fn identity() -> Self {
        F(0)
    }

    fn apply(&self, x: &M, size: usize) -> M {
        M(self.0 * size as u64 + x.0)
    }

    fn compose(&self, rhs: &Self) -> Self {
        F(self.0 + rhs.0)
    }
}
