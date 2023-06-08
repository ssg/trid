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
//! fn test() -> Result<TurkishId, Error> {
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
    ops::Range,
    str::{self, FromStr},
};

pub const LENGTH: usize = 11;

pub(crate) type Bytes = [u8; LENGTH];

/// Turkish citizenship ID number. The number is stored as ASCII digits
/// "0".."9" in the structure.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TurkishId(Bytes);

/// Represents the parser error for a given Turkish citizenship ID number.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidLength,
    InvalidDigit,
    InvalidChecksum,
}

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
fn validate(str: &str) -> Result<(), Error> {
    /// Flattens and handles the error together
    fn next_digit<T>(t: &mut impl Iterator<Item = Option<T>>) -> Result<T, Error> {
        t.next().flatten().ok_or(Error::InvalidDigit)
    }

    if str.len() != LENGTH {
        return Err(Error::InvalidLength);
    }

    // get a digit iterator
    let mut digits = str
        .chars()
        .map(|c| c.to_digit(10).and_then(|u| i32::try_from(u).ok()));

    // start calculating checksums
    let mut odd_sum = next_digit(&mut digits)?;
    if odd_sum == 0 {
        // the first digit cannot be zero
        return Err(Error::InvalidDigit);
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
        return Err(Error::InvalidChecksum);
    }

    // we use euclidian remainder due to the possibility that the final
    // checksum wmight be negative.
    let first_checksum_computed = ((odd_sum * 7) - even_sum).rem_euclid(10);
    if first_checksum_computed != first_checksum {
        return Err(Error::InvalidChecksum);
    }

    Ok(())
}

/// TurkishId types are displayed as regular numbers.
impl Display for TurkishId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            str::from_utf8(&self.0).map_err(|_| core::fmt::Error::default())?
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum FromSeqError {
    OutOfRange,
}

impl TurkishId {
    pub const SEQ_RANGE: Range<u32> = 100_000_000..1_000_000_000;

    /// Generate a valid TurkishId from a sequence number by calculating
    /// checksums and building the buffer for it.
    ///
    /// # Arguments
    /// - seq - A number between 100,000,000 and 999,999,999
    ///
    /// # Returns
    /// A Result with `TurkishId` if the values are in range, otherwise
    /// `FromSeqError`
    pub fn from_seq(seq: u32) -> Result<TurkishId, FromSeqError> {
        fn to_ascii(digit: i32) -> u8 {
            digit as u8 + b'0'
        }
        if !TurkishId::SEQ_RANGE.contains(&seq) {
            return Err(FromSeqError::OutOfRange);
        }
        let mut d: Bytes = [0; LENGTH];
        let mut odd_sum: i32 = 0;
        let mut even_sum: i32 = 0;
        let mut divisor = TurkishId::SEQ_RANGE.start;
        for (i, item) in d.iter_mut().enumerate().take(9) {
            let digit = (seq / divisor % 10) as i32;
            if i % 2 == 0 {
                odd_sum += digit;
            } else {
                even_sum += digit;
            }
            *item = to_ascii(digit);
            divisor /= 10;
        }
        let first_checksum = ((odd_sum * 7) - even_sum).rem_euclid(10);
        let second_checksum = (odd_sum + even_sum + first_checksum) % 10;
        d[9] = to_ascii(first_checksum);
        d[10] = to_ascii(second_checksum);
        Ok(TurkishId(d))
    }
}

/// TurkishId can only be constructed from a string despite that it's stored
/// as a fixed-length byte array internally.
impl FromStr for TurkishId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate(s)?;
        let result = Self(s.as_bytes().try_into().map_err(|_| Error::InvalidLength)?);
        Ok(result)
    }
}

#[cfg(test)]
mod tests;
