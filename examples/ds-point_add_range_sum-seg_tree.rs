// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum
use proconio::{fastout, input};
use segment_tree::{Monoid, SegmentTree};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    // test `from`
    let mut seg_tree = SegmentTree::from(Vec::from_iter(a.into_iter().map(|a| M(a))));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: usize, x: u64, }

            let value = M(x + seg_tree.get(p).unwrap().0);
            seg_tree.set(p, value);
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", seg_tree.query(l..r).0)
        } else {
            unreachable!()
        }
    }
}

struct M(u64);

impl Monoid for M {
    fn identity() -> Self {
        Self(0)
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
