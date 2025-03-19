// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::{fastout, input};
use seg_lib::{Monoid, SegmentTree};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u32; n], query: [(usize, usize); q], }

    let seg_tree = SegmentTree::from(Vec::from_iter(a.into_iter().map(|a| M(a))));

    for (l, r) in query {
        println!("{}", seg_tree.range_query(l..r).0)
    }
}

#[derive(Clone)]
struct M(u32);

impl Monoid for M {
    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self {
        M(!0)
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        M(self.0.min(rhs.0))
    }
}
