// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit

use fast_io::{FastInput, FastOutput};
use std::io::{stdin, stdout};

fn main() {
    let mut fast_in = FastInput::<{ 8 * 1024 * 2 }, _>::new(stdin().lock());
    let n = fast_in.parse_unwrap();

    let mut fast_out = FastOutput::new(stdout().lock());
    for _ in 0..n {
        let x: i128 = fast_in.parse_unwrap();
        let y: i128 = fast_in.parse_unwrap();

        fast_out.fast_writeln(x + y).unwrap();
    }
}
