// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit

use std::io::{stdin, stdout, BufWriter};

use fast_io::{FastInput, FastWrite, FromBytes};

fn main() {
    let mut buf_r = FastInput::<{ 8 * 1024 }, _>::new(stdin().lock());
    let n = usize::from_bytes(buf_r.next_token().unwrap().as_slice()).unwrap();
    let mut buf_w = BufWriter::new(stdout().lock());
    for _ in 0..n {
        let x = u64::from_bytes(buf_r.next_token().unwrap().as_slice()).unwrap();
        let y = u64::from_bytes(buf_r.next_token().unwrap().as_slice()).unwrap();

        buf_w.fast_writeln(x + y).unwrap();
    }
}
