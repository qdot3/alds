//! Algorithms and Data Structures for Range Operations.
//!
//! ## Selection Guide
//!
//! |                  | create                      | update       | query                       | online | constraints on operations | comments     |
//! |------------------|-----------------------------|--------------|-----------------------------|--------|---------------------------|--------------|
//! | [`SparseTable`]  | *O*(*N* log *N*)            | N/A          | *O*(1)                      | Yes    | *x* &#x2218; *x* = *x*    |              |
//! | [`SegmentTree`]  | *O*(*N* log *N*)            | *O*(log *N*) | *O*(log *N*)                | Yes    | unit element              |              |
//! | [`mo_algorithm`] | *O*(*Q*(log *Q* + log *N*)) | N/A          | *O*(*N* sqrt(*Q*)) in total | No     |                           | sort queries |
//!
//! Common constraints on range operations:
//! * associativity: (*x* &#x2218; *y*) &#x2218; *z* = *x* &#x2218; (*y* &#x2218; *z*)
mod mo_alg;
mod segment_tree;
mod sparse_table;

pub use mo_alg::mo_algorithm;
pub use segment_tree::SegmentTree;
pub use sparse_table::SparseTable;
