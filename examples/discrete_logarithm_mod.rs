// verification-helper: PROBLEM https://judge.yosupo.jp/problem/discrete_logarithm_mod

use alds::modint::Barret;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { t: usize, }

    for _ in 0..t {
        input! { x: u64, y: u64, m: u32, }

        let barret = Barret::new(m);
        let x = barret.mint(x);
        let y = barret.mint(y);

        if let Some(k) = y.log(x) {
            println!("{}", k)
        } else {
            println!("-1")
        }
    }
}
