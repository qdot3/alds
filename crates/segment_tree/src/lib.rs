//! Collection of segment tree variances.
//!
//! # Performance note
//!
//! |                   | `get`        | `set`        | `apply`      | `eval`       |
//! |-------------------|--------------|--------------|--------------|--------------|
//! | [SegmentTree]     | *Θ*(1)       | *O*(log *N*) | N/A          | *O*(log *N*) |
//! | [DualSegmentTree] | *O*(log *N*) | *O*(log *N*) | *O*(log *N*) | N/A          |
//! | [LazySegmentTree] | *O*(log *N*) | *O*(log *N*) | *O*(log *N*) | *O*(log *N*) |
//!
//! * *N* is the number of elements.
mod dual;
mod lazy;
mod normal;
mod traits;
mod assign;

pub use dual::DualSegmentTree;
pub use lazy::LazySegmentTree;
pub use normal::SegmentTree;
pub use traits::{MonoidAction, Monoid, MonoidAct};
pub use assign::AssignSegmentTree;