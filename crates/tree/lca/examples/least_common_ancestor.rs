// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use lca::LCA;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, q: usize, p: [usize; n - 1], uv: [(usize, usize); q], }

    let lca = LCA::from_edges(
        Vec::from_iter(p.into_iter().enumerate().map(|(i, p)| (i + 1, p))),
        0,
    );
    println!("{:?}", lca);
    for (u, v) in uv {
        println!("{}", lca.lca(u, v).0)
    }
}
