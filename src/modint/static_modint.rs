use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{BDMint, Barret};

/// Modular integer with a compile-time fixed modulus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SMint<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> SMint<MOD> {
    const MAX_MOD: u64 = 1 << u64::BITS / 2;

    pub const fn new(value: u64) -> Self {
        assert!(MOD <= Self::MAX_MOD);

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
        if let Some((inv, 1)) = BDMint::inv_gcd(self.value, MOD) {
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

impl<const MOD: u64> Display for SMint<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

macro_rules! forward_ref_mod_binop {
    (impl $imp:ident, $method:ident for $t:ident <$u:ident>) => {
        impl<const MOD: $u> $imp<&$t<MOD>> for $t<MOD> {
            type Output = $t<MOD>;

            #[inline]
            fn $method(self, other: &$t<MOD>) -> $t<MOD> {
                self.$method(*other)
            }
        }

        impl<const MOD: $u> $imp<$t<MOD>> for &$t<MOD> {
            type Output = $t<MOD>;

            #[inline]
            fn $method(self, other: $t<MOD>) -> $t<MOD> {
                (*self).$method(other)
            }
        }

        impl<const MOD: $u> $imp<&$t<MOD>> for &$t<MOD> {
            type Output = $t<MOD>;

            #[inline]
            fn $method(self, other: &$t<MOD>) -> $t<MOD> {
                (*self).$method(*other)
            }
        }
    };
}

macro_rules! forward_ref_mod_op_assign {
    (impl $imp:ident, $method:ident for $t:ident <$u:ident>) => {
        impl<const MOD: $u> $imp<&$t<MOD>> for $t<MOD> {
            #[inline]
            fn $method(&mut self, other: &$t<MOD>) {
                $imp::$method(self, *other);
            }
        }
    };
}

impl<const MOD: u64> Add for SMint<MOD> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let value = (self.value + rhs.value) % MOD;
        Self { value }
    }
}

forward_ref_mod_binop!(impl Add, add for SMint<u64>);

impl<const MOD: u64> AddAssign for SMint<MOD> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value) % MOD;
    }
}

forward_ref_mod_op_assign!(impl AddAssign, add_assign for SMint<u64>);

impl<const MOD: u64> Sub for SMint<MOD> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let value = (self.value + MOD - rhs.value) % MOD;
        Self { value }
    }
}

forward_ref_mod_binop!(impl Sub, sub for SMint<u64>);

impl<const MOD: u64> SubAssign for SMint<MOD> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value = (self.value + MOD - rhs.value) % MOD;
    }
}

forward_ref_mod_op_assign!(impl SubAssign, sub_assign for SMint<u64>);

impl<const MOD: u64> Mul for SMint<MOD> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let value = self.value * rhs.value % MOD;
        Self { value }
    }
}

forward_ref_mod_binop!(impl Mul, mul for SMint<u64>);

impl<const MOD: u64> MulAssign for SMint<MOD> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.value = self.value * rhs.value % MOD;
    }
}

forward_ref_mod_op_assign!(impl MulAssign, mul_assign for SMint<u64>);

macro_rules! forward_ref_mod_unop {
    ( impl<const $const_generics:ident: $const_ty:ty> $trait:ident, $method:ident for $t:ty ) => {
        impl<const $const_generics: $const_ty> $trait for &$t {
            type Output = $t;

            fn $method(self) -> Self::Output {
                (*self).$method()
            }
        }
    };
}

forward_ref_mod_unop! { impl<const MOD: u64> Neg, neg for SMint<MOD> }

impl<const MOD: u64> Neg for SMint<MOD> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = MOD - self.value;
        }
        self
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
