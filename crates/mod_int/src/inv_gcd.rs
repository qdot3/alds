/// Returns `(inv?(a) mod b, gcd(a, b))`, where `a < b` and `a * inv?(a) = g mod b`.
pub(crate) const fn inv_gcd(a: u64, b: u64) -> Option<(u64, u64)> {
    if a == 0 || b == 0 {
        return None;
    }
    assert!(a < b);

    // a * x + b * y = g  <=>  g - a * x = 0 mod b
    let (mut g0, mut g1) = (b as i64, a as i64);
    let (mut x0, mut x1) = (0, 1);
    while g1 > 0 {
        let (div, rem) = (g0 / g1, g0 % g1);

        (g0, g1) = (g1, rem);
        (x0, x1) = (x1, x0 - x1 * div);
    }

    if x0.is_negative() {
        x0 += b as i64 / g0
    }

    Some((x0 as u64, g0 as u64))
}
