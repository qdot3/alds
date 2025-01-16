//! Data Structures for range operations.
//!
//! ## Selection Guide
//!
//! |                 | update | online | operation constraints  |
//! |-----------------|--------|--------|------------------------|
//! | [`SparseTable`] | No     | Yes    | *x* &#x2218; *x* = *x* |
//! | [`SegmentTree`] | Yes    | Yes    | unit element           |
//!
//! Common constraints on range operations:
//! * (*x* &#x2218; *y*) &#x2218; *z* = *y* &#x2218; (*x* &#x2218; *z*)
mod segment_tree;
mod sparse_table;

pub use segment_tree::SegmentTree;
pub use sparse_table::SparseTable;
