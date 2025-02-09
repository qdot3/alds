/// Dynamic ModInt based on Montgomery Multiplication.
///
/// ## Algorithm
pub struct Montgomery {
    n: u64,
    inv_n_mod_r: u64,
    r2_mod_n: u64,
}

impl Montgomery {
    /// # Panics
    ///
    /// modulus *n* should be an odd integer
    pub const fn new(n: u64) -> Self {
        assert!(n % 2 == 1, "modulus should be odd");

        // r^2 mod n = r^2 - n mod n in u128 for r = 2^64.
        let r2_mod_n = ((n as u128).wrapping_neg() % n as u128) as u64;

        // if np * n = 1 (mod m), then
        //   (np * n + q * m)^2
        // = n(np^2 * n + 2np(1 - np * n)) + (q * m)^2
        // = n * np(2 - np * n)
        // = 1 (mod m^2).
        let inv_n_mod_r = {
            let mut inv_n_mod_r = n; // mod 4
            let mut i = 5; // 2^2 -> 2^4 -> 2^8 -> 2^16 -> 2^32 -> 2^64
            while i > 0 {
                inv_n_mod_r =
                    inv_n_mod_r.wrapping_mul(2u64.wrapping_sub(inv_n_mod_r.wrapping_mul(n)));
                i -= 1;
            }
            inv_n_mod_r
        };
        assert!(n.wrapping_mul(inv_n_mod_r) == 1);

        Self {
            n,
            inv_n_mod_r,
            r2_mod_n,
        }
    }

    pub fn new_mint(&self, value: u64) -> MDMint {
        const RADIX: u128 = 1 << 64;
        let t = value as u128 * self.r2_mod_n as u128; // nr
        let (res, overflowed) =
            t.overflowing_sub(t % RADIX * self.inv_n_mod_r as u128 % RADIX * self.n as u128);
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MDMint {
    value: u64,
    modulus: u64,
}

#[cfg(test)]
mod tests {
    use super::Montgomery;

    #[test]
    fn test_montgomery_params_sparse() {
        let even = (1 << 48) + (1 << 24) + (1 << 12) + (1 << 6) + (1 << 3);
        for modulo in (1..u64::MAX).step_by(even) {
            Montgomery::new(modulo);
        }
    }
}
