// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use fast_io::{FastInput, FastOutput, FromBytes};
use std::io::{stdin, stdout};

fn main() {
    let mut fast_in = FastInput::<{ 8 * 1024 }, _>::new(stdin().lock());
    let n = usize::from_bytes(fast_in.next_token().unwrap().as_slice()).unwrap();

    let mut fast_out = FastOutput::with_capacity(1 << 17, stdout().lock());
    for _ in 0..n {
        let x = u64::from_bytes(fast_in.next_token().unwrap().as_slice()).unwrap();
        let y = u64::from_bytes(fast_in.next_token().unwrap().as_slice()).unwrap();

        fast_out.fast_writeln(x + y).unwrap();
    }
}
