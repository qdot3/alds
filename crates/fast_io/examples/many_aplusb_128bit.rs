// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit

use std::io::{stdout, BufWriter};

use fast_io::{FastWrite, Usize, I128};
use proconio::input;

fn main() {
    input! { n: Usize, xy: [(I128, I128); n], }

    let mut buf_w = BufWriter::new(stdout().lock());
    for (x, y) in xy {
        buf_w.fast_writeln(x + y).unwrap();
    }
}
