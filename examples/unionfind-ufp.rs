// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use alds::union_find::{Group, UnionFindWithPotential};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, q: usize, query: [(u8, usize, usize); q], }

    let mut ufp = UnionFindWithPotential::new(n);
    for (flag, i, j) in query {
        if flag == 0 {
            let _ = ufp.unite(i, j, Potential);
        } else if flag == 1 {
            println!("{}", if ufp.same(i, j) { 1 } else { 0 })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Potential;

impl Group for Potential {
    fn identity() -> Self {
        Self
    }

    fn binary_operation(&self, _rhs: Self) -> Self {
        Self
    }

    fn inverse(&self) -> Self {
        Self
    }
}
