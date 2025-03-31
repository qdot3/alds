// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use fast_io::prelude::{fast_stdin_locked, fast_stdout_locked};

fn main() {
    let mut fast_in = fast_stdin_locked();
    let n: usize = fast_in.parse_unwrap();

    let mut fast_out = fast_stdout_locked();
    for _ in 0..n {
        let x = fast_in.parse_unwrap::<u64>();
        let y = fast_in.parse_unwrap::<u64>();

        fast_out.fast_writeln(x + y).unwrap();
    }
}
