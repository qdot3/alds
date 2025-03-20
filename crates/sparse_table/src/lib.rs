mod disjoint;
mod normal;
mod sqrt;
mod traits;

pub use disjoint::DisjointSparseTable;
pub use normal::SparseTable;
pub use sqrt::SqrtTable;
pub use traits::{Idempotent, Semigroup};
