// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get

use mod_int::SMint;
use proconio::{fastout, input};
use segment_tree::{DualSegmentTree, Monoid};

type Mint = SMint<998_244_353>;

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    let mut dst = DualSegmentTree::new(n);

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, b: u64, c: u64, }

            dst.range_update(l..r, Affine::new(b, c));
        } else if flag == 1 {
            input! { i: usize, }

            println!("{}", dst.point_query(i).apply(Mint::new(a[i])));
            // println!("{:#?}", dst)
        } else {
            unreachable!()
        }
    }
}

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
            tilt: self.tilt * rhs.tilt,
            offset: self.tilt * rhs.offset + self.offset,
        }
    }
}
