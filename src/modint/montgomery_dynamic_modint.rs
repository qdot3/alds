use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::macros::{
    forward_ref_dyn_mint_binop, forward_ref_dyn_mint_op_assign, forward_ref_dyn_mint_unop,
};

/// Owner and factory for [`MDMint`] instances with the same modulus.
///
/// To use a different modulus, create a new [`Montgomery`] with the desired modulus.
pub struct Montgomery {
    modulus: u64,
    neg_inv_modulus_mod_radix: u64,
    radix2_mod_modulus: u64,
}

impl Montgomery {
    const RADIX: u64 = 1 << u64::BITS / 2; // 1^32

    /// Creates a new [`Barret`] with the given `modulus`.
    ///
    /// # Panics
    ///
    /// `modulus` should be an positive odd integer.
    pub const fn new(modulus: u32) -> Self {
        assert!(modulus % 2 == 1, "modulus should be an odd integer");
        let modulus = modulus as u64;

        // r^2 mod m = r^2 - m mod m in u64 for r = 2^32.
        let radix2_mod_modulus = modulus.wrapping_neg() % modulus;

        // 1. m * im = 1 mod r  =>  (m * im + q * r)^2 = 1, where q * r = 1 - m * im
        // 2. (m * im + a * r)^2 = (m * im)^2 + 2 * m * im * q * r + (q * r)^2
        //                       = (m * im)^2 + 2 * m * im * (1 - m * im) + (q * r)^2
        //                       = m * im * (2 - m * im) + (q * r)^2
        // 3. m * [im * (2 - m * im)] = 1 mod r^2
        let inv_modulus_mod_radix = {
            let mut inv_modulus_mod_radix = modulus; // mod 4
            let mut i = 4; // 2^2 -> 2^4 -> 2^8 -> 2^16 -> 2^32
            while i > 0 {
                inv_modulus_mod_radix = inv_modulus_mod_radix
                    .wrapping_mul(2u64.wrapping_sub(inv_modulus_mod_radix.wrapping_mul(modulus)));
                i -= 1;
            }
            inv_modulus_mod_radix % Self::RADIX
        };
        assert!(modulus.wrapping_mul(inv_modulus_mod_radix) % Self::RADIX == 1);

        Self {
            modulus,
            neg_inv_modulus_mod_radix: Self::RADIX - inv_modulus_mod_radix, // im > 0
            radix2_mod_modulus,
        }
    }

    /// Creates a new [`MDMint`] instance with the given `value` and the fixed modulus.
    pub const fn mint(&self, value: u32) -> MDMint {
        // `value < RADIX = 2^32`
        let value = self.reduce(value as u64 * self.radix2_mod_modulus);

        MDMint {
            value,
            montgomery: &self,
        }
    }

    /// Returns `x * inv(RADIX) mod modulus` if `x < modulus * RADIX`
    const fn reduce(&self, x: u64) -> u64 {
        assert!(x < self.modulus * Self::RADIX);

        // s * m = x * m * im = s * (r * ir - 1) = -x mod r => x + s * m = 0 mod r
        let s = (x % Self::RADIX) * self.neg_inv_modulus_mod_radix % Self::RADIX;
        // s * m + (r - 1) <= (r - 1)^2 + (r - 1) = r * (r - 1) < r^2 => non-overflowing
        let t = x / Self::RADIX + (x % Self::RADIX + s * self.modulus) / Self::RADIX;

        // 0 <= x + s * m < m * r + r * m < 2 * m * r => t < 2 * m
        if t < self.modulus {
            t
        } else {
            t - self.modulus
        }
    }
}

/// Modular integer with a runtime-specified modulus based on
/// [Montgomery reduction](https://en.wikipedia.org/wiki/Montgomery_modular_multiplication) algorithm.
///
/// Any binary operations are restricted to elements with the same owner
/// to ensure that they share the same modulus.
///
/// To use [`MDMint`] with a different modulus, create a new [`Montgomery`] instance as its owner.
#[derive(Clone, Copy)]
pub struct MDMint<'a> {
    /// x * RADIX mod modulus
    value: u64,
    montgomery: &'a Montgomery,
}

impl<'a> MDMint<'a> {
    /// Returns the value.
    pub const fn value(&self) -> u64 {
        self.montgomery.reduce(self.value)
    }

    /// Returns the modulus.
    pub const fn modulus(&self) -> u64 {
        self.montgomery.modulus
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub fn pow(mut self, mut exp: u32) -> Self {
        let mut res = self.montgomery.mint(1);
        while exp > 0 {
            if exp % 2 == 1 {
                res *= self
            }
            self *= self;
            exp /= 2;
        }

        res
    }
}

impl<'a> Debug for MDMint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MDMint")
            .field("value", &self.value())
            .field("modulus", &self.modulus())
            .finish()
    }
}

impl<'a> Display for MDMint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl<'a> Hash for MDMint<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<'a> PartialEq for MDMint<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'a> Eq for MDMint<'a> {}

impl<'a> PartialOrd for MDMint<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<'a> Ord for MDMint<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

forward_ref_dyn_mint_binop!(impl<'a> Add, add for MDMint<'a>);
forward_ref_dyn_mint_binop!(impl<'a> Sub, sub for MDMint<'a>);
forward_ref_dyn_mint_binop!(impl<'a> Mul, mul for MDMint<'a>);

impl<'a> Add for MDMint<'a> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;

        self
    }
}

impl<'a> Sub for MDMint<'a> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;

        self
    }
}

impl<'a> Mul for MDMint<'a> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;

        self
    }
}

forward_ref_dyn_mint_op_assign!(impl<'a> AddAssign, add_assign for MDMint<'a>);
forward_ref_dyn_mint_op_assign!(impl<'a> SubAssign, sub_assign for MDMint<'a>);
forward_ref_dyn_mint_op_assign!(impl<'a> MulAssign, mul_assign for MDMint<'a>);

impl<'a> AddAssign for MDMint<'a> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

impl<'a> SubAssign for MDMint<'a> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value = self.value + self.montgomery.modulus - rhs.value
        } else {
            self.value -= rhs.value
        }
    }
}

impl<'a> MulAssign for MDMint<'a> {
    fn mul_assign(&mut self, rhs: Self) {
        // v1 * v2 < m * m < m * r
        self.value = self.montgomery.reduce(self.value * rhs.value)
    }
}

forward_ref_dyn_mint_unop!(impl<'a> Neg, neg for MDMint<'a>);

impl<'a> Neg for MDMint<'a> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = self.montgomery.modulus - self.value;
        }

        self
    }
}
