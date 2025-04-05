// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_primes

use fast_io::prelude::{fast_stdin_locked, fast_stdout_locked};
use sieve_of_eratosthenes::SieveOfEratosthenes;

fn main() {
    let [n, a, b] = {
        let mut fast_in = fast_stdin_locked();
        std::array::from_fn(|_| fast_in.next_token().unwrap())
    };

    let mut pi = 0;
    let mut primes = Vec::with_capacity(n / a);
    for (i, p) in SieveOfEratosthenes::new(n).into_primes().enumerate() {
        pi += 1;

        if i >= b && (i - b) % a == 0 {
            primes.push(p)
        }
    }

    let mut fast_out = fast_stdout_locked();
    fast_out.fast_writeln_all([pi, primes.len()], " ").unwrap();
    fast_out.fast_writeln_all(primes, " ").unwrap();
}
