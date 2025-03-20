// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::{fastout, input};
use sparse_table::{DisjointSparseTable, Semigroup};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u32; n], lr: [(usize, usize); q], }

    let dst = DisjointSparseTable::from_iter(a.into_iter().map(|a| RMQ(a)));
    println!("{:?}", dst);

    for (l, r) in lr {
        println!("{}", dst.range_query(l..r).unwrap().0)
    }
}

#[derive(Debug, Clone)]
struct RMQ(u32);

impl Semigroup for RMQ {
    fn binary_operation(&self, rhs: &Self) -> Self {
        Self(self.0.min(rhs.0))
    }
}
