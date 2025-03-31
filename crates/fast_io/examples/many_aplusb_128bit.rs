// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit

use fast_io::{FastInput, FastOutput, FromBytes};
use std::io::{stdin, stdout};

fn main() {
    let mut fast_in = FastInput::<{ 8 * 1024 }, _>::new(stdin().lock());
    let n = usize::from_bytes(fast_in.next_token().unwrap().as_slice()).unwrap();

    let mut fast_out = FastOutput::new(stdout().lock());
    for _ in 0..n {
        let x = i128::from_bytes(fast_in.next_token().unwrap().as_slice()).unwrap();
        let y = i128::from_bytes(fast_in.next_token().unwrap().as_slice()).unwrap();

        fast_out.fast_writeln(x + y).unwrap();
    }
}
