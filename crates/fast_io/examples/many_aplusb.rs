// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use fast_io::{FastInput, FastOutput};
use std::io::{stdin, stdout};

fn main() {
    let mut fast_in = FastInput::<{ 8 * 1024 }, _>::new(stdin().lock());
    let n: usize = fast_in.parse_unwrap();

    let mut fast_out = FastOutput::with_capacity(1 << 17, stdout().lock());
    for _ in 0..n {
        let x = fast_in.parse_unwrap::<u64>();
        let y = fast_in.parse_unwrap::<u64>();

        fast_out.fast_writeln(x + y).unwrap();
    }
}
