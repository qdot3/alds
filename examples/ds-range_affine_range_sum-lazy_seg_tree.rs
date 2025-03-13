// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use mod_int::SMint;
use proconio::{fastout, input};
use segment_tree::{LazySegmentTree, Monoid, MonoidAction};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    const MOD: u64 = 998_244_353;
    let mut lst = LazySegmentTree::<SUM<MOD>, Affine<MOD>>::from(Vec::from_iter(
        a.into_iter().map(|a| SUM::new(a)),
    ));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, b: u64, c: u64, }

            lst.apply(l..r, Affine::new(b, c));
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", lst.eval(l..r).sum);
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
struct SUM<const MOD: u64> {
    sum: SMint<MOD>,
    size: SMint<MOD>,
}

impl<const MOD: u64> SUM<MOD> {
    fn new(value: u64) -> Self {
        Self {
            sum: SMint::new(value),
            size: SMint::new(1),
        }
    }
}

impl<const MOD: u64> Monoid for SUM<MOD> {
    fn identity() -> Self {
        Self {
            sum: SMint::new(0),
            size: SMint::new(0),
        }
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        Self {
            sum: self.sum + rhs.sum,
            size: self.size + rhs.size,
        }
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

impl<const MOD: u64> MonoidAction<SUM<MOD>> for Affine<MOD> {
    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self {
        Self::new(1, 0)
    }

    fn apply(&self, arg: &SUM<MOD>) -> SUM<MOD> {
        SUM {
            sum: self.tilt * arg.sum + self.offset * arg.size,
            size: arg.size,
        }
    }

    fn composite(&self, rhs: &Self) -> Self {
        Self {
            tilt: self.tilt * rhs.tilt,
            offset: self.tilt * rhs.offset + self.offset,
        }
    }
}
