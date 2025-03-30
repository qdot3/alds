mod fast_in;
mod write;
mod from_bytes;

pub use write::{FastWrite, Writable};
pub use from_bytes::FromBytes;
pub use fast_in::FastInput;