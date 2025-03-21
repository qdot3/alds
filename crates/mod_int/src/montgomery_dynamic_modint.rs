use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    inv_gcd,
    macros::{forward_ref_mint_binop, forward_ref_mint_op_assign, forward_ref_mint_unop},
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
    const RADIX: u64 = 1 << (u64::BITS / 2); // 1^32

    /// Creates a new [`Montgomery`] with the given `modulus`.
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
        let r_value = self.reduce(value as u64 * self.radix2_mod_modulus);

        MDMint {
            r_value,
            montgomery: self,
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
// TODO
// Any binary operations are restricted to elements with the same owner
// to ensure that they share the same modulus.
///
/// Operations between elements with different moduli are currently allowed but meaningless.
/// It is possible to prohibit such operations by using unique constant parameters,
/// but manually setting them is cumbersome.
///
/// To use [`MDMint`] with a different modulus, create a new [`Montgomery`] instance.
#[derive(Clone, Copy)]
pub struct MDMint<'a> {
    /// x * RADIX mod modulus
    r_value: u64,
    montgomery: &'a Montgomery,
}

impl MDMint<'_> {
    /// Returns the value.
    pub const fn value(&self) -> u64 {
        self.montgomery.reduce(self.r_value)
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

    /// Returns the inverse of `self` if exists.
    pub const fn inv(mut self) -> Option<Self> {
        if let Some((inv, 1)) = inv_gcd(self.value(), self.modulus()) {
            let mont = self.montgomery;

            self.r_value = mont.reduce(mont.radix2_mod_modulus * inv);
            return Some(self);
        }

        None
    }
}

impl Debug for MDMint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MDMint")
            .field("value", &self.value())
            .field("modulus", &self.modulus())
            .finish()
    }
}

impl Display for MDMint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Hash for MDMint<'_> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r_value.hash(state);
        self.montgomery.modulus.hash(state);
    }
}

impl PartialEq for MDMint<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.r_value == other.r_value
    }
}

impl Eq for MDMint<'_> {}

impl PartialOrd for MDMint<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MDMint<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

forward_ref_mint_binop!( impl<'a> Add, add for MDMint<'a> );
forward_ref_mint_binop!( impl<'a> Sub, sub for MDMint<'a> );
forward_ref_mint_binop!( impl<'a> Mul, mul for MDMint<'a> );

impl Add for MDMint<'_> {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;

        self
    }
}

impl Sub for MDMint<'_> {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;

        self
    }
}

impl Mul for MDMint<'_> {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;

        self
    }
}

forward_ref_mint_op_assign!( impl<'a> AddAssign, add_assign for MDMint<'a> );
forward_ref_mint_op_assign!( impl<'a> SubAssign, sub_assign for MDMint<'a> );
forward_ref_mint_op_assign!( impl<'a> MulAssign, mul_assign for MDMint<'a> );

impl AddAssign for MDMint<'_> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.r_value += rhs.r_value;
        if self.r_value > self.modulus() {
            self.r_value -= self.modulus()
        }
    }
}

impl SubAssign for MDMint<'_> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.r_value = self.r_value.wrapping_sub(rhs.r_value);
        if self.r_value >= self.modulus() {
            self.r_value = self.r_value.wrapping_add(self.modulus());
        }
    }
}

impl MulAssign for MDMint<'_> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        // v1 * v2 < m * m < m * r
        self.r_value = self.montgomery.reduce(self.r_value * rhs.r_value)
    }
}

forward_ref_mint_unop!( impl<'a> Neg, neg for MDMint<'a> );

impl Neg for MDMint<'_> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self::Output {
        if self.r_value > 0 {
            self.r_value = self.montgomery.modulus - self.r_value;
        }

        self
    }
}
