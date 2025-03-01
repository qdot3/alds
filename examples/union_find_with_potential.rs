// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential

use alds::{
    modint::Mint,
    union_find::{Group, UnionFindWithPotential},
};
use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! { n: usize, q: usize, }

    let mut uf = UnionFindWithPotential::new(n);
    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { u: usize, v: usize, x_uv: u64, }

            if uf.unite(u, v, Potential(Mint::new(x_uv))).is_ok() {
                println!("1")
            } else {
                println!("0")
            }
        } else if flag == 1 {
            input! { u: usize, v: usize, }

            if let Some(p_uv) = uf.potential(u, v) {
                println!("{}", p_uv.0)
            } else {
                println!("-1")
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Potential(Mint<MOD>);

impl Group for Potential {
    fn identity() -> Self {
        Self(Mint::new(0))
    }

    fn binary_operation(&self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    fn inverse(&self) -> Self {
        Self(-self.0)
    }
}
