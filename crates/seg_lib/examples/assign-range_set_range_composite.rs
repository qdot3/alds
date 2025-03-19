// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_set_range_composite

use mod_int::SMint;
use proconio::{fastout, input};
use seg_lib::{AssignSegmentTree, Monoid};

type Mint = SMint<998_244_353>;

#[fastout]
fn main() {
    input! { n: usize, q: usize, ab: [(u64, u64); n], }

    let mut ast = AssignSegmentTree::from(Vec::from_iter(
        ab.into_iter().map(|(a, b)| Affine::new(a, b)),
    ));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, c: u64, d: u64, }

            ast.assign(l..r, Affine::new(c, d));
        } else if flag == 1 {
            input! { l: usize, r: usize, x: u64, }

            let res = ast.composite(l..r).apply(Mint::new(x));
            println!("{}", res)
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone, Copy)]
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
