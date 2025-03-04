// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use proconio::{fastout, input};
use union_find::UnionFind;

#[fastout]
fn main() {
    input! { n: usize, q: usize, query: [(u8, usize, usize); q], }

    let mut uf = UnionFind::new(n);
    for (flag, i, j) in query {
        if flag == 0 {
            let _ = uf.unite(i, j);
        } else if flag == 1 {
            println!("{}", if uf.same(i, j) { 1 } else { 0 })
        }
    }
}
