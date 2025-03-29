// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use fast_io::FromBytes;
use std::io::{stdout, BufWriter, Read, Write};
use atoi::atoi;

fn main() {
    let mut buf_r = Vec::new();
    let _ = std::io::stdin().lock().read_to_end(&mut buf_r);
    let mut num = buf_r
        .split(|b| b.is_ascii_whitespace())
        .filter(|bytes| !bytes.is_empty())
        .skip(1)
        .map(|bytes| atoi::<u64>(bytes).unwrap());
        // .map(|bytes| u64::from_bytes(bytes).unwrap());

    let mut buf_w = BufWriter::new(stdout().lock());
    while let Some(x) = num.next() {
        let y = num.next().unwrap();

        writeln!(&mut buf_w, "{}", x + y).unwrap();
    }
}
