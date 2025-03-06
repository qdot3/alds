// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::{fastout, input};
use segment_tree::{LazySegmentTree, MonoidAction, Monoid};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    // test `from`
    let mut seg_tree =
        LazySegmentTree::<_, F>::from(Vec::from_iter(a.into_iter().map(|a| M::new(a))));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: usize, x: u64, }

            let value = M::new(x + seg_tree.get(p).value);
            seg_tree.set(p, value);
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", seg_tree.eval(l..r).value)
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

impl MonoidAction<M> for F {
    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self {
        F(0)
    }

    fn apply(&self, arg: &M) -> M {
        M {
            value: self.0 * arg.size as u64 + arg.value,
            size: arg.size,
        }
    }

    fn composite(&self, rhs: &Self) -> Self {
        F(self.0 + rhs.0)
    }
}
