use std::num::IntErrorKind;

#[inline]
fn parse_2_digits_radix_10(mut bytes_le: u16) -> Result<u16, IntErrorKind> {
    if (bytes_le & 0xf0f0) | (((bytes_le & 0x0808) * 0b11 >> 2) & bytes_le) != 0x3030 {
        return Err(IntErrorKind::InvalidDigit);
    }

    // [b|a] -> [ab]
    bytes_le = (bytes_le & 0x0f0f).wrapping_mul((10 << 8) + 1) >> 8;
    Ok(bytes_le)
}

#[inline]
fn parse_4_digits_radix_10(mut bytes_le: u32) -> Result<u32, IntErrorKind> {
    if (bytes_le & 0xf0f0_f0f0) | (((bytes_le & 0x0808_0808) * 0b11 >> 2) & bytes_le) != 0x3030_3030
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
fn parse_8_digits_radix_10(mut bytes_le: u64) -> Result<u64, IntErrorKind> {
    if (bytes_le & 0xf0f0_f0f0_f0f0_f0f0)
        | (((bytes_le & 0x0808_0808_0808_0808) * 0b11 >> 2) & bytes_le)
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

pub enum Sign {
    Plus,
    Minus,
}

pub trait FromBytes: Sized {
    type Output;

    fn from_bytes(bytes: &[u8]) -> Self::Output;
}

macro_rules! from_bytes_impl {
    ( $( $int_ty:ty[max_len=$n:expr; max_prefix=$b:expr] )* ) => {$(
        impl FromBytes for $int_ty {
            type Output = Result<Self, IntErrorKind>;

            fn from_bytes(bytes: &[u8]) -> Self::Output {
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

                let i = bytes.iter().take_while(|&&b| b == b'0').count();
                match (bytes.len() - i).cmp(&$n) {
                    std::cmp::Ordering::Less => match sign {
                        Sign::Plus => Ok(parse_digits!(bytes[i..]; as $int_ty; wrapping_add)),
                        Sign::Minus => Ok(parse_digits!(bytes[i..]; as $int_ty; wrapping_sub)),
                    },
                    std::cmp::Ordering::Equal if bytes[i] <= $b => {
                        match sign {
                            Sign::Plus => {
                                let result = parse_digits!(bytes[i..]; as $int_ty; wrapping_add);

                                const TH: $int_ty = (10 as $int_ty).pow($n as u32 - 1);
                                if result / TH == 0 {
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
                    }
                    _ => {
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

from_bytes_impl! { i128[max_len=39; max_prefix=b'1'] }
from_bytes_impl! { u128[max_len=39; max_prefix=b'3'] }
from_bytes_impl! { i64 [max_len=19; max_prefix=b'9'] }
from_bytes_impl! { u64 [max_len=20; max_prefix=b'1'] }
from_bytes_impl! { isize [max_len=19; max_prefix=b'9'] }
from_bytes_impl! { usize [max_len=20; max_prefix=b'1'] }
from_bytes_impl! { i32 [max_len=10; max_prefix=b'2'] }
from_bytes_impl! { u32 [max_len=10; max_prefix=b'4'] }
from_bytes_impl! { i16 [max_len=5;  max_prefix=b'3'] }
from_bytes_impl! { u16 [max_len=5;  max_prefix=b'6'] }
from_bytes_impl! { i8  [max_len=3;  max_prefix=b'1'] }
from_bytes_impl! { u8  [max_len=3;  max_prefix=b'2'] }

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

        check_min_max! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 }
    }
}
