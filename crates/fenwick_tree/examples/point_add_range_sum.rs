// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use fenwick_tree::FenwickTree;
use math_traits::{marker::Commutative, Group};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [i64; n], }

    let mut ft = FenwickTree::from_iter(a.into_iter().map(|a| A(a)));
    for _ in 0..q {
        input! { flag: u8, }
        match flag {
            0 => {
                input! { p: usize, x: i64, }

                ft.point_update(p, A(x))
            }
            1 => {
                input! { l: usize, r: usize, }

                println!("{}", ft.range_query(l..r).0)
            }
            _ => unreachable!(),
        }
    }
}

struct A(i64);
impl Commutative for A {}
impl Group for A {
    fn identity() -> Self {
        Self(0)
    }

    fn bin_op(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }

    fn inverse(&self) -> Self {
        Self(-self.0)
    }
}
