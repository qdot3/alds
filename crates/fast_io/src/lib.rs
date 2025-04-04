mod from_bytes;
mod input;
mod write;

pub use from_bytes::FromBytes;
pub use input::FastInput;
pub use write::{FastOutput, Writable};

pub mod prelude {
    use super::{FastInput, FastOutput};
    use std::io::{stdin, stdout, StdinLock, StdoutLock};

    /// Copied from <https://doc.rust-lang.org/src/std/sys_common/io.rs.html>
    // Bare metal platforms usually have very small amounts of RAM
    // (in the order of hundreds of KB)
    pub const DEFAULT_BUF_SIZE: usize = if cfg!(target_os = "espidf") {
        512
    } else {
        8 * 1024
    };

    /// Constructs a new handle to the standard input of the current process.
    #[inline]
    pub fn fast_stdin_locked() -> FastInput<StdinLock<'static>> {
        FastInput::new(stdin().lock())
    }

    /// Constructs a new handle to the standard output of the current process.
    #[inline]
    pub fn fast_stdout_locked() -> FastOutput<StdoutLock<'static>> {
        FastOutput::new(stdout().lock())
    }
}
