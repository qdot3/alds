// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum

use proconio::{fastout, input};
use sparse_table::{DisjointSparseTable, Semigroup};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], lr: [(usize, usize); q], }

    let dst = DisjointSparseTable::from_iter(a.into_iter().map(|a| RangeSum(a)));
    for (l, r) in lr {
        println!("{}", dst.range_query(l..r).unwrap().0)
    }
}

#[derive(Debug, Clone)]
struct RangeSum(u64);

impl Semigroup for RangeSum {
    fn binary_operation(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
