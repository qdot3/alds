use proconio::{fastout, input};
use sparse_table::{Semigroup, SqrtTable};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], lr: [(usize, usize); q], }

    let sqrt = SqrtTable::from(Vec::from_iter(a.into_iter().map(|a| RangeSum(a))));
    println!("{:?}", sqrt);
    for (l, r) in lr {
        println!("{}", sqrt.range_query(l..r).unwrap().0)
    }
}

#[derive(Debug, Clone)]
struct RangeSum(u64);

impl Semigroup for RangeSum {
    fn binary_operation(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
