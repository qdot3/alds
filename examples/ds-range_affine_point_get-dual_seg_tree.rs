// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get

use mod_int::SMint;
use proconio::{fastout, input};
use segment_tree::{DualSegmentTree, MonoidAction};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    const MOD: u64 = 998_244_353;
    let mut dst =
        DualSegmentTree::from(Vec::from_iter(a.into_iter().map(|a| SMint::<MOD>::new(a))));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, b: u64, c: u64, }

            dst.apply(l..r, Affine::new(b, c));
        } else if flag == 1 {
            input! { i: usize, }

            println!("{}", dst.get(i));
            // println!("{:#?}", dst)
        } else {
            unreachable!()
        }
    }
}

struct Affine<const MOD: u64> {
    tilt: SMint<MOD>,
    offset: SMint<MOD>,
}

impl<const MOD: u64> Affine<MOD> {
    fn new(tilt: u64, offset: u64) -> Self {
        Self {
            tilt: SMint::new(tilt),
            offset: SMint::new(offset),
        }
    }
}

impl<const MOD: u64> MonoidAction<SMint<MOD>> for Affine<MOD> {
    const IS_COMMUTATIVE: bool = false;
    
    fn identity() -> Self {
        Self::new(1, 0)
    }

    fn apply(&self, arg: &SMint<MOD>) -> SMint<MOD> {
        self.tilt * arg + self.offset
    }

    fn composite(&self, rhs: &Self) -> Self {
        Self {
            tilt: self.tilt * rhs.tilt,
            offset: self.tilt * rhs.offset + self.offset,
        }
    }
}
