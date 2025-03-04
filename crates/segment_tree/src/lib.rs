mod traits;
mod normal;
mod lazy;

pub use traits::{Monoid, Map};
pub use normal::SegmentTree;
pub use lazy::LazySegmentTree;