// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use mod_int::SMint;
use proconio::{fastout, input};
use segment_tree::{Monoid, SegmentTree};

#[fastout]
fn main() {
    input! { n: usize, q: usize, ab: [(u64, u64); n], }

    const MOD: u64 = 998_244_353;
    let mut seg_tree = SegmentTree::from(Vec::from_iter(
        ab.into_iter()
            .map(|(a, b)| LinearFunction::<MOD>::new(a, b)),
    ));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: usize, c: u64, d: u64, }

            seg_tree.set(p, LinearFunction::new(c, d));
        } else if flag == 1 {
            input! { l: usize, r: usize, x: u64, }

            let res = seg_tree.eval(l..r).calc(SMint::new(x));
            println!("{}", res)
        } else {
            unreachable!()
        }
    }
}

struct LinearFunction<const MOD: u64> {
    tilt: SMint<MOD>,
    offset: SMint<MOD>,
}

impl<const MOD: u64> LinearFunction<MOD> {
    fn new(tile: u64, offset: u64) -> Self {
        Self {
            tilt: SMint::new(tile),
            offset: SMint::new(offset),
        }
    }

    fn calc(&self, x: SMint<MOD>) -> SMint<MOD> {
        self.tilt * x + self.offset
    }
}

impl<const MOD: u64> Monoid for LinearFunction<MOD> {
    fn identity() -> Self {
        Self {
            tilt: SMint::new(1),
            offset: SMint::new(0),
        }
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        Self {
            tilt: rhs.tilt * self.tilt,
            offset: rhs.tilt * self.offset + rhs.offset,
        }
    }
}
