// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use mod_int::SMint;
use proconio::{fastout, input};
use seg_lib::{Monoid, SegmentTree};

type Mint = SMint<998_244_353>;

#[fastout]
fn main() {
    input! { n: usize, q: usize, ab: [(u64, u64); n], }

    let mut seg_tree = SegmentTree::from(Vec::from_iter(
        ab.into_iter().map(|(a, b)| Affine::new(a, b)),
    ));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: usize, c: u64, d: u64, }

            seg_tree.point_update(p, Affine::new(c, d));
        } else if flag == 1 {
            input! { l: usize, r: usize, x: u64, }

            let res = seg_tree.range_query(l..r).calc(SMint::new(x));
            println!("{}", res)
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone)]
struct Affine {
    tilt: Mint,
    offset: Mint,
}

impl Affine {
    fn new(tile: u64, offset: u64) -> Self {
        Self {
            tilt: SMint::new(tile),
            offset: SMint::new(offset),
        }
    }

    fn calc(&self, x: Mint) -> Mint {
        self.tilt * x + self.offset
    }
}

impl Monoid for Affine {
    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self {
        Self {
            tilt: Mint::new(1),
            offset: Mint::new(0),
        }
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        Self {
            tilt: rhs.tilt * self.tilt,
            offset: rhs.tilt * self.offset + rhs.offset,
        }
    }
}
