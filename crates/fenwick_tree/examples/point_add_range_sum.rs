// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use fast_io::prelude::{fast_stdin_locked, fast_stdout_locked};
use fenwick_tree::FenwickTree;
use math_traits::{marker::Commutative, Group};

fn main() {
    let mut fast_in = fast_stdin_locked();
    let n = fast_in.next_token().unwrap();
    let q = fast_in.next_token().unwrap();

    let mut fast_out = fast_stdout_locked();
    let mut ft =
        FenwickTree::from_iter(std::iter::repeat_with(|| A(fast_in.next_token().unwrap())).take(n));
    for _ in 0..q {
        let flag: u8 = fast_in.next_token().unwrap();
        match flag {
            0 => {
                let p = fast_in.next_token().unwrap();
                let x = fast_in.next_token().unwrap();

                ft.point_update(p, A(x))
            }
            1 => {
                let l: usize = fast_in.next_token().unwrap();
                let r = fast_in.next_token().unwrap();

                fast_out.fast_writeln(&ft.range_query(l..r).0).unwrap();
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
