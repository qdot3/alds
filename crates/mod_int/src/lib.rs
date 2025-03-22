//! Collection of modular integer data structures and algorithms.
//!
//! # Selection Guide
//!
//! ## [`SMint`]
//!
//! * Compile-time fixed non-zero modulus
//!
//! ## [`MDMint`]
//!
//! * Runtime-specified *odd* modulus
//! * May be faster than [`BDMint`]
//!
//! ## [`BDMint`]
//!
//! * Runtime-specified any non-zero modulus
//!
//!
//! # Performance note
//!
//! | `+`, `-`, and `*` | `pow`        | `inv`           | `log`            | `sqrt`, `cbrt` and `nth_root` |
//! |-------------------|--------------|-----------------|------------------|-------------------------------|
//! | *O*(1)            | *O*(log *M*) |*O*(log *M*)[^1] | *O*( sqrt(*M*) ) | under construction            |
//!
//! * *M* is modulus
//!
//! [^1]: More precisely, same cost as Euclidean GCD algorithm.
//!
//!
//! # References
//!
//! ## Montgomery reduction
//!
//! * [wiki](https://en.wikipedia.org/wiki/Montgomery_modular_multiplication)
//!
//! ## Barret reduction
//!
//! * [wiki](https://en.wikipedia.org/wiki/Barrett_reduction)
mod barret_dynamic_modint;
mod inv_gcd;
mod macros;
mod montgomery_dynamic_modint;
mod static_modint;

pub use barret_dynamic_modint::{BDMint, Barret};
pub(self) use inv_gcd::inv_gcd;
pub use montgomery_dynamic_modint::{MDMint, Montgomery};
pub use static_modint::SMint;
