//! A collection of union-find tree variants
//!
//!
mod normal;
mod partially_persistent;
mod potential;

pub use normal::{Groups, UnionFind};
pub use partially_persistent::PartiallyPersistentUnionFind;
pub use potential::{Group, UnionFindWithPotential};
