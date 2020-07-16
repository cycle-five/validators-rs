#![allow(dead_code)]

use core::fmt::{self, Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TypeEnum {
    String,
    VecU8,
    Boolean,
    U16,
    U64,
    U128,
    Number,
    SignedInteger,
    UnsignedInteger,
    OptionU16,
    OptionString,
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
    Host,
    Protocol,
    Serde,
    Version,
    VersionReq,
    Url,
    CollectionLength,
}

impl TypeEnum {
    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            TypeEnum::String => "String",
            TypeEnum::VecU8 => "Vec<u8>",
            TypeEnum::Boolean => "bool",
            TypeEnum::U16 => "u16",
            TypeEnum::U64 => "u64",
            TypeEnum::U128 => "u128",
            TypeEnum::Number => "f32 | f64",
            TypeEnum::SignedInteger => "isize | i8 | i16 | i32 | i64 | i128",
            TypeEnum::UnsignedInteger => "usize | u8 | u16 | u32 | u64 | u128",
            TypeEnum::OptionU16 => "Option<u16>",
            TypeEnum::OptionString => "Option<String>",
            TypeEnum::IpAddr => "std::net::IpAddr",
            TypeEnum::Ipv4Addr => "std::net::Ipv4Addr",
            TypeEnum::Ipv6Addr => "std::net::Ipv6Addr",
            TypeEnum::Host => "crate::validators::models::Host",
            TypeEnum::Protocol => "crate::validators::models::Protocol",
            TypeEnum::Serde => "T: crate::serde::se::Serialize + crate::serde::de::Deserialize",
            TypeEnum::Version => "crate::semver::Version",
            TypeEnum::VersionReq => "crate::semver::VersionReq",
            TypeEnum::Url => "url::Url",
            #[cfg(feature = "serde")]
            TypeEnum::CollectionLength => "T: crate::validators::traits::CollectionLength + crate::serde::se::Serialize + crate::serde::de::Deserialize",
            #[cfg(not(feature = "serde"))]
            TypeEnum::CollectionLength => "T: crate::validators::traits::CollectionLength",
        }
    }
}

impl Debug for TypeEnum {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.as_str())
    }
}
