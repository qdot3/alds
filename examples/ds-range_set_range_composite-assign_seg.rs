use mod_int::SMint;
use proconio::{fastout, input};
use segment_tree::{AssignSegmentTree, MonoidAct};

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

            ast.update(l..r, Affine::new(c, d));
        } else if flag == 1 {
            input! { l: usize, r: usize, x: u64, }

            let res = ast.product(l..r).apply(&Mint::new(x));
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
    fn new(tilt: u64, offset: u64) -> Self {
        Self {
            tilt: Mint::new(tilt),
            offset: Mint::new(offset),
        }
    }
}

impl MonoidAct for Affine {
    type Arg = Mint;

    fn identity() -> Self {
        Self::new(1, 0)
    }

    fn composite(&self, rhs: &Self) -> Self {
        Self {
            tilt: rhs.tilt * self.tilt,
            offset: rhs.tilt * self.offset + rhs.offset,
        }
    }

    fn apply(&self, arg: &Self::Arg) -> Self::Arg {
        self.tilt * arg + self.offset
    }
}
