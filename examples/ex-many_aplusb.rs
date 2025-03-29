// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use fast_io::FromBytes;
use std::io::{stdout, BufWriter, Read, Write};

fn main() {
    let mut buf_r = String::with_capacity(40_000_000);
    let _ = std::io::stdin().lock().read_to_string(&mut buf_r);
    let mut num = buf_r
        .split_ascii_whitespace()
        .skip(1)
        .filter_map(|bytes| u128::from_bytes(bytes.as_bytes()).ok());

    let mut buf_w = BufWriter::new(stdout().lock());
    while let Some(x) = num.next() {
        let y = num.next().unwrap();

        buf_w.write((x + y).to_string().as_bytes()).unwrap();
        buf_w.write(b"\n").unwrap();
    }
}
