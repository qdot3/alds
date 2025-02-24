//! A collection of union-find tree variants
//!
//!
mod normal;
mod partially_persistent;
mod weighted;

pub use normal::UnionFind;
pub use partially_persistent::PartiallyPersistentUnionFind;
pub use weighted::WeightedUnionFind;
