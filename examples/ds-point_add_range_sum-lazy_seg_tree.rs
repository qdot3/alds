// verification-helper: PRO/BLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::{fastout, input};
use seg_lib::{LazySegmentTree, Monoid, MonoidAct};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    // test `from`
    let mut seg_tree = LazySegmentTree::<F>::from(Vec::from_iter(a.into_iter().map(|a| M::new(a))));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: usize, x: u64, }

            seg_tree.point_update(p, F(x));
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", seg_tree.range_query(l..r).value)
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone)]
struct M {
    value: u64,
    size: u64,
}

impl M {
    fn new(value: u64) -> Self {
        Self { value, size: 1 }
    }
}

impl Monoid for M {
    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self {
        Self::new(0)
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        Self {
            value: self.value + rhs.value,
            size: self.size + rhs.size,
        }
    }
}

#[derive(Clone)]
struct F(u64);

impl MonoidAct for F {
    type Arg = M;
    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self {
        F(0)
    }

    fn apply(&self, arg: &Self::Arg) -> Self::Arg {
        M {
            value: self.0 * arg.size as u64 + arg.value,
            size: arg.size,
        }
    }

    fn composite(&self, rhs: &Self) -> Self {
        F(self.0 + rhs.0)
    }
}
