// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use fast_io::{FastInput, FastWrite};
use std::io::{stdout, BufWriter};

fn main() {
    let mut fast_in = FastInput::new();
    let n = fast_in.next_token::<usize>().unwrap();
    let mut buf_w = BufWriter::new(stdout().lock());
    for _ in 0..n {
        let x = fast_in.next_token::<u64>().unwrap();
        let y = fast_in.next_token::<u64>().unwrap();

        buf_w.fast_writeln(x + y).unwrap();
    }
}
