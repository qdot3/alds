// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use fast_io::FromBytes;
use std::{fmt::Write, io::Read};

fn main() {
    let mut buf_r = String::with_capacity(40_000_000);
    let _ = std::io::stdin().lock().read_to_string(&mut buf_r);
    let mut num = buf_r.split_ascii_whitespace().skip(1);

    let mut buf_w = String::with_capacity(20_000_000);
    while let Some(x) = num.next() {
        let y = num.next().unwrap();

        writeln!(
            &mut buf_w,
            "{}",
            // x.parse::<u64>().unwrap() + y.parse::<u64>().unwrap()
            u64::from_bytes(x.as_bytes()).unwrap() + u64::from_bytes(y.as_bytes()).unwrap()
        )
        .unwrap();
    }

    print!("{}", buf_w)
}
