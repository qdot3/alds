//! Collection of modular integer data structures and algorithms.
//!
//! # Selection Guide
//!
//! ## [`SMint`]
//!
//! * Compile-time fixed modulus
//! * Many operations: `add`, `sub`, `mul`, `neg`, `pow`, `inv` and `log`
//!
//! ## [`MDMint`]
//!
//! * Runtime-specified *odd* modulus
//! * Limited operations: `add`, `sub`, `mul`, `neg` and `pow`
//! * May be faster than [`BDMint`]
//!
//! ## [`BDMint`]
//!
//! * Runtime-specified modulus
//! * Same operations as [`SMint`]
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
mod macros;
mod montgomery_dynamic_modint;
mod static_modint;

pub use barret_dynamic_modint::{BDMint, Barret};
pub use montgomery_dynamic_modint::{MDMint, Montgomery};
pub use static_modint::SMint;
