// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::{fastout, input};
use sparse_table::{Idempotent, Semigroup, SparseTable};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u32; n], lr: [(usize, usize); q], }

    let st = SparseTable::from_iter(a.into_iter().map(|a| RMQ(a)));
    for (l, r) in lr {
        println!("{}", st.range_query(l..r).unwrap().0)
    }
}

#[derive(Debug)]
struct RMQ(u32);

impl Idempotent for RMQ {}

impl Semigroup for RMQ {
    fn binary_operation(&self, rhs: &Self) -> Self {
        Self(self.0.min(rhs.0))
    }
}
