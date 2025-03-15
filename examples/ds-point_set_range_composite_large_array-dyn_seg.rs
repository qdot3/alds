// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite_large_array

use mod_int::SMint;
use proconio::{fastout, input};
use segment_tree::{DynamicSegmentTree, Monoid};

type Mint = SMint<998_244_353>;

#[fastout]
fn main() {
    input! { n: isize, q: usize, }

    let mut dst = DynamicSegmentTree::with_capacity(q, 0..n);

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: isize, c: u64, d: u64, }

            dst.point_set(p, Affine::new(c, d));
        } else if flag == 1 {
            input! { l: isize, r: isize, x: u64, }

            println!("{}", dst.range_query(l..r).apply(Mint::new(x)));
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Affine {
    tilt: Mint,
    offset: Mint,
}

impl Affine {
    fn new(tilt: u64, offset: u64) -> Self {
        Self {
            tilt: Mint::new(tilt),
            offset: Mint::new(offset),
        }
    }

    fn apply(&self, arg: Mint) -> Mint {
        self.tilt * arg + self.offset
    }
}

impl Monoid for Affine {
    const IS_COMMUTATIVE: bool = false;
    
    fn identity() -> Self {
        Self::new(1, 0)
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        Self {
            tilt: rhs.tilt * self.tilt,
            offset: rhs.tilt * self.offset + rhs.offset,
        }
    }
}
