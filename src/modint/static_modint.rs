use std::{
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{
    inv_gcd::inv_gcd,
    macros::{forward_ref_mint_binop, forward_ref_mint_op_assign, forward_ref_mint_unop},
    Barret,
};

/// Modular integer with a compile-time fixed modulus.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SMint<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> SMint<MOD> {
    const MAX_MOD: u64 = 1 << (u64::BITS / 2);

    pub const fn new(value: u64) -> Self {
        assert!(
            MOD <= Self::MAX_MOD,
            "modulus should be less than or equal to 2^32"
        );

        Self { value: value % MOD }
    }

    /// Returns the value.
    pub const fn value(&self) -> u64 {
        self.value
    }

    /// Returns the modulus.
    pub const fn modulus(&self) -> u64 {
        MOD
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub fn pow(mut self, mut exp: u32) -> Self {
        let mut res = Self::new(1);
        while exp > 0 {
            if exp & 1 == 1 {
                res *= self
            }
            self *= self;
            exp >>= 1
        }

        res
    }

    /// Returns the inverse of `self` if exists.
    pub const fn inv(mut self) -> Option<Self> {
        if let Some((inv, 1)) = inv_gcd(self.value, MOD) {
            self.value = inv;
            return Some(self);
        }

        None
    }

    /// Returns the logarithm of `self` with respect to the given `base` if exists.
    ///
    /// # Note
    ///
    /// * `0^0` is defined to be `1`.
    /// * wrapper of [`BDMint::log`]
    pub fn log(self, base: Self) -> Option<u32> {
        let barret = Barret::new(MOD as u32);
        let x = barret.mint(base.value);
        let y = barret.mint(self.value);
        y.log(x)
    }
}

impl<const MOD: u64> Debug for SMint<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SMint")
            .field("value", &self.value)
            .field("modulus", &MOD)
            .finish()
    }
}

impl<const MOD: u64> Display for SMint<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl<const MOD: u64> Sum for SMint<MOD> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |acc, x| acc + x)
    }
}

impl<const MOD: u64> Product for SMint<MOD> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |acc, x| acc * x)
    }
}

forward_ref_mint_binop!( impl<const MOD: u64> Add, add for SMint<MOD> );
forward_ref_mint_binop!( impl<const MOD: u64> Sub, sub for SMint<MOD> );
forward_ref_mint_binop!( impl<const MOD: u64> Mul, mul for SMint<MOD> );

impl<const MOD: u64> Add for SMint<MOD> {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;

        self
    }
}

impl<const MOD: u64> Sub for SMint<MOD> {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;

        self
    }
}

impl<const MOD: u64> Mul for SMint<MOD> {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;

        self
    }
}

forward_ref_mint_op_assign!( impl<const MOD: u64> AddAssign, add_assign for SMint<MOD> );
forward_ref_mint_op_assign!( impl<const MOD: u64> SubAssign, sub_assign for SMint<MOD> );
forward_ref_mint_op_assign!( impl<const MOD: u64> MulAssign, mul_assign for SMint<MOD> );

impl<const MOD: u64> AddAssign for SMint<MOD> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value) % MOD;
    }
}

impl<const MOD: u64> SubAssign for SMint<MOD> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value = (self.value + MOD - rhs.value) % MOD;
    }
}

impl<const MOD: u64> MulAssign for SMint<MOD> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.value = self.value * rhs.value % MOD;
    }
}

forward_ref_mint_unop!( impl<const MOD: u64> Neg, neg for SMint<MOD> );

impl<const MOD: u64> Neg for SMint<MOD> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(0) - self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inv_prime() {
        const MOD: u64 = 998_244_353;
        let m = SMint::<MOD>::new(2);
        let m_inv = m.inv().unwrap();
        assert_eq!(m * m_inv, SMint::new(1));

        let m_inv_inv = m_inv.inv().unwrap();
        assert_eq!(m_inv * m_inv_inv, SMint::new(1))
    }

    #[test]
    fn inv_composite() {
        const MOD: u64 = 2 * 3 * 7;
        let m = SMint::<MOD>::new(5);
        let m_inv = m.inv().unwrap();
        assert_eq!(m * m_inv, SMint::new(1));

        let m = m * SMint::new(1_000_000_000);
        assert_eq!(m.inv(), None)
    }
}
