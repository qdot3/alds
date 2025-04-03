use crate::forward_ref_binop;

pub trait GCD<Other = Self> {
    type Output;

    /// Returns GCD (Greatest Common Divisor) of the pair.
    ///
    /// # Example
    ///
    /// ```
    /// use math_traits::GCD;
    ///
    /// let non_zero1: i32 = 2 * 3 * 5;
    /// let non_zero2: i32 = 3 * 5 * 7;
    ///
    /// assert!(non_zero1.gcd(0).is_none());
    /// assert_eq!(non_zero1.gcd(&non_zero2), Some(3 * 5));
    /// ```
    #[must_use]
    fn gcd(self, other: Other) -> Self::Output;
}

pub trait LCM<Other = Self> {
    type Output;

    /// Returns LCM (Least Common Multiplier) of the pair.
    ///
    /// # Example
    ///
    /// ```
    /// use math_traits::LCM;
    ///
    /// let non_zero1: i32 = 2 * 3 * 5;
    /// let non_zero2: i32 = 3 * 5 * 7;
    ///
    /// assert!(non_zero1.lcm(0).is_none());
    /// assert_eq!(non_zero1.lcm(&non_zero2), Some(2 * 3 * 5 * 7));
    /// ```
    #[must_use]
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

        forward_ref_binop! { impl GCD, gcd for $t }

        impl LCM for $t {
            type Output = Option<$t>;

            fn lcm(self, other: $t) -> Self::Output {
                match self.gcd(other) {
                    Some(gcd) => Some(self / gcd * other),
                    None => None,
                }
            }
        }

        forward_ref_binop! { impl LCM, lcm for $t }
    )*};
}

gcd_lcm_impl! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
