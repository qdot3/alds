/// Dynamic ModInt based on Montgomery Multiplication.
///
/// ## Algorithm
pub struct Montgomery {
    n: u64,
    inv_n_mod_r: u64,
    r2_mod_n: u64,
}

impl Montgomery {
    /// ## Panics
    ///
    /// *n* should be an odd number below `RADIX` (= 2^32)
    pub const fn set_modulus(n: u64) -> Self {
        assert!(n % 2 == 1, "modulus should be odd");

        // r^2 mod n = r^2 - n mod n in u64 for r = 2^32.
        let r2_mod_n = n.wrapping_neg() % n;

        // if np * n = 1 (mod m), then
        //   (np * n + q * m)^2
        // = n(np^2 * n + 2np(1 - np * n)) + (q * m)^2
        // = n * np(2 - np * n)
        // = 1 (mod m^2).
        let inv_n_mod_r = {
            let mut inv_n_mod_r = n; // mod 4
            let mut i = 4;
            while i > 0 {
                inv_n_mod_r = inv_n_mod_r.wrapping_mul(2u64.wrapping_sub(inv_n_mod_r * n));
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

    fn reduction(&self, value: u64) -> MDMint {
        todo!()
    }
}

pub struct MDMint {
    value: u64,
}

#[cfg(test)]
mod tests {}
