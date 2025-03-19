// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use mod_int::SMint;
use proconio::{fastout, input};
use segment_tree::{LazySegmentTree, Monoid, MonoidAct};

type Mint = SMint<998_244_353>;

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    let mut lst =
        LazySegmentTree::<Affine>::from(Vec::from_iter(a.into_iter().map(|a| SUM::new(a))));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, b: u64, c: u64, }

            lst.range_update(l..r, Affine::new(b, c));
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", lst.range_query(l..r).sum);
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
struct SUM {
    sum: Mint,
    size: Mint,
}

impl SUM {
    fn new(value: u64) -> Self {
        Self {
            sum: SMint::new(value),
            size: SMint::new(1),
        }
    }
}

impl Monoid for SUM {
    const IS_COMMUTATIVE: bool = true;

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
struct Affine {
    tilt: Mint,
    offset: Mint,
}

impl Affine {
    fn new(tilt: u64, offset: u64) -> Self {
        Self {
            tilt: SMint::new(tilt),
            offset: SMint::new(offset),
        }
    }
}

impl MonoidAct for Affine {
    type Arg = SUM;
    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self {
        Self::new(1, 0)
    }

    fn apply(&self, arg: &Self::Arg) -> Self::Arg {
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
