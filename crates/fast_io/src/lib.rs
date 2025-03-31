mod from_bytes;
mod write;

// pub use fast_in::;
pub use from_bytes::FromBytes;
pub use write::{FastWrite, Writable};

/// [proconio](https://docs.rs/proconio/latest/proconio/)
pub mod marker {
    use super::FromBytes;
    use proconio::source::{Readable, Source};
    use std::{any::type_name, io::BufRead};

    macro_rules! readable_int_impl {
        ( $( $wrapper:tt ($inner:ty) )* ) => {$(
            pub struct $wrapper;

            impl Readable for $wrapper {
                type Output = $inner;

                fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output {
                    let token = source.next_token_unwrap();
                    <$inner>::from_bytes(token.as_bytes()).unwrap_or_else(|e| panic!(
                        concat!(
                            "failed to parse the input `{input}` ",
                            "to the value of type `{ty}`: {err:?}; ",
                            "ensure that the input format is correctly specified ",
                            "and that the input value must handle specified type.",
                        ),
                        input = token,
                        ty = type_name::<$inner>(),
                        err = e,
                    ))
                }
            }
        )*};
    }

    readable_int_impl! {
        I8(i8) U8(u8) I16(i16) U16(u16) I32(i32) U32(u32) I64(i64) U64(u64)
        I128(i128) U128(u128) Isize(isize) Usize(usize)
    }
}
