use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rustc_hash::FxHashMap;

use crate::{
    inv_gcd,
    macros::{forward_ref_mint_binop, forward_ref_mint_op_assign, forward_ref_mint_unop},
};

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
    /// # Example
    ///
    /// ```
    /// use mod_int::{Barret, BDMint};
    ///
    /// let barret = Barret::new(123_456);
    /// let x = barret.mint(123);
    /// let y = barret.mint(456);
    /// let z = barret.mint(123 * 456);
    ///
    /// assert_eq!(x * y, z);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `modulus` is zero.
    ///
    /// ```should_panic
    /// use mod_int::{Barret, BDMint};
    ///
    /// let modulus_must_be_more_than_zero = Barret::new(0);
    /// ```
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
    ///
    /// ```
    /// use mod_int::{Barret, BDMint};
    ///
    /// let barret = Barret::new(1);
    /// let any = barret.mint(123_456_789);
    /// let zero = barret.mint(0);
    ///
    /// assert_eq!(any, zero);
    /// ```
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
//  TODO
//  Any operations are restricted to elements with the same owner
//  to ensure that they share the same modulus.
///
/// Operations between elements with different moduli are currently allowed but meaningless.
/// It is possible to prohibit such operations by using unique constant parameters,
/// but manually setting them is cumbersome.
///
/// ```
/// use mod_int::{Barret, BDMint};
///
/// let modulus = 123_456;
/// let barret1 = Barret::new(123);
/// let v1 = barret1.mint(1);
///
/// let barret2 = Barret::new(456);
/// let v2 = barret2.mint(4);
///
/// let allowed_but_meaningless = v1 + v2;
/// ```
///
/// To use [`BDMint`] with a different modulus, create a new [`Barret`] instance.
///
/// ```
/// use mod_int::{Barret, BDMint};
///
/// let barret1 = Barret::new(123);
/// let v1 = barret1.mint(4);
///
/// let barret2 = Barret::new(567);
/// let v2 = barret2.mint(8);
///
/// let not_allowed = v1 * v2;
/// ```
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

    /// Returns the inverse of `self` if exists.
    pub const fn inv(mut self) -> Option<Self> {
        if let Some((inv, 1)) = inv_gcd(self.value(), self.modulus()) {
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
            (_, 1) => return Some(0), // 0^0 = 1
            (0, _) | (1, _) => return None,
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
        if let Some((_, g)) = inv_gcd(pow_base.value(), self.modulus()) {
            if self.value() % g != 0 {
                return None;
            } else if g == self.modulus() {
                return Some(d);
            }

            let barret = Barret::new((self.modulus() / g) as u32);
            let x = barret.mint(base.value());
            let inv_x = x.inv().expect("x and new modulus should be coprime");
            let y = barret.mint(self.value()) * inv_x.pow(d);
            match (base.value(), self.value()) {
                (0, 0) => return Some(d + 1),
                (_, 1) => return Some(d), // 0^0 = 1
                (0, _) | (1, _) => return None,
                _ => (),
            }

            // solve x^k = y by baby-step-giant-step algorithm
            // x^(p * i + j) = y, 0 <= i, j < p  <=>  x^j = y * (x^-p)^i
            // TODO: use isqrt()
            let p = (x.modulus() as f64).sqrt() as u32 + 1;

            let mut pow_x = x.pow(p);
            let mut lhs = FxHashMap::default();
            lhs.reserve(p as usize);
            // insert items in descending order for smaller *q*.
            for j in (0..p).rev() {
                pow_x *= inv_x;
                lhs.insert(pow_x, j);
            }

            let mut rhs = y;
            let pow_inv_x = inv_x.pow(p);
            for i in 0..p {
                if let Some(j) = lhs.get(&rhs) {
                    return Some(p * i + j + d);
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
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.barret.modulus.hash(state);
    }
}

impl PartialEq for BDMint<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for BDMint<'_> {}

impl PartialOrd for BDMint<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BDMint<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

forward_ref_mint_binop!( impl<'a> Add, add for BDMint<'a> );
forward_ref_mint_binop!( impl<'a> Sub, sub for BDMint<'a> );
forward_ref_mint_binop!( impl<'a> Mul, mul for BDMint<'a> );

impl Add for BDMint<'_> {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;

        self
    }
}

impl Sub for BDMint<'_> {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;

        self
    }
}

impl Mul for BDMint<'_> {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;

        self
    }
}

forward_ref_mint_op_assign!( impl<'a> AddAssign, add_assign for BDMint<'a> );
forward_ref_mint_op_assign!( impl<'a> SubAssign, sub_assign for BDMint<'a> );
forward_ref_mint_op_assign!( impl<'a> MulAssign, mul_assign for BDMint<'a> );

impl AddAssign for BDMint<'_> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if self.value > self.barret.modulus {
            self.value -= self.barret.modulus
        }
    }
}

impl SubAssign for BDMint<'_> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += self.barret.modulus - rhs.value
        } else {
            self.value -= rhs.value
        }
    }
}

impl MulAssign for BDMint<'_> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.value = self.barret.reduce(self.value * rhs.value);
    }
}

forward_ref_mint_unop!( impl<'a> Neg, neg for BDMint<'a> );

impl Neg for BDMint<'_> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = self.modulus() - self.value();
        }
        self
    }
}
