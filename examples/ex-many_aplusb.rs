// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum

use std::{fmt::Write, io::Read, num::IntErrorKind};

fn main() {
    let mut buf_r = String::with_capacity(40_000_000);
    let _ = std::io::stdin().lock().read_to_string(&mut buf_r);
    let mut num = buf_r.split_ascii_whitespace().skip(1);

    let mut buf_w = String::with_capacity(20_000_000);
    while let Some(x) = num.next() {
        let y = num.next().unwrap();

        write!(
            &mut buf_w,
            "{}",
            // x.parse::<u64>().unwrap() + y.parse::<u64>().unwrap()
            u64::from_str_radix10(x).unwrap() + u64::from_str_radix10(y).unwrap()
        )
        .unwrap();
        buf_w.push('\n');
    }

    print!("{}", buf_w)
}

pub trait FromStrRadix10: Sized {
    fn from_str_radix10(src: &str) -> Result<Self, IntErrorKind>;
}

macro_rules! from_str_radix10_impl_large {
    ( $($int:ty)* ) => {$(
        impl FromStrRadix10 for $int {
            fn from_str_radix10(src: &str) -> Result<Self, IntErrorKind> {
                let src = src.as_bytes();
                let (is_positive, digits) = match src {
                    [b'+'| b'-'] => return Err(IntErrorKind::InvalidDigit),
                    [b'+', rest @ ..] => (true, rest),
                    [b'-', rest @ ..] => (false, rest),
                    _ => (true, src),
                };

                macro_rules! perform_checked_op {
                    ( $checked_op:ident, $overflow:ident ) => {{
                        let mut res: $int = 0;

                        const N: usize = 8;
                        let i = digits.len() % N;
                        if i > 0 {
                            let offset = N - i;
                            let val = from_str_radix10_8digits(std::array::from_fn(|j| {
                                (j >= offset).then(|| digits[j - offset]).unwrap_or(b'0')
                            }))?;

                            let val = <$int>::try_from(val).map_err(|_| IntErrorKind::$overflow)?;
                            res = res.$checked_op(val).ok_or(IntErrorKind::$overflow)?;
                        }
                        for digits in digits[i..].chunks(N) {
                            let val = from_str_radix10_8digits(std::array::from_fn(|i| digits[i]))?;
                            let val = <$int>::try_from(val).map_err(|_| IntErrorKind::$overflow)?;

                            res = res
                                .checked_mul(1_0000_0000)
                                .and_then(|res| res.$checked_op(val))
                                .ok_or(IntErrorKind::$overflow)?;
                        }

                        res
                    }};
                }

                let res = if is_positive {
                    perform_checked_op!(checked_add, PosOverflow)
                } else {
                    perform_checked_op!(checked_sub, NegOverflow)
                };
                Ok(res)
            }
        }
    )*};
}

from_str_radix10_impl_large! { u32 u64 u128 i32 i64 i128 }

macro_rules! from_str_radix10_impl_small {
    ( $($int:ty)* ) => {$(
        impl FromStrRadix10 for $int {
            fn from_str_radix10(src: &str) -> Result<Self, IntErrorKind> {
                let src = src.as_bytes();
                let (is_positive, digits) = match src {
                    [b'+'| b'-'] => return Err(IntErrorKind::InvalidDigit),
                    [b'+', rest @ ..] => (true, rest),
                    [b'-', rest @ ..] => (false, rest),
                    _ => (true, src),
                };

                macro_rules! perform_checked_op {
                    ( $checked_op:ident, $overflow:ident ) => {{
                        let mut res: $int = 0;

                        const N: usize = 8;
                        let i = digits.len() % N;
                        let offset = N - i;
                        let val = from_str_radix10_8digits(std::array::from_fn(|j| {
                            (j >= offset).then(|| digits[j - offset]).unwrap_or(b'0')
                        }))?;

                        let val = <$int>::try_from(val).map_err(|_| IntErrorKind::$overflow)?;
                        res = res.$checked_op(val).ok_or(IntErrorKind::$overflow)?;

                        res
                    }};
                }

                let res = if is_positive {
                    perform_checked_op!(checked_add, PosOverflow)
                } else {
                    perform_checked_op!(checked_sub, NegOverflow)
                };
                Ok(res)
            }
        }
    )*};
}

from_str_radix10_impl_small! { u8 u16 i8 i16 }

const fn from_str_radix10_8digits(digits: [u8; 8]) -> Result<u64, IntErrorKind> {
    let mut reversed = u64::from_le_bytes(digits);

    // validation
    // b'0' = 0x30, b'9' = 0x39
    if reversed & 0xf0f0_f0f0_f0f0_f0f0 != 0x3030_3030_3030_3030 {
        return Err(IntErrorKind::InvalidDigit);
    }
    // 8 = 0b1000, 9 = 0b1001, 0xa = 0b1010, ..
    let test =
        ((reversed & 0x0404_0404_0404_0404) << 1) | ((reversed & 0x0202_0202_0202_0202) << 2);
    if (reversed & 0x0808_0808_0808_0808) & test != 0 {
        return Err(IntErrorKind::InvalidDigit);
    }

    // |h|g|f|e|d|c|b|a| -> |gh|ef|cd|ab|
    reversed = (reversed & 0x0f0f_0f0f_0f0f_0f0f).wrapping_mul((10 << 8) + 1) >> 8;
    // |gh|ef|cd|ba| -> |efgh|abcd|
    reversed = (reversed & 0x00ff_00ff_00ff_00ff).wrapping_mul((100 << 16) + 1) >> 16;
    // |efgh|abcd| -> |abcdefgh|
    reversed = (reversed & 0x0000_ffff_0000_ffff).wrapping_mul((10_000 << 32) + 1) >> 32;

    Ok(reversed)
}
