// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit

use fast_io::{prelude::DEFAULT_BUF_SIZE, FastInput, FastOutput};
use std::io::{stdin, stdout};

fn main() {
    let mut fast_in = FastInput::new(stdin().lock());
    let n = fast_in.next_token().unwrap();

    let mut fast_out = FastOutput::with_capacity(DEFAULT_BUF_SIZE * 4, stdout().lock());
    for _ in 0..n {
        let x: i128 = fast_in.next_token().unwrap();
        let y: i128 = fast_in.next_token().unwrap();

        fast_out.fast_writeln(&(x + y)).unwrap();
    }
}
