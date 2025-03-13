//! Collection of segment tree variances.
//!
//! # Performance note
//!
//! |                     | point query  | point apply  | range query  | range apply  |
//! |---------------------|--------------|--------------|--------------|--------------|
//! | [SegmentTree]       | *Θ*(1)       | *O*(log *N*) | N/A          | *O*(log *N*) |
//! | [DualSegmentTree]   | *O*(log *N*) | *O*(log *N*) | *O*(log *N*) | N/A          |
//! | [LazySegmentTree]   | *O*(log *N*) | *O*(log *N*) | *O*(log *N*) | *O*(log *N*) |
//! | [AssignSegmentTree] | *O*(log *N*) | *O*(log *N*) | *O*(log *N*) | *O*(log *N*) |
//!
//! * *N* is the number of elements.
mod assign;
mod dual;
mod dynamic;
mod lazy;
mod normal;
mod traits;

pub use assign::AssignSegmentTree;
pub use dual::DualSegmentTree;
pub use dynamic::DynamicSegmentTree;
pub use lazy::LazySegmentTree;
pub use normal::SegmentTree;
pub use traits::{Monoid, MonoidAct, MonoidAction};
