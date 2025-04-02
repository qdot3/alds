use std::num::IntErrorKind;

#[inline]
const fn parse_2_digits_radix_10(mut bytes_le: u16) -> Result<u16, IntErrorKind> {
    if (bytes_le & 0xf0f0) | ((((bytes_le & 0x0808) * 0b11) >> 2) & bytes_le) != 0x3030 {
        return Err(IntErrorKind::InvalidDigit);
    }

    // [b|a] -> [ab]
    bytes_le = (bytes_le & 0x0f0f).wrapping_mul((10 << 8) + 1) >> 8;
    Ok(bytes_le)
}

#[inline]
const fn parse_4_digits_radix_10(mut bytes_le: u32) -> Result<u32, IntErrorKind> {
    if (bytes_le & 0xf0f0_f0f0) | ((((bytes_le & 0x0808_0808) * 0b11) >> 2) & bytes_le)
        != 0x3030_3030
    {
        return Err(IntErrorKind::InvalidDigit);
    }

    // [d|c|b|a] -> [cd|ab]
    bytes_le = (bytes_le & 0x0f0f_0f0f).wrapping_mul((10 << 8) + 1) >> 8;
    // [cd|ab] -> [abcd]
    bytes_le = (bytes_le & 0x00ff_00ff).wrapping_mul((100 << 16) + 1) >> 16;
    Ok(bytes_le)
}

#[inline]
const fn parse_8_digits_radix_10(mut bytes_le: u64) -> Result<u64, IntErrorKind> {
    if (bytes_le & 0xf0f0_f0f0_f0f0_f0f0)
        | ((((bytes_le & 0x0808_0808_0808_0808) * 0b11) >> 2) & bytes_le)
        != 0x3030_3030_3030_3030
    {
        return Err(IntErrorKind::InvalidDigit);
    }

    // [h|g|f|e|d|c|b|a] -> [gh|ef|cd|ab]
    bytes_le = (bytes_le & 0x0f0f_0f0f_0f0f_0f0f).wrapping_mul((10 << 8) + 1) >> 8;
    // [gh|ef|cd|ab] -> [efgh|abcd]
    bytes_le = (bytes_le & 0x00ff_00ff_00ff_00ff).wrapping_mul((100 << 16) + 1) >> 16;
    // [efgh|abcd] -> [abcdefgh]
    bytes_le = (bytes_le & 0x0000_ffff_0000_ffff).wrapping_mul((10000 << 32) + 1) >> 32;

    Ok(bytes_le)
}

macro_rules! parse_digits {
    ( @len 1 @src $digits:expr; as $int_ty:ty ) => {{
        match $digits[0] {
            b @ b'0'..=b'9' => (b - b'0') as $int_ty,
            _ => return Err(IntErrorKind::InvalidDigit),
        }
    }};
    ( @len 2 @src $digits:expr; as $int_ty:ty ) => {{
        parse_2_digits_radix_10(u16::from_le_bytes([$digits[0], $digits[1]]))? as $int_ty
    }};
    ( @len 3 @src $digits:expr; as $int_ty:ty ) => {{
        parse_4_digits_radix_10(u32::from_le_bytes([
            b'0', $digits[0], $digits[1], $digits[2],
        ]))? as $int_ty
    }};
    ( @len 4 @src $digits:expr; as $int_ty:ty ) => {{
        parse_4_digits_radix_10(u32::from_le_bytes([
            $digits[0], $digits[1], $digits[2], $digits[3],
        ]))? as $int_ty
    }};
    ( @len 5 @src $digits:expr; as $int_ty:ty ) => {{
        parse_8_digits_radix_10(u64::from_le_bytes([
            b'0', b'0', b'0', $digits[0], $digits[1], $digits[2], $digits[3], $digits[4],
        ]))? as $int_ty
    }};
    ( @len 6 @src $digits:expr; as $int_ty:ty ) => {{
        parse_8_digits_radix_10(u64::from_le_bytes([
            b'0', b'0', $digits[0], $digits[1], $digits[2], $digits[3], $digits[4], $digits[5],
        ]))? as $int_ty
    }};
    ( @len 7 @src $digits:expr; as $int_ty:ty ) => {{
        parse_8_digits_radix_10(u64::from_le_bytes([
            b'0', $digits[0], $digits[1], $digits[2], $digits[3], $digits[4], $digits[5],
            $digits[6],
        ]))? as $int_ty
    }};
    ( @len 8 @src $digits:expr; as $int_ty:ty ) => {{
        parse_8_digits_radix_10(u64::from_le_bytes([
            $digits[0], $digits[1], $digits[2], $digits[3], $digits[4], $digits[5], $digits[6],
            $digits[7],
        ]))? as $int_ty
    }};

    // interface
    ( $digits:expr; as $int_ty:ty; $combine:ident ) => {{
        let iter = $digits.rchunks_exact(8);
        let rem = iter.remainder();
        let prefix = match rem.len() {
            0 => 0,
            1 => parse_digits! { @len 1 @src rem; as $int_ty },
            2 => parse_digits! { @len 2 @src rem; as $int_ty },
            3 => parse_digits! { @len 3 @src rem; as $int_ty },
            4 => parse_digits! { @len 4 @src rem; as $int_ty },
            5 => parse_digits! { @len 5 @src rem; as $int_ty },
            6 => parse_digits! { @len 6 @src rem; as $int_ty },
            7 => parse_digits! { @len 7 @src rem; as $int_ty },
            _ => unreachable!(),
        };

        let mut result = (0 as $int_ty).$combine(prefix);
        #[allow(overflowing_literals)]
        for chunk in iter.into_iter().rev() {
            result = result
                .wrapping_mul(1_0000_0000)
                .$combine(parse_digits! { @len 8 @src chunk; as $int_ty })
        }
        result
    }};
}

enum Sign {
    Plus,
    Minus,
}

pub trait FromBytes: Sized {
    type Err;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Err>;
}

macro_rules! from_bytes_int_impl {
    ( $( $int_ty:ty )* ) => {$(
        impl FromBytes for $int_ty {
            type Err = IntErrorKind;

            fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Err> {
                if bytes.is_empty() {
                    return Err(IntErrorKind::Empty);
                }
                let (sign, bytes) = match bytes {
                    [b'+' | b'-'] => return Err(IntErrorKind::InvalidDigit),
                    [b'+', rest @ ..] => (Sign::Plus, rest),
                    #[allow(unused_comparisons)]
                    [b'-', rest @ ..] if <$int_ty>::MIN < 0 => (Sign::Minus, rest),
                    _ => (Sign::Plus, bytes),
                };

                enum CheckOverflow {
                    Never,
                    Always,
                    Unknown,
                }

               #[inline]
                fn check_overflow(digits: &[u8]) -> CheckOverflow {
                    const MAX_DIGIT_NUM: usize = <$int_ty>::MAX.ilog10() as usize + 1;
                    const MAX_TEN_POW: $int_ty = (10 as $int_ty).pow(MAX_DIGIT_NUM as u32 - 1);
                    const MAX_PREFIX_BYTE: u8 = (<$int_ty>::MAX / MAX_TEN_POW) as u8 + b'0';

                    match digits.len().cmp(&MAX_DIGIT_NUM) {
                        std::cmp::Ordering::Less => CheckOverflow::Never,
                        std::cmp::Ordering::Equal => match digits[0].cmp(&MAX_PREFIX_BYTE) {
                            std::cmp::Ordering::Less => CheckOverflow::Never,
                            std::cmp::Ordering::Equal => CheckOverflow::Unknown,
                            std::cmp::Ordering::Greater => CheckOverflow::Always,
                        },
                        std::cmp::Ordering::Greater => CheckOverflow::Always,
                    }
                }

                // ignore prefix zeros
                let i = bytes.iter().take_while(|&&b| b == b'0').count();
                match check_overflow(&bytes[i..]) {
                    CheckOverflow::Never => {
                        match sign {
                            Sign::Plus => Ok(parse_digits!(bytes[i..]; as $int_ty; wrapping_add)),
                            Sign::Minus => Ok(parse_digits!(bytes[i..]; as $int_ty; wrapping_sub)),
                        }
                    },
                    CheckOverflow::Unknown => {
                        match sign {
                            Sign::Plus => {
                                let result = parse_digits!(bytes[i..]; as $int_ty; wrapping_add);

                                const MAX_TEN_POW: $int_ty = (10 as $int_ty).pow(<$int_ty>::MAX.ilog10());
                                if result / MAX_TEN_POW == 0 {
                                    Err(IntErrorKind::PosOverflow)
                                } else {
                                    Ok(result)
                                }
                            },
                            Sign::Minus => {
                                let result = parse_digits!(bytes[i..]; as $int_ty; wrapping_sub);

                                if result > 0 {
                                    Err(IntErrorKind::NegOverflow)
                                } else {
                                    Ok(result)
                                }
                            },
                        }
                    },
                    CheckOverflow::Always => {
                        match sign {
                            Sign::Plus => Err(IntErrorKind::PosOverflow),
                            Sign::Minus => Err(IntErrorKind::NegOverflow),
                        }
                    }
                }
            }
        }
    )*};
}

from_bytes_int_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 }

macro_rules! from_bytes_size_impl {
    ( $( $size:ty as $fixed_size:ty ), * $(,)?) => {$(
        impl FromBytes for $size {
            type Err = IntErrorKind;

            fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Err> {
                match <$fixed_size>::from_bytes(bytes) {
                    Ok(v) => Ok(v as $size),
                    Err(e) => Err(e)
                }
            }
        }
    )*};
}

#[cfg(target_pointer_width = "16")]
from_bytes_size_impl! { isize as i16, usize as u16 }
#[cfg(target_pointer_width = "32")]
from_bytes_size_impl! { isize as i32, usize as u32 }
#[cfg(target_pointer_width = "64")]
from_bytes_size_impl! { isize as i64, usize as u64 }

#[cfg(test)]
mod tests {
    use super::FromBytes;

    #[test]
    fn check_min_max() {
        macro_rules! check_min_max {
            ( $( $int_ty:ty )* ) => {$(
                assert_eq!(<$int_ty>::MIN, <$int_ty>::from_bytes(<$int_ty>::MIN.to_string().as_bytes()).unwrap());
                assert_eq!(<$int_ty>::MAX, <$int_ty>::from_bytes(<$int_ty>::MAX.to_string().as_bytes()).unwrap());
            )*};
        }

        check_min_max! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize }
    }
}
