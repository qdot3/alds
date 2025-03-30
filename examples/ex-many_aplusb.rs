// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use fast_io::{FastWrite, FromBytes};
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter};

fn main() {
    let mut buf_r = BufReader::with_capacity(40_000_000, stdin().lock());
    let mut num = buf_r
        .fill_buf()
        .unwrap()
        .split(|b| b.is_ascii_whitespace())
        .filter(|bytes| !bytes.is_empty())
        .skip(1)
        .map(|bytes| u64::from_bytes(bytes).unwrap());

    let mut buf_w = BufWriter::new(stdout().lock());
    while let Some(x) = num.next() {
        let y = num.next().unwrap();

        buf_w.fast_writeln(x + y).unwrap();
        // writeln!(&mut buf_w, "{}", x + y).unwrap();
    }
}
