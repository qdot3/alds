// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use fast_io::{FastInput, FastOutput};
use std::io::{stdin, stdout};

fn main() {
    let mut fast_in = FastInput::new(stdin().lock());
    let n: usize = fast_in.next_token().unwrap();

    let mut fast_out = FastOutput::new(stdout().lock());
    for _ in 0..n {
        let x: u64 = fast_in.next_token().unwrap();
        let y: u64 = fast_in.next_token().unwrap();

        fast_out.fast_writeln(&(x + y)).unwrap();
    }
}
