use std::{
    io::{self, BufWriter, Write},
    mem::MaybeUninit,
    ptr, slice,
};

pub trait FastWrite: Write {
    fn fast_write<T>(&mut self, value: T) -> io::Result<usize>
    where
        T: Writable,
    {
        value.write(self)
    }

    fn fast_writeln<T>(&mut self, value: T) -> io::Result<usize>
    where
        T: Writable,
    {
        Ok(value.write(self)? + self.write(b"\n")?)
    }

    fn fast_write_all<T, U>(&mut self, values: &[T], sep: U) -> io::Result<usize>
    where
        T: Writable,
        U: Writable,
    {
        let mut iter = values.into_iter();
        let mut n = 0;
        if let Some(value) = iter.next() {
            n += value.write(self)?;
            for value in iter {
                n += sep.write(self)?;
                n += value.write(self)?;
            }
        }

        Ok(n)
    }

    fn fast_writeln_all<T, U>(&mut self, values: &[T], sep: U) -> io::Result<usize>
    where
        T: Writable,
        U: Writable,
    {
        let mut iter = values.into_iter();
        let mut n = 0;
        if let Some(value) = iter.next() {
            n += value.write(self)?;
            for value in iter {
                n += sep.write(self)?;
                n += value.write(self)?;
            }
        }
        n += self.write(b"\n")?;

        Ok(n)
    }
}

impl<W: Write> FastWrite for BufWriter<W> {}

pub trait Writable {
    fn write<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<usize>;
}

impl Writable for String {
    fn write<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write(self.as_bytes())
    }
}

impl Writable for str {
    fn write<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write(self.as_bytes())
    }
}

macro_rules! writable_int_impl {
    ( $( ($signed:ty, $unsigned:ty) ),* ) => {$(
        impl Writable for $unsigned {
            fn write<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<usize> {
                let mut num = *self;
                const SIZE: usize = <$unsigned>::MAX.ilog10() as usize + 1;
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
                        while num >= 10_000 {
                            let rem = (num % 10000) as usize;
                            num /= 10_000;

                            // We are allowed to copy to `buf_ptr[curr..curr + 3]` here since
                            // otherwise `curr < 0`. But then `n` was originally at least `10000^10`
                            // which is `10^40 > 2^128 > n`.
                            curr -= 4;
                            ptr::copy_nonoverlapping(lut_ptr.add(rem as usize * 4), buf_ptr.add(curr), 4);
                        }
                    }

                    // if we reach here numbers are <= 9999, so at most 4 chars long
                    // possibly reduce 64bit math
                    let num = num as usize;
                    // decode at most 4 chars
                    if num >= 1_000 {
                        curr -= 4;
                        ptr::copy_nonoverlapping(lut_ptr.add(num as usize * 4), buf_ptr.add(curr), 4);
                    } else if num >= 100 {
                        curr -= 3;
                        ptr::copy_nonoverlapping(lut_ptr.add(num as usize * 4 + 1), buf_ptr.add(curr), 3);
                    } else if num >= 10 {
                        curr -= 2;
                        ptr::copy_nonoverlapping(lut_ptr.add(num as usize * 4 + 2), buf_ptr.add(curr), 2);
                    } else {
                        curr -= 1;
                        ptr::copy_nonoverlapping(lut_ptr.add(num as usize * 4 + 3), buf_ptr.add(curr), 1);
                    }
                }

                unsafe { writer.write(slice::from_raw_parts(buf_ptr.add(curr), buf.len() - curr)) }
            }
        }

        impl Writable for $signed {
            fn write<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<usize> {
                if self.is_negative() {
                    writer.write(b"-")?;
                }
                self.unsigned_abs().write(writer)
            }
        }
    )*};
}

// TODO: specialization for 128 bit integers
writable_int_impl! { (i8, u8), (i16, u16), (i32, u32), (i64, u64), (isize, usize), (i128, u128) }

// look up table
static DEC_DIGITS_LUT: [u8; 40000] = {
    let mut lut = [0; 40_000];
    let mut i = 0;
    while i < 10_000 {
        let (upper, lower) = (i / 100, i % 100);
        lut[i * 4 + 0] = (upper / 10) as u8 + b'0';
        lut[i * 4 + 1] = (upper % 10) as u8 + b'0';
        lut[i * 4 + 2] = (lower / 10) as u8 + b'0';
        lut[i * 4 + 3] = (lower % 10) as u8 + b'0';
        i += 1
    }
    lut
};
