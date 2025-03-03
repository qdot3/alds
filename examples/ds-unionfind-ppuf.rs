// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use alds::union_find::PartiallyPersistentUnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, q: usize, query: [(u8, usize, usize); q], }

    const TIME_INFINITY: u32 = !0;
    let mut ppuf = PartiallyPersistentUnionFind::new(n);
    for (flag, i, j) in query {
        if flag == 0 {
            let _ = ppuf.unite(i, j);
        } else if flag == 1 {
            println!("{}", if ppuf.same(i, j, TIME_INFINITY) { 1 } else { 0 })
        }
    }
}
