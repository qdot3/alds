// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group

use std::ops::Deref;

use mod_int::SMint;
use proconio::{fastout, input};
use union_find::{Group, UnionFindWithPotential};

type Mint = SMint<998_244_353>;

#[fastout]
fn main() {
    input! { n: usize, q: usize, }

    let mut ufp = UnionFindWithPotential::new(n);

    for _ in 0..q {
        input! { flag: u8, }

        match flag {
            0 => {
                input! { u: usize, v: usize, x00: u64, x01: u64, x10: u64, x11: u64, }

                let p_uv = Matrix2x2 {
                    values: [
                        [Mint::new(x00), Mint::new(x01)],
                        [Mint::new(x10), Mint::new(x11)],
                    ],
                };

                if ufp.unite(u, v, p_uv).is_ok() {
                    println!("1")
                } else {
                    println!("0")
                }
            }
            1 => {
                input! { u: usize, v: usize, }

                if let Some(p_uv) = ufp.potential(u, v) {
                    println!(
                        "{} {} {} {}",
                        p_uv[0][0], p_uv[0][1], p_uv[1][0], p_uv[1][1]
                    )
                } else {
                    println!("-1")
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Matrix2x2 {
    values: [[Mint; 2]; 2],
}

impl Deref for Matrix2x2 {
    type Target = [[Mint; 2]; 2];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl Group for Matrix2x2 {
    fn identity() -> Self {
        Self {
            values: [[Mint::new(1), Mint::new(0)], [Mint::new(0), Mint::new(1)]],
        }
    }

    fn binary_operation(&self, rhs: Self) -> Self {
        let mut values = [[Mint::new(0); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    values[i][j] += rhs[i][k] * self[k][j]
                }
            }
        }

        Self { values }
    }

    fn inverse(&self) -> Self {
        let det = self[0][0] * self[1][1] - self[0][1] * self[1][0];
        // constraint of this problem
        assert!(det == Mint::new(1));

        Self {
            values: [[self[1][1], -self[0][1]], [-self[1][0], self[0][0]]],
        }
    }
}
