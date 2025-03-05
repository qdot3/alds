// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get

use mod_int::SMint;
use proconio::{fastout, input};
use segment_tree::{LazySegmentTree, Map, Monoid};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    const MOD: u64 = 998_244_353;
    let mut lst = LazySegmentTree::<NoneOp<MOD>, Affine<MOD>>::from(Vec::from_iter(
        a.into_iter().map(|a| NoneOp(SMint::new(a))),
    ));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, b: u64, c: u64, }

            lst.apply(l..r, Affine::new(b, c));
        } else if flag == 1 {
            input! { i: usize, }

            println!("{}", lst.get(i).0)
        } else {
            unreachable!()
        }
    }
}

struct NoneOp<const MOD: u64>(SMint<MOD>);

impl<const MOD: u64> Monoid for NoneOp<MOD> {
    fn identity() -> Self {
        Self(SMint::new(0))
    }

    fn binary_operation(&self, _rhs: &Self) -> Self {
        Self::identity()
    }
}

#[derive(Clone)]
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

impl<const MOD: u64> Map<NoneOp<MOD>> for Affine<MOD> {
    fn identity() -> Self {
        Self::new(1, 0)
    }

    fn apply(&self, x: &NoneOp<MOD>, _size: usize) -> NoneOp<MOD> {
        NoneOp(self.tilt * x.0 + self.offset)
    }

    fn compose(&self, rhs: &Self) -> Self {
        Self {
            tilt: self.tilt * rhs.tilt,
            offset: self.tilt * rhs.offset + self.offset,
        }
    }
}
