// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::{fastout, input};
use segment_tree::{Monoid, SegmentTree};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u32; n], query: [(usize, usize); q], }

    let seg_tree = SegmentTree::from(Vec::from_iter(a.into_iter().map(|a| M(a))));

    for (l, r) in query {
        println!("{}", seg_tree.query(l..r).0)
    }
}

struct M(u32);

impl Monoid for M {
    fn identity() -> Self {
        M(!0)
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        M(self.0.min(rhs.0))
    }
}
