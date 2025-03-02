use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rustc_hash::FxHashMap;

use super::macros::{forward_ref_mint_binop, forward_ref_mint_op_assign, forward_ref_mint_unop};

/// Owner and factory for [`BDMint`] instances with the same modulus.
///
/// To use a different modulus, create a new [`Barret`] with the desired modulus.
pub struct Barret {
    modulus: u64,
    /// `(2^64 / modulus).ceil()`
    inv_modulus: u64,
}

impl Barret {
    /// Creates a new [`Barret`] with the given `modulus`.
    ///
    /// # Panics
    ///
    /// Panics if `modulus` is zero.
    pub const fn new(modulus: u32) -> Self {
        assert!(modulus != 0);

        let modulus = modulus as u64;
        let inv_modulus = (1_u64.wrapping_neg() / modulus).wrapping_add(1);

        Self {
            modulus,
            inv_modulus,
        }
    }

    /// Creates a new [`BDMint`] instance with the given `value` and the fixed modulus.
    pub const fn mint(&self, value: u64) -> BDMint {
        let value = if value < self.modulus * self.modulus {
            self.reduce(value)
        } else {
            value % self.modulus
        };

        BDMint {
            value,
            barret: self,
        }
    }

    /// Returns `x % modulus` for `0 <= x < modulus^2`.
    const fn reduce(&self, x: u64) -> u64 {
        if x < self.modulus {
            return x;
        }

        // 1. x = p * m + q, 0 <= p, q < 2^32  =>  x * im = p * (m * im) + q * im
        // 2. m * im = m * ceil(2^64 / m) = 2^64 + r, 0 <= r < m  =>  x * im = p * 2^64 + r * p + q * im
        // 3. r * p + q * im < m * m + m * im < 2^64 + m * (m + 1) < 2 * 2^64
        // 4. floor(x * im / 2^64) = p or p + 1
        assert!(x < self.modulus * self.modulus);
        //* use `carrying_mul` if stabilized*/
        let carry = ((x as u128 * self.inv_modulus as u128) >> u64::BITS) as u64;
        let x = x.wrapping_sub(carry.wrapping_mul(self.modulus));

        if x < self.modulus {
            x
        } else {
            x.wrapping_add(self.modulus)
        }
    }
}

/// Modular integer with a runtime-specified modulus based on
/// [Barret reduction](https://en.wikipedia.org/wiki/Barrett_reduction) algorithm.
///
/// Any binary operations are restricted to elements with the same owner
/// to ensure that they share the same modulus.
///
/// To use [`BDMint`] with a different modulus, create a new [`Barret`] instance as its owner.
#[derive(Clone, Copy)]
pub struct BDMint<'a> {
    value: u64,
    barret: &'a Barret,
}

impl BDMint<'_> {
    /// Returns the value.
    pub const fn value(&self) -> u64 {
        self.value
    }

    /// Returns the fixed modulus.
    pub const fn modulus(&self) -> u64 {
        self.barret.modulus
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub fn pow(mut self, mut exp: u32) -> Self {
        let mut res = self.barret.mint(1);
        while exp > 0 {
            if exp % 2 == 1 {
                res *= self;
            }
            self = self * self;
            exp /= 2;
        }

        res
    }

    /// Returns `(inv?(a) mod b, gcd(a, b))`, where `a < b` and `a * inv?(a) = g mod b`.
    pub(super) const fn inv_gcd(a: u64, b: u64) -> Option<(u64, u64)> {
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

    /// Returns the inverse of `self` if exists.
    pub const fn inv(mut self) -> Option<Self> {
        if let Some((inv, 1)) = Self::inv_gcd(self.value(), self.modulus()) {
            self.value = inv;
            return Some(self);
        }

        None
    }

    /// Returns the logarithm of `self` with respect to the given `base` if exists.
    ///
    /// # Note
    ///
    /// `0^0` is defined to be `1`.
    pub fn log(self, base: Self) -> Option<u32> {
        if self.modulus() == 1 {
            return Some(0);
        }
        match (base.value(), self.value()) {
            (0, 0) => return Some(1),
            (0, _) => return None,
            (_, 1) => return Some(0),
            (1, _) => return None,
            _ => (),
        }

        let d = self.modulus().ilog2() + 1;
        let mut pow_base = self.barret.mint(1);
        for k in 0..d {
            if pow_base == self {
                return Some(k);
            }
            pow_base *= base;
        }

        // gcd(base^d, modulus) = gcd(base^d % modulus, modulus)
        if let Some((_, g)) = Self::inv_gcd(pow_base.value(), self.modulus()) {
            if self.value() % g != 0 {
                return None;
            } else if g == self.modulus() {
                return Some(d);
            }

            let barret = Barret::new((self.modulus() / g) as u32);
            let x = barret.mint(base.value());
            let inv_x = x.inv().expect("x and new modulus will be coprime");
            let y = barret.mint(self.value()) * inv_x.pow(d);
            match (x.value(), y.value()) {
                (0, 0) => return Some(1 + d),
                (0, _) => return None,
                (_, 1) => return Some(d),
                (1, _) => return None,
                _ => (),
            }

            // solve x^k = y by baby-step-giant-step algorithm
            // x^(p * i + q) = y, 0 <= i, q < p  <=>  x^q = y * (x^-p)^i
            let p = (x.modulus() as u32).isqrt() + 1;

            let mut pow_x = barret.mint(1).pow(p);
            let mut lhs = FxHashMap::default();
            lhs.reserve(p as usize);
            // insert items in descending order for smaller *q*.
            for q in (0..p).rev() {
                pow_x *= inv_x;
                lhs.insert(pow_x, q);
            }

            let mut rhs = y;
            let pow_inv_x = inv_x.pow(p);
            for i in 0..p {
                if let Some(q) = lhs.get(&rhs) {
                    return Some(p * i + q + d);
                }
                rhs *= pow_inv_x
            }
        }

        None
    }
}

impl Debug for BDMint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BDMint")
            .field("value", &self.value())
            .field("modulus", &self.modulus())
            .finish()
    }
}

impl Display for BDMint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Hash for BDMint<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for BDMint<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for BDMint<'_> {}

impl PartialOrd for BDMint<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for BDMint<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

forward_ref_mint_binop!( impl<'a> Add, add for BDMint<'a> );
forward_ref_mint_binop!( impl<'a> Sub, sub for BDMint<'a> );
forward_ref_mint_binop!( impl<'a> Mul, mul for BDMint<'a> );

impl Add for BDMint<'_> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;

        self
    }
}

impl Sub for BDMint<'_> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;

        self
    }
}

impl Mul for BDMint<'_> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;

        self
    }
}

forward_ref_mint_op_assign!( impl<'a> AddAssign, add_assign for BDMint<'a> );
forward_ref_mint_op_assign!( impl<'a> SubAssign, sub_assign for BDMint<'a> );
forward_ref_mint_op_assign!( impl<'a> MulAssign, mul_assign for BDMint<'a> );

impl AddAssign for BDMint<'_> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if self.value > self.barret.modulus {
            self.value -= self.barret.modulus
        }
    }
}

impl SubAssign for BDMint<'_> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += self.barret.modulus - rhs.value
        } else {
            self.value -= rhs.value
        }
    }
}

impl MulAssign for BDMint<'_> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = self.barret.reduce(self.value * rhs.value);
    }
}

forward_ref_mint_unop!( impl<'a> Neg, neg for BDMint<'a> );

impl Neg for BDMint<'_> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = self.modulus() - self.value();
        }
        self
    }
}
