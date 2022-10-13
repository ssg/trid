//! Provides a `TurkishId` data type that holds a valid Turkish citizenship ID number.
//! ID numbers can also be directly validated on `&str`'s by using `is_valid` function.
//!
//! # Examples
//!
//! Validate a Turkish citizenship ID number:
//!
//! ```rust
//! if trid::is_valid("12345678901") {
//!     // yayyy!
//! }
//! ```
//!
//! Try parsing a string into `TurkishId`:
//!
//! ```rust
//! use trid::*;
//!
//! fn test() -> Result<TurkishId, TurkishIdError> {
//!     let id : TurkishId = "12345678901".parse()?;
//!     Ok(id)
//! }
//! ```
//!
//! # License
//!
//! Apache License Version 2.0, see LICENSE file for details.

#![cfg_attr(not(test), no_std)]

use core::{
    convert::TryInto,
    fmt::{Display, Formatter},
    str::{self, FromStr},
};

pub const LENGTH: usize = 11;

pub(crate) type Bytes = [u8; LENGTH];

/// Turkish citizenship ID number.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TurkishId(Bytes);

/// Represents the parser error for a given Turkish citizenship ID number.
#[derive(Debug, Eq, PartialEq)]
pub enum TurkishIdError {
    InvalidLength,
    InvalidDigit,
    InvalidChecksum,
}

/// Internal alias for `TurkishIdError`.
pub(crate) type Err = TurkishIdError;

/// Checks if the given string slice is a valid Turkish citizenship ID number.
///
/// # Arguments
///
/// * `value` - The string to check.
///
/// # Returns
///
/// `true` if the string is a valid Turkish ID number, `false` otherwise.
///
/// # Example
/// ```
/// use trid;
///
/// assert!(trid::is_valid("76558242278"));
/// ```
///
/// ```
/// use trid;
///
/// assert!(!trid::is_valid("06558242278"));
/// ```
pub fn is_valid(value: &str) -> bool {
    validate(value).is_ok()
}

/// Internal function to validate a given Turkish ID number.
fn validate(str: &str) -> Result<(), Err> {
    /// Flattens and handles the error together
    fn next_digit<T>(t: &mut impl Iterator<Item = Option<T>>) -> Result<T, Err> {
        t.next().flatten().ok_or(Err::InvalidDigit)
    }

    if str.len() != LENGTH {
        return Err(Err::InvalidLength);
    }

    // get a digit iterator
    let mut digits = str.chars()
        .map(|c| c.to_digit(10))
        .map(|c| c.and_then(|c| i32::try_from(c).ok()));

    // start calculating checksums
    let mut odd_sum = next_digit(&mut digits)?;
    if odd_sum == 0 {
        // the first digit cannot be zero
        return Err(Err::InvalidDigit);
    }
    let mut even_sum = 0;
    for _ in 0..4 {
        even_sum += next_digit(&mut digits)?;
        odd_sum += next_digit(&mut digits)?;
    }

    let first_checksum = next_digit(&mut digits)?;
    let final_checksum = next_digit(&mut digits)?;

    // we check for the final checksum first because it's computationally 
    // cheaper.
    let final_checksum_computed = (odd_sum + even_sum + first_checksum) % 10;
    if final_checksum_computed != final_checksum {
        return Err(Err::InvalidChecksum);
    }

    let first_checksum_computed = ((odd_sum * 7) - even_sum).rem_euclid(10);
    if first_checksum_computed != first_checksum {
        return Err(Err::InvalidChecksum);
    }

    Ok(())
}

impl Display for TurkishId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            str::from_utf8(&self.0).map_err(|_| core::fmt::Error::default())?
        )
    }
}

impl FromStr for TurkishId {
    type Err = Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate(s)?;
        let result = Self(s.as_bytes().try_into().map_err(|_| Err::InvalidLength)?);
        Ok(result)
    }
}

#[cfg(test)]
mod tests;
