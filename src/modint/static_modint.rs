use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mint<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> Mint<MOD> {
    const MAX_MOD: u64 = 1 << u64::BITS / 2;

    pub const fn new(value: u64) -> Self {
        assert!(MOD <= Self::MAX_MOD);

        Self { value: value % MOD }
    }

    pub fn pow(mut self, mut exp: u64) -> Self {
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

    const fn gcd(mut a: u64, mut b: u64) -> Option<u64> {
        if a == 0 || b == 0 {
            return None;
        }

        while b > 0 {
            (a, b) = (b, a % b)
        }
        Some(a)
    }

    pub fn inv(self) -> Option<Self> {
        if Self::gcd(MOD, self.value) == Some(1) {
            //? overflow? non-recursive version?
            fn inv_(a: i64, b: i64) -> i64 {
                if a == 1 {
                    return 1;
                } else {
                    return b + (1 - b * inv_(b % a, a)) / a;
                }
            }

            return Some(Self::new(
                inv_(self.value as i64, MOD as i64).rem_euclid(MOD as i64) as u64,
            ));
        }

        None
    }

    /// define 0^0 = 1
    pub fn log(self, base: Self) -> Option<u64> {
        if MOD == 1 {
            return Some(0);
        }
        assert!(MOD >= 2);

        if base.value == 0 {
            return match self.value {
                0 => Some(1),
                1 => Some(0),
                _ => None,
            };
        } else if base.value == 1 {
            return if self.value == 1 { Some(0) } else { None };
        }

        if let Some(g) = Self::gcd(MOD, base.value) {
            if g == 1 {
                let p = MOD.isqrt() + 1;

                // base^(pi+q) = self (mod MOD)
                // base^q = self (base^-p)^i (mod MOD) for 0 <= i, q < p
                let inv_base = base.inv().expect("MOD and base is coprime");

                let mut lhs = FxHashMap::default();
                lhs.reserve(p as usize);
                // insert items in descending order for smaller q
                let mut pow_base = base.pow(p);
                for q in (0..p).rev() {
                    pow_base *= inv_base;
                    lhs.insert(pow_base, q);
                }

                let pow_inv_base = inv_base.pow(p);
                let mut rhs = self;
                for i in 0..p {
                    if let Some(q) = lhs.get(&rhs) {
                        return Some(p * i + q);
                    }
                    rhs *= pow_inv_base
                }

                return None;
            } else {
                let (mut small_mod, mut large_g) = (MOD / g, g);
                // O(log^2 MOD)
                while let Some(g) = Self::gcd(base.value, small_mod) {
                    if g == 1 {
                        break;
                    }

                    small_mod /= g;
                    large_g *= g
                }
                debug_assert_eq!(Self::gcd(base.value, small_mod), Some(1));

                if self.value % large_g != 0 {
                    return None;
                }

                // base^(k-1) = (self/g) (base/g)^-1 (mod small mod)
                todo!("impl dynamic modint")
            }
        }

        None
    }
}

impl<const MOD: u64> Display for Mint<MOD> {
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
