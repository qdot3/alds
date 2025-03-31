// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use std::io::{stdout, BufWriter};

use fast_io::{
    marker::{Usize, U64},
    FastWrite,
};
use proconio::input;

fn main() {
    input! { n: Usize, xy: [(U64, U64); n], }

    let mut buf_w = BufWriter::new(stdout().lock());
    for (x, y) in xy {
        buf_w.fast_writeln(x + y).unwrap();
    }
}
