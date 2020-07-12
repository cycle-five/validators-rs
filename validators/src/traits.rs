use alloc::string::String;
use alloc::vec::Vec;

/// Validate and deserialize strings.
pub trait ValidateString {
    type Error;
    type Output;

    fn parse_string<S: Into<String>>(s: S) -> Result<Self::Output, Self::Error>;
    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error>;
    fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error>;
}

/// Validate and deserialize bytes.
pub trait ValidateBytes {
    type Error;
    type Output;

    fn parse_vec_u8<V: Into<Vec<u8>>>(v: V) -> Result<Self::Output, Self::Error>;
    fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<Self::Output, Self::Error>;
    fn validate_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<(), Self::Error>;
}

/// Validate and deserialize characters.
pub trait ValidateChar {
    type Error;
    type Output;

    fn parse_char(c: char) -> Result<Self::Output, Self::Error>;
    fn validate_char(c: char) -> Result<(), Self::Error>;
}

/// Validate and deserialize signed integers.
pub trait ValidateSignedInteger {
    type Error;
    type Output;

    fn parse_i128(i: i128) -> Result<Self::Output, Self::Error>;

    fn validate_i128(i: i128) -> Result<(), Self::Error>;

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i128(i128::from(i))
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i64(i as i64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i32(i as i32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i16(i as i16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i8(i as i8)
    }

    #[inline]
    fn parse_i64(i: i64) -> Result<Self::Output, Self::Error> {
        Self::parse_i128(i128::from(i))
    }

    #[inline]
    fn parse_i32(i: i32) -> Result<Self::Output, Self::Error> {
        Self::parse_i64(i64::from(i))
    }

    #[inline]
    fn parse_i16(i: i16) -> Result<Self::Output, Self::Error> {
        Self::parse_i32(i32::from(i))
    }

    #[inline]
    fn parse_i8(i: i8) -> Result<Self::Output, Self::Error> {
        Self::parse_i16(i16::from(i))
    }

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i128(i128::from(i))
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i64(i as i64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i32(i as i32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i16(i as i16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i8(i as i8)
    }

    #[inline]
    fn validate_i64(i: i64) -> Result<(), Self::Error> {
        Self::validate_i128(i128::from(i))
    }

    #[inline]
    fn validate_i32(i: i32) -> Result<(), Self::Error> {
        Self::validate_i64(i64::from(i))
    }

    #[inline]
    fn validate_i16(i: i16) -> Result<(), Self::Error> {
        Self::validate_i32(i32::from(i))
    }

    #[inline]
    fn validate_i8(i: i8) -> Result<(), Self::Error> {
        Self::validate_i16(i16::from(i))
    }
}

/// Validate and deserialize unsigned integers.
pub trait ValidateUnsignedInteger {
    type Error;
    type Output;

    fn parse_u128(u: u128) -> Result<Self::Output, Self::Error>;

    fn validate_u128(u: u128) -> Result<(), Self::Error>;

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u128(u128::from(u))
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u64(u as u64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u32(u as u32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u16(u as u16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u8(u as u8)
    }

    #[inline]
    fn parse_u64(u: u64) -> Result<Self::Output, Self::Error> {
        Self::parse_u128(u128::from(u))
    }

    #[inline]
    fn parse_u32(u: u32) -> Result<Self::Output, Self::Error> {
        Self::parse_u64(u64::from(u))
    }

    #[inline]
    fn parse_u16(u: u16) -> Result<Self::Output, Self::Error> {
        Self::parse_u32(u32::from(u))
    }

    #[inline]
    fn parse_u8(u: u8) -> Result<Self::Output, Self::Error> {
        Self::parse_u16(u16::from(u))
    }

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u128(u128::from(u))
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u64(u as u64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u32(u as u32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u16(u as u16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u8(u as u8)
    }

    #[inline]
    fn validate_u64(u: u64) -> Result<(), Self::Error> {
        Self::validate_u128(u128::from(u))
    }

    #[inline]
    fn validate_u32(u: u32) -> Result<(), Self::Error> {
        Self::validate_u64(u64::from(u))
    }

    #[inline]
    fn validate_u16(u: u16) -> Result<(), Self::Error> {
        Self::validate_u32(u32::from(u))
    }

    #[inline]
    fn validate_u8(u: u8) -> Result<(), Self::Error> {
        Self::validate_u16(u16::from(u))
    }
}