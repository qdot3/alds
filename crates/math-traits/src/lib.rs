mod gcd_lcm;
mod macros;

pub use gcd_lcm::{GCD, LCM};
pub(crate) use macros::forward_ref_binop;

pub trait Monoid {
    fn identity() -> Self;
    fn bin_op(&self, rhs: &Self) -> Self;
}

pub trait Group {
    fn identity() -> Self;
    fn bin_op(&self, rhs: &Self) -> Self;
    fn inverse(&self) -> Self;
}

pub mod marker {
    /// A marker trait for idempotent binary operations.
    pub trait Idempotent {}

    /// A marker trait for commutative binary operations.
    pub trait Commutative {}
}
