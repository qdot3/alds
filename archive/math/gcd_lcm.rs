//! Calculate GCD and LCM based on Euclidean Algorithm (see [wiki](https://en.wikipedia.org/wiki/Euclidean_algorithm)).
//!
//! ## Example
//!
//! ```
//! use alds::math::{GCD, LCM};
//!
//! assert!(0i32.gcd(1).is_none());
//! assert!(0i32.lcm(1).is_none());
//!
//! let a = 2 * 3 * 5;
//! let b = 3 * 5 * 7;
//! assert_eq!(a.gcd(b), Some(3 * 5));
//! assert_eq!(a.lcm(b), Some(2 * 3 * 5 * 7));
//! ```
use forward_ref::forward_ref_binop;

/// Calculates greatest common divisor (GCD).
///
/// # Example
///
/// ```
/// use alds::math::GCD;
///
/// let non_zero = 3 * 5 * 7;
/// assert_eq!(non_zero.gcd(0), None);
///
/// let non_zero2 = 5 * 7 * 11;
/// assert_eq!(non_zero.gcd(non_zero2), Some(5 * 7));
/// ```
pub trait GCD<Other = Self> {
    type Output;

    fn gcd(self, other: Other) -> Self::Output;
}

/// Calculates least common multiplier (LCM).
///
/// # Example
///
/// ```
/// use alds::math::LCM;
///
/// let non_zero = 2 * 3 * 5;
/// assert_eq!(non_zero.lcm(0), None);
///
/// let non_zero2 = 3 * 5 * 7;
/// assert_eq!(non_zero.lcm(-non_zero2), Some(2 * 3 * 5 * 7));
/// ```
pub trait LCM<Other = Self> {
    type Output;

    fn lcm(self, other: Other) -> Self::Output;
}

macro_rules! gcd_lcm_impl {
    ($( $t:ty )*) => {$(
        impl GCD for $t {
            type Output = Option<$t>;

            fn gcd(self, other: $t) -> Self::Output {
                if self == 0 || other == 0 {
                    return None
                }

                let (mut a, mut b) = (self, other);
                while b != 0 {
                    (a, b) = (b, a % b)
                }

                Some(a)
            }
        }

        forward_ref_binop! { impl GCD, gcd for $t, $t }

        impl LCM for $t {
            type Output = Option<$t>;

            fn lcm(self, other: $t) -> Self::Output {
                match self.gcd(other) {
                    Some(gcd) => Some(self / gcd * other),
                    None => None,
                }
            }
        }

        forward_ref_binop! { impl LCM, lcm for $t, $t }
    )*};
}

gcd_lcm_impl! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert!(0u32.gcd(0).is_none());
        assert!(0u32.gcd(&1).is_none());
        assert!((&1u32).gcd(0).is_none());
        assert!((&1u32).gcd(&0).is_none());

        assert_eq!(GCD::gcd(2 * 3 * 5, 3 * 5 * 7), Some(3 * 5));
        assert_eq!(GCD::gcd(998_244_353, 1_000_000_000 + 7), Some(1));
    }

    #[test]
    fn test_lcm() {
        assert!(0i32.lcm(0).is_none());
        assert!(0i32.lcm(&1).is_none());
        assert!((&1i32).lcm(0).is_none());
        assert!((&0i32).lcm(&0).is_none());

        assert_eq!(LCM::lcm(2 * 3 * 5, 3 * 5 * 7), Some(2 * 3 * 5 * 7));
    }
}
