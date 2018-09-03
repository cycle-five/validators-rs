//! # Validators
//!
//! This crate provides many validators for validating data from users and modeling them to structs without much extra effort.
//!
//! All validators are separated into different modules and unified for two main types: **XXX** and **XXXValidator** where **XXX** is a type that you want to validate.
//! The former is a struct or a enum, and the latter is a struct which can be considered as a generator of the former.
//! A **XXXValidator** struct usually contains some values of `ValidatorOption` in order to use different rules to check data.
//!
//! For example, the mod `domain` has `Domain` and `DomainValidator` structs. If we want to create a `Domain` instance, we need to create a `DomainValidator` instance first.
//! When initialing a `DomainValidator`, we can choose to make this `DomainValidator` **allow** or **not allow** the input to have or **must** have a port number.
//!
//! ```
//! extern crate validators;
//!
//! use validators::ValidatorOption;
//! use validators::domain::{Domain, DomainValidator};
//!
//! let domain = "tool.magiclen.org:8080".to_string();
//!
//! let dv = DomainValidator {
//!     port: ValidatorOption::Allow,
//!     localhost: ValidatorOption::NotAllow,
//! };
//!
//! let domain = dv.parse_string(domain).unwrap();
//!
//! assert_eq!("tool.magiclen.org:8080", domain.get_full_domain());
//! assert_eq!("tool.magiclen.org", domain.get_full_domain_without_port());
//! assert_eq!("org", domain.get_top_level_domain().unwrap());
//! assert_eq!("tool", domain.get_sub_domain().unwrap());
//! assert_eq!("magiclen", domain.get_domain());
//! assert_eq!(8080, domain.get_port().unwrap());
//! ```
//!
//! If you want the **XXX** model to be stricter, you can use its wrapper type which is something like **XXXWithPort** or **XXXWithoutPort**.
//! For instance, `Domain` has some wrappers, such as **DomainLocalhostableWithPort**, **DomainLocalhostableAllowPort** and **DomainLocalhostableWithoutPort**.
//!
//! ```
//! extern crate validators;
//!
//! use validators::domain::{DomainLocalhostableWithPort};
//!
//! let domain = "tool.magiclen.org:8080".to_string();
//!
//! let domain = DomainLocalhostableWithPort::from_string(domain).unwrap();
//!
//! assert_eq!("tool.magiclen.org:8080", domain.get_full_domain());
//! assert_eq!("tool.magiclen.org", domain.get_full_domain_without_port());
//! assert_eq!("org", domain.get_top_level_domain().unwrap());
//! assert_eq!("tool", domain.get_sub_domain().unwrap());
//! assert_eq!("magiclen", domain.get_domain());
//! assert_eq!(8080, domain.get_port()); // This function does not use `Option` as its return value, because the struct `DomainLocalhostableWithPort` has already made sure the input must have a port number!
//! ```
//!
//! This crate aims to use the simplest and slackest way (normally only use regular expressions) to validate data, in order to minimize the overhead.
//! Therefore, it may not be competent in some critical situations. Use it carefully. Check out the documentation to see more useful validators and wrapper types.
//!
//! ## Customization
//!
//! This crate also provides macros to create customized validated structs for any strings and numbers.
//!
//! For example, to create a struct which only allows **"Hi"** or **"Hello"** restricted by a regular expression,
//!
//! ```
//! #[macro_use] extern crate validators;
//!
//! validated_customized_regex_string!(Greet, "^(Hi|Hello)$");
//!
//! let s = Greet::from_str("Hi").unwrap();
//! ```
//!
//! You can also make your struct public by adding a **pub** keyword,
//!
//! ```
//! #[macro_use] extern crate validators;
//!
//! validated_customized_regex_string!(pub Greet, "^(Hi|Hello)$");
//!
//! let s = Greet::from_str("Hi").unwrap();
//! ```
//!
//! For numbers limited in a range,
//!
//! ```
//! #[macro_use] extern crate validators;
//!
//! validated_customized_ranged_number!(Score, u8, 0, 100);
//!
//! let score = Score::from_str("80").unwrap();
//! ```
//!
//! For a Vec whose length is limited in a range,
//!
//! ```
//! #[macro_use] extern crate validators;
//!
//! validated_customized_regex_string!(Name, "^[A-Z][a-zA-Z]*( [A-Z][a-zA-Z]*)*$");
//! validated_customized_ranged_length_vec!(Names, 1, 5);
//!
//! let mut names = Vec::new();
//!
//! names.push(Name::from_str("Ron").unwrap());
//! names.push(Name::from_str("Magic Len").unwrap());
//!
//! let names = Names::from_vec(names).unwrap();
//! ```
//!
//! All validated wrapper types and validated customized structs implement the `ValidatedWrapper` trait.
//!
//! Read the documentation to know more helpful customized macros.
//!
//! ## Rocket Support
//!
//! This crate supports [Rocket](https://rocket.rs/) framework. All validated wrapper types and validated customized structs implement the `FromFormValue` trait.
//! To use with Rocket support, you have to enable **rocketly** feature for this crate.
//!
//! ```toml
//! [dependencies.validators]
//! version = "*"
//! features = ["rocketly"]
//! ```
//!
//! For example,
//!
//! ```rust,ignore
//! #![feature(plugin)]
//! #![feature(custom_derive)]
//! #![plugin(rocket_codegen)]
//!
//! #[macro_use] extern crate validators;
//!
//! extern crate rocket;
//!
//! use rocket::request::Form;
//!
//! use validators::http_url::HttpUrlUnlocalableWithProtocol;
//! use validators::email::Email;
//!
//! validated_customized_regex_string!(Name, r"^[\S ]{1,80}$");
//! validated_customized_ranged_number!(PersonAge, u8, 0, 130);
//!
//! #[derive(Debug, FromForm)]
//! struct ContactModel {
//!     name: Name,
//!     age: Option<PersonAge>,
//!     email: Email,
//!     url: Option<HttpUrlUnlocalableWithProtocol>
//! }
//!
//! #[post("/contact", data = "<model>")]
//! fn contact(model: Form<ContactModel>) -> &'static str {
//!     println!("{:?}", model);
//!     "do something..."
//! }
//! ```

#![cfg_attr(feature = "nightly", feature(ip))]

#[doc(hidden)]
pub extern crate regex;

#[cfg(feature = "rocketly")]
#[doc(hidden)]
pub extern crate rocket;

use std::fmt::{Display, Debug};
use std::cmp::PartialEq;
use std::str::Utf8Error;

#[doc(hidden)]
pub const REGEX_SIZE_LIMIT: usize = 26214400;

pub enum ValidatorOption {
    Must,
    Allow,
    NotAllow,
}

impl ValidatorOption {
    pub fn allow(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => true,
            ValidatorOption::NotAllow => false
        }
    }

    pub fn not_allow(&self) -> bool {
        match self {
            ValidatorOption::Must => false,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => true
        }
    }

    pub fn must(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => false
        }
    }
}

pub trait Validated: Display + PartialEq + Clone + Debug {}

pub trait ValidatedWrapper: Validated {
    type Error;

    fn from_string(from_string_input: String) -> Result<Self, Self::Error>;

    fn from_str(from_str_input: &str) -> Result<Self, Self::Error>;
}

pub mod domain;
pub mod email;
pub mod ipv4;
pub mod ipv6;
pub mod host;
pub mod http_url;
pub mod base64;
pub mod base64_url;
pub mod base32;
pub mod short_crypt_url_component;
pub mod short_crypt_qr_code_alphanumeric;

// TODO -----ValidatedCustomizedString START-----

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedStringError {
    RegexError(regex::Error),
    NotMatch,
    UTF8Error(Utf8Error),
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a> ::validators::rocket::request::FromFormValue<'a> for $name {
            type Error = ::validators::ValidatedCustomizedStringError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedStringError::UTF8Error(err))?)
            }
        }
    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_string_struct {
    ( $name:ident, $field:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        impl Clone for $name {
            fn clone(&self) -> Self{
                let $field = self.$field.clone();

                $name{$field}
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.$field))?;
                Ok(())
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(&self.$field)?;
                Ok(())
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.$field.eq(&other.$field)
            }

            fn ne(&self, other: &Self) -> bool {
                self.$field.ne(&other.$field)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self.$field.as_bytes()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.$field.as_ref()
            }
        }

        impl ::validators::Validated for $name {}

        impl ::validators::ValidatedWrapper for $name {
            type Error = ::validators::ValidatedCustomizedStringError;

            fn from_string($from_string_input: String) -> Result<Self, Self::Error>{
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> Result<Self, Self::Error>{
                $name::from_str($from_str_input)
            }
        }

        impl<'a> $name {
            fn as_str(&'a self) -> &'a str {
                &self.$field
            }

            fn from_string($from_string_input: String) -> Result<Self, ::validators::ValidatedCustomizedStringError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            fn from_str($from_str_input: &str) -> Result<Self, ::validators::ValidatedCustomizedStringError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }
        }

        validated_customized_string_struct_implement_from_form_value!($name);
    };
    ( $name:ident, $field:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string_struct!($name, $field, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $field:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string_struct!($name, $field, $from_string_input $from_string, $from_str_input $from_str);
    };
}

#[macro_export]
macro_rules! validated_customized_string {
    ( $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        struct $name{
            s: String
        }

        validated_customized_string_struct!($name, s, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string!($name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string!($name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        pub struct $name{
            s: String
        }

        validated_customized_string_struct!($name, s, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string!(pub $name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string!(pub $name, $from_string_input $from_string, $from_str_input $from_str);
    };
}

#[macro_export]
macro_rules! validated_customized_regex_string_struct {
    ( $name:ident, $field:ident, $re:expr ) => {
        validated_customized_string_struct!($name, $field,
        input {
            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedStringError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedStringError::NotMatch)
            }
        },
        input {
            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedStringError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.to_string())
            } else{
                Err(::validators::ValidatedCustomizedStringError::NotMatch)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_regex_string {
    ( $name:ident, $re:expr ) => {
        struct $name{
            s: String
        }

        validated_customized_regex_string_struct!($name, s, $re);
    };
    ( pub $name:ident, $re:expr ) => {
        pub struct $name{
            s: String
        }

        validated_customized_regex_string_struct!($name, s, $re);
    };
}

// TODO -----ValidatedCustomizedString END-----

// TODO -----ValidatedCustomizedNumber START-----

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedNumberError {
    RegexError(regex::Error),
    ParseError(String),
    OutRange,
    NotMatch,
    UTF8Error(Utf8Error),
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a> ::validators::rocket::request::FromFormValue<'a> for $name {
            type Error = ::validators::ValidatedCustomizedNumberError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedNumberError::UTF8Error(err))?)
            }
        }
    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_number_struct {
    ( $name:ident, $field:ident, $t:ty, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        impl Clone for $name {
            fn clone(&self) -> Self{
                let $field = self.$field;

                $name{$field}
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.$field))?;
                Ok(())
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}", self.$field))?;
                Ok(())
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.$field == other.$field
            }

            fn ne(&self, other: &Self) -> bool {
                self.$field != other.$field
            }
        }

        impl ::validators::Validated for $name {}

        impl ::validators::ValidatedWrapper for $name {
            type Error = ::validators::ValidatedCustomizedNumberError;

            fn from_string($from_string_input: String) -> Result<Self, Self::Error>{
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> Result<Self, Self::Error>{
                $name::from_str($from_str_input)
            }
        }

        impl $name {
            fn get_number(&self) -> $t {
                self.$field
            }

            fn from_string($from_string_input: String) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            fn from_str($from_str_input: &str) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }
        }

        validated_customized_number_struct_implement_from_form_value!($name);
    };
    ( $name:ident, $field:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
}

#[macro_export]
macro_rules! validated_customized_number {
    ( $name:ident, $t:ty, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        struct $name{
            n: $t
        }

        validated_customized_number_struct!($name, n, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_number_struct!($name, n, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number_struct {
    ( $name:ident, $field:ident, $t:ty, $re:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let input = input.to_string();

            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number {
    ( $name:ident, $t:ty, $re:expr ) => {
        struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
    ( pub $name:ident, $t:ty, $re:expr ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_number_struct {
    ( $name:ident, $field:ident, $t:ty, $min:expr, $max:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if input >= $min && input <= $max {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        },
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if input >= $min && input <= $max {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        },
        input {
            if input >= $min && input <= $max {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_number {
    ( $name:ident, $t:ty, $min:expr, $max:expr ) => {
        struct $name{
            n: $t
        }

        validated_customized_ranged_number_struct!($name, n, $t, $min, $max);
    };
    ( pub $name:ident, $t:ty, $min:expr, $max:expr ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_ranged_number_struct!($name, n, $t, $min, $max);
    };
}

#[macro_export]
macro_rules! validated_customized_primitive_number_struct {
    ( $name:ident, $field:ident, $t:ty ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            Ok(input)
        },
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            Ok(input)
        },
        input {
            Ok(input)
        });
    };
}

#[macro_export]
macro_rules! validated_customized_primitive_number {
    ( $name:ident, $t:ty ) => {
        struct $name{
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
    ( pub $name:ident, $t:ty ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
}

// TODO -----ValidatedCustomizedNumber END-----

// TODO -----ValidatedCustomizedRangedLengthVec START-----

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedVecError {
    Overflow,
    Underflow,
    NotSupport,
    UTF8Error(Utf8Error),
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a, T: ::validators::ValidatedWrapper> ::validators::rocket::request::FromFormValue<'a> for $name<T> {
            type Error = ::validators::ValidatedCustomizedVecError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedVecError::UTF8Error(err))?)
            }
        }
    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_vec_struct {
    ( $name:ident, $field:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        impl<T: ::validators::ValidatedWrapper> Clone for $name<T> {
            fn clone(&self) -> Self{
                let $field = self.$field.clone();

                $name{$field}
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}[", stringify!($name)))?;

                let len = self.$field.len();

                if len > 0 {
                    for n in self.$field.iter().skip(1) {
                        ::std::fmt::Debug::fmt(n, f)?;


                        f.write_str(", ")?;
                    }

                    ::std::fmt::Debug::fmt(&self.$field[len - 1], f)?;
                }

                f.write_str("]")?;

                Ok(())
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::std::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("[")?;

                let len = self.$field.len();

                if len > 0 {
                    for n in self.$field.iter().skip(1) {
                        ::std::fmt::Display::fmt(n, f)?;


                        f.write_str(", ")?;
                    }

                    ::std::fmt::Display::fmt(&self.$field[len - 1], f)?;
                }

                f.write_str("]")?;

                Ok(())
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::std::cmp::PartialEq for $name<T> {
            fn eq(&self, other: &Self) -> bool {
                self.$field == other.$field
            }

            fn ne(&self, other: &Self) -> bool {
                self.$field != other.$field
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::validators::Validated for $name<T> {}

        impl<T: ::validators::ValidatedWrapper> ::validators::ValidatedWrapper for $name<T> {
            type Error = ::validators::ValidatedCustomizedVecError;

            fn from_string($from_string_input: String) -> Result<Self, Self::Error>{
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> Result<Self, Self::Error>{
                $name::from_str($from_str_input)
            }
        }

        impl<T: ::validators::ValidatedWrapper> $name<T> {
            fn as_vec(&self) -> &Vec<T> {
                &self.$field
            }

            pub fn into_vec(self) -> Vec<T> {
                self.$field
            }

            fn from_string($from_string_input: String) -> Result<Self, ::validators::ValidatedCustomizedVecError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            fn from_str($from_str_input: &str) -> Result<Self, ::validators::ValidatedCustomizedVecError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            fn from_vec($from_vec_input: Vec<T>) -> Result<Self, ::validators::ValidatedCustomizedVecError>{
                let $field = match $from_vec {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }
        }

         validated_customized_vec_struct_implement_from_form_value!($name);
    };
}

#[macro_export]
macro_rules! validated_customized_vec {
    ( $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_vec_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        pub struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_vec_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_length_vec_struct {
    ( $name:ident, $field:expr, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_vec_struct!($name, v,
        $from_string_input $from_string,
        $from_str_input $from_str,
        input {
            let len = input.len();

            if len > $max {
                Err(::validators::ValidatedCustomizedVecError::Overflow)
            } else if len < $min {
                Err(::validators::ValidatedCustomizedVecError::Underflow)
            } else {
                Ok(input)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_length_vec {
    ( $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_ranged_length_vec_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!($name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_vec!($name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_vec!($name, $min, $max,
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)},
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)});
    };
    ( $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!($name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $equal:expr) => {
        validated_customized_ranged_length_vec!($name, $equal, $equal);
    };
    ( pub $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        pub struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_ranged_length_vec_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!(pub $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_vec!(pub $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_vec!(pub $name, $min, $max,
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)},
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)});
    };
    ( pub $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!(pub $name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $equal:expr) => {
        validated_customized_ranged_length_vec!(pub $name, $equal, $equal);
    };
}

// TODO -----ValidatedCustomizedRangedLengthVec End-----
