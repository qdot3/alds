#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mint<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> Mint<MOD> {
    pub fn new(value: u64) -> Self {
        Self { value: value % MOD }
    }

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

    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            return a;
        }
        Self::gcd(b, a % b)
    }

    pub fn inv(self) -> Option<Self> {
        if Self::gcd(MOD, self.value) == 1 {
            fn inv_(a: i64, b: i64) -> i64 {
                if a == 1 {
                    return 1;
                } else {
                    return b + (1 - b * inv_(b % a, a)) / a;
                }
            }

            Some(Self::new(
                inv_(self.value as i64, MOD as i64).rem_euclid(MOD as i64) as u64,
            ))
        } else {
            None
        }
    }

    pub fn log(self, base: Self) -> Option<Self> {
        if self.value == 1 {
            return Some(Self::new(0));
        } else if base.value == 0 {
            return match self.value {
                0 => Some(Self::new(1)),
                1 => Some(Self::new(0)),
                _ => None,
            };
        } else if base.value == 1 {
            return None;
        }

        todo!("Implement baby-step-giant-step algorithm!")
    }
}

impl<const MOD: u64> std::fmt::Display for Mint<MOD> {
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

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

impl<const MOD: u64> Add for Mint<MOD> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let value = (self.value + rhs.value) % MOD;
        Self { value }
    }
}

forward_ref_mod_binop!(impl Add, add for Mint<u64>);

impl<const MOD: u64> AddAssign for Mint<MOD> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value) % MOD;
    }
}

forward_ref_mod_op_assign!(impl AddAssign, add_assign for Mint<u64>);

impl<const MOD: u64> Sub for Mint<MOD> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let value = (self.value + MOD - rhs.value) % MOD;
        Self { value }
    }
}

forward_ref_mod_binop!(impl Sub, sub for Mint<u64>);

impl<const MOD: u64> SubAssign for Mint<MOD> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value = (self.value + MOD - rhs.value) % MOD;
    }
}

forward_ref_mod_op_assign!(impl SubAssign, sub_assign for Mint<u64>);

impl<const MOD: u64> Mul for Mint<MOD> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let value = self.value * rhs.value % MOD;
        Self { value }
    }
}

forward_ref_mod_binop!(impl Mul, mul for Mint<u64>);

impl<const MOD: u64> MulAssign for Mint<MOD> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.value = self.value * rhs.value % MOD;
    }
}

forward_ref_mod_op_assign!(impl MulAssign, mul_assign for Mint<u64>);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inv_prime() {
        const MOD: u64 = 998_244_353;
        let m = Mint::<MOD>::new(2);
        let m_inv = m.inv().unwrap();
        assert_eq!(m * m_inv, Mint::new(1));

        let m_inv_inv = m_inv.inv().unwrap();
        assert_eq!(m_inv * m_inv_inv, Mint::new(1))
    }

    #[test]
    fn inv_composite() {
        const MOD: u64 = 2 * 3 * 7;
        let m = Mint::<MOD>::new(5);
        let m_inv = m.inv().unwrap();
        assert_eq!(m * m_inv, Mint::new(1));

        let m = m * Mint::new(1_000_000_000);
        assert_eq!(m.inv(), None)
    }
}
