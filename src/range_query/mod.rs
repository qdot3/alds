//! Algorithms and Data Structures for Interval Operations.
//!
//! ## Selection Guide
//!
//! |                          | construction                | update       | query                       | online | constraints on operations | comments            |
//! |--------------------------|-----------------------------|--------------|-----------------------------|--------|---------------------------|---------------------|
//! | [`SparseTable`]          | *O*(*N* log *N*)            | N/A          | *O*(1)                      | Yes    | *x* &#x2218; *x* = *x*    |                     |
//! | [`DisjointSparseTable`]  | *O*(*N* log *N*)            | N/A          | *O*(1)                      | Yes    |                           |                     |
//! | [`SegmentTree`]          | *O*(*N* log *N*)            | *O*(log *N*) | *O*(log *N*)                | Yes    | unit element              | single point update |
//! | [`mo_algorithm`]         | *O*(*Q*(log *Q* + log *N*)) | N/A          | *Θ*(*N* / sqrt(*Q*))        | No     | inverse operation         | sort queries        |
//!
//! Common constraints on interval operations.
//! * (*x* &#x2218; *y*) &#x2218; *z* = *x* &#x2218; (*y* &#x2218; *z*)
mod disjoint_sparse_table;
mod mo_alg;
mod segment_tree;
mod sparse_table;

pub use disjoint_sparse_table::DisjointSparseTable;
pub use mo_alg::{hilbert_order, mo_algorithm};
pub use segment_tree::SegmentTree;
pub use sparse_table::SparseTable;
