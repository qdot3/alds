// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use fast_io::prelude::fast_stdin_locked;
use fenwick_tree::FenwickTree;
use math_traits::{marker::Commutative, Group};

fn main() {
    let mut fast_in = fast_stdin_locked();
    let n = fast_in.parse_unwrap();
    let q = fast_in.parse_unwrap();

    let mut ft =
        FenwickTree::from_iter(std::iter::repeat_with(|| A(fast_in.parse_unwrap::<i64>())).take(n));
    for _ in 0..q {
        let flag: u8 = fast_in.parse_unwrap();
        match flag {
            0 => {
                let p = fast_in.parse_unwrap();
                let x = fast_in.parse_unwrap();

                ft.point_update(p, A(x))
            }
            1 => {
                let l: usize = fast_in.parse_unwrap();
                let r = fast_in.parse_unwrap();

                println!("{}", ft.range_query(l..r).0)
            }
            _ => unreachable!(),
        }
    }
}

struct A(i64);
impl Commutative for A {}
impl Group for A {
    fn identity() -> Self {
        Self(0)
    }

    fn bin_op(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }

    fn inverse(&self) -> Self {
        Self(-self.0)
    }
}
