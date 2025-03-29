use std::mem::MaybeUninit;
use std::ptr;
use std::slice;

// 2 digit decimal look up table
static DEC_DIGITS_LUT: &[u8; 200] = b"0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

macro_rules! into_bytes_impl {
    ( $( $signed:ty, $unsigned:ty, )* ) => {$(
        impl IntoBytes for $unsigned {
            fn into_bytes(mut self) -> Vec<u8> {
                const SIZE: usize = <$unsigned>::MAX.ilog(10) as usize + 1;
                let mut buf = [MaybeUninit::<u8>::uninit(); SIZE];
                let mut curr = SIZE;
                let buf_ptr = buf.as_mut_ptr() as *mut u8;
                let lut_ptr = DEC_DIGITS_LUT.as_ptr();

                // SAFETY: Since `d1` and `d2` are always less than or equal to `198`, we
                // can copy from `lut_ptr[d1..d1 + 1]` and `lut_ptr[d2..d2 + 1]`. To show
                // that it's OK to copy into `buf_ptr`, notice that at the beginning
                // `curr == buf.len() == 39 > log(n)` since `n < 2^128 < 10^39`, and at
                // each step this is kept the same as `n` is divided. Since `n` is always
                // non-negative, this means that `curr > 0` so `buf_ptr[curr..curr + 1]`
                // is safe to access.
                unsafe {
                    // need at least 16 bits for the 4-characters-at-a-time to work.
                    #[allow(overflowing_literals)]
                    #[allow(unused_comparisons)]
                    // This block will be removed for smaller types at compile time and in the worst
                    // case, it will prevent to have the `10000` literal to overflow for `i8` and `u8`.
                    if core::mem::size_of::<$unsigned>() >= 2 {
                        // eagerly decode 4 characters at a time
                        while self >= 10000 {
                            let rem = (self % 10000) as usize;
                            self /= 10000;

                            let d1 = (rem / 100) << 1;
                            let d2 = (rem % 100) << 1;
                            curr -= 4;

                            // We are allowed to copy to `buf_ptr[curr..curr + 3]` here since
                            // otherwise `curr < 0`. But then `n` was originally at least `10000^10`
                            // which is `10^40 > 2^128 > n`.
                            ptr::copy_nonoverlapping(lut_ptr.add(d1 as usize), buf_ptr.add(curr), 2);
                            ptr::copy_nonoverlapping(lut_ptr.add(d2 as usize), buf_ptr.add(curr + 2), 2);
                        }
                    }

                    // if we reach here numbers are <= 9999, so at most 4 chars long
                    let mut n = self as usize; // possibly reduce 64bit math

                    // decode 2 more chars, if > 2 chars
                    if n >= 100 {
                        let d1 = (n % 100) << 1;
                        n /= 100;
                        curr -= 2;
                        ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(curr), 2);
                    }

                    // if we reach here numbers are <= 100, so at most 2 chars long
                    // The biggest it can be is 99, and 99 << 1 == 198, so a `u8` is enough.
                    // decode last 1 or 2 chars
                    if n < 10 {
                        curr -= 1;
                        *buf_ptr.add(curr) = (n as u8) + b'0';
                    } else {
                        let d1 = n << 1;
                        curr -= 2;
                        ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(curr), 2);
                    }
                }

                // SAFETY: `curr` > 0 (since we made `buf` large enough), and all the chars are valid
                // UTF-8 since `DEC_DIGITS_LUT` is
                let buf_vec = unsafe {
                    Vec::from(slice::from_raw_parts(buf_ptr.add(curr), buf.len() - curr))
                };

                buf_vec
            }
        }

        impl IntoBytes for $signed {
            fn into_bytes(self) -> Vec<u8> {
                if self < 0 {
                    let mut result = self.unsigned_abs().into_bytes();
                    result.insert(0, b'-');
                    result
                } else {
                    self.unsigned_abs().into_bytes()
                }
            }
        }
    )*};
}

into_bytes_impl! {
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    i128, u128,
    isize, usize,
}
