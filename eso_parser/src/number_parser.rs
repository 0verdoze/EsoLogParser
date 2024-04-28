use atoi::{FromRadix10, FromRadix10Signed};
use crate::eso_serde::Error;

use std::intrinsics::likely;

pub(crate) trait NumberParser
where
    Self: Sized,
{
    fn parse(s: &[u8]) -> Result<(Self, usize), Error>;
}

macro_rules! impl_int_parser {
    ($fn:expr, $t:ty) => {
        impl NumberParser for $t {
            #[inline]
            fn parse(s: &[u8]) -> Result<(Self, usize), Error> {
                let (number, parsed_bytes) = $fn(s);
                let next_byte = s.get(parsed_bytes);
                let add = if next_byte.is_some() { 1 } else { 0 };

                if likely((parsed_bytes != 0) & (next_byte == Some(&b',')) | next_byte.is_none()) {
                    Ok((number, parsed_bytes + add))
                } else {
                    Err(Error::ParseIntError)
                }
            }
        }
    };

    ($fn:expr, $t:ty, $($ts:ty),+) => {
        impl_int_parser!( $fn, $t );
        impl_int_parser!( $fn, $($ts),+ );
    };
}

// macro_rules! impl_float_parser {
//     ($t:ty) => {
//         impl NumberParser for $t {
//             #[inline]
//             fn parse(s: &[u8]) -> Result<(Self, usize), Error> {
//                 match Self::parse_float_partial(s) {
//                     Ok((number, parsed_bytes)) => {
//                         let next_byte = s.get(parsed_bytes);
//                         let add = if next_byte.is_some() { 1 } else { 0 };
//                         if likely((next_byte == Some(&b',')) | next_byte.is_none()) {
//                             Ok((number, parsed_bytes + add))
//                         } else {
//                             Err(Error::ParseFloatError)
//                         }
//                     },
//                     Err(_) => Err(Error::ParseFloatError),
//                 }
//             }
//         }
//     };
//     ($t:ty, $($ts:ty),+) => {
//         impl_float_parser!( $t );
//         impl_float_parser!( $($ts),+ );
//     };
// }

macro_rules! impl_float_parser {
    ($t:ty) => {
        impl NumberParser for $t {
            #[inline]
            fn parse(s: &[u8]) -> Result<(Self, usize), Error> {
                let (num, parsed): (i64, _) = FromRadix10Signed::from_radix_10_signed(s);
                let next_byte = s.get(parsed);
                let num = num as Self;

                match next_byte {
                    Some(b'.') => {
                        let (decimal_part, decimal_parsed): (u64, _) = FromRadix10::from_radix_10(&s[parsed + 1..]);
                        let parsed_total = parsed + decimal_parsed + 1;

                        // this is funny, with lto enabled FMA is fastest, and /4.0 is the slowest
                        // but with lto disabled its the opposite, *0.25 is always in between
                        let final_number = if likely(decimal_parsed == 4) {
                            // i would expect fma to be faster but it not, 3% regression
                            (0.25 as Self).mul_add(decimal_part as Self, num)
                            // / 4.0, and * 0.25 are almost equal, 0.25 is actually a bit slower (1%, weird stuff)
                            // num + decimal_part as Self * 0.25
                            // num + decimal_part as Self / 4.0
                        } else {
                            num + decimal_part as Self / decimal_parsed as Self
                        };

                        let next_byte = s.get(parsed_total);
                        let add = next_byte.is_some() as usize;
                        if (next_byte == Some(&b',')) | next_byte.is_none() {
                            Ok((final_number, parsed_total + add))
                        } else {
                            Err(Error::ParseIntError)
                        }
                    },
                    Some(b',')|None => {
                        Ok((num, parsed + next_byte.is_some() as usize))
                    },
                    _ => Err(Error::ParseFloatError),
                }
            }
        }
    };

    ($t:ty, $($ts:ty),+) => {
        impl_float_parser!( $t );
        impl_float_parser!( $($ts),+ );
    };
}

impl_int_parser! {
    FromRadix10Signed::from_radix_10_signed,
    i8,
    i16,
    i32,
    i64
}

impl_int_parser! {
    FromRadix10::from_radix_10,
    u8,
    u16,
    u32,
    u64
}

impl_float_parser! {
    f32,
    f64
}
