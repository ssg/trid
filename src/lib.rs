use std::{
    convert::TryInto,
    error::Error,
    fmt::{Display, Formatter},
    str,
    str::FromStr,
};

pub const LENGTH: usize = 11;

type Bytes = [u8; LENGTH];

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

type Err = TurkishIdError;

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "invalid length"),
            Self::InvalidDigit => write!(f, "invalid digit"),
            Self::InvalidChecksum => write!(f, "invalid checksum"),
        }
    }
}

impl Error for Err {}

/// Checks if the given string is a valid Turkish citizenship ID number.
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
    validate(value.as_bytes()).is_ok()
}

fn next<T>(t: &mut impl Iterator<Item = Option<T>>) -> Result<T, Err> {
    t.next().flatten().ok_or(Err::InvalidDigit)
}

/// Internal function to validate a given Turkish ID number.
fn validate(bytes: &[u8]) -> Result<(), Err> {
    if bytes.len() != LENGTH {
        return Err(Err::InvalidLength);
    }

    let mut digits = bytes
        .iter()
        .map(|b| (*b as i32) - 48)
        .map(|i| (0..=9).contains(&i).then(|| i));
    let mut odd_sum = next(&mut digits)?;
    if odd_sum == 0 {
        return Err(Err::InvalidDigit);
    }
    let mut even_sum = 0i32;
    for _ in 0..4 {
        even_sum += next(&mut digits)?;
        odd_sum += next(&mut digits)?;
    }

    let first_checksum = next(&mut digits)?;
    let final_checksum = next(&mut digits)?;

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
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}

impl TryFrom<&Bytes> for TurkishId {
    type Error = Err;
    fn try_from(value: &Bytes) -> Result<Self, Self::Error> {
        validate(value)?;
        Ok(Self(value.clone()))
    }
}

impl TryFrom<&[u8]> for TurkishId {
    type Error = Err;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        validate(value)?;
        let val: Bytes = value.try_into().map_err(|_| Err::InvalidLength)?;
        Ok(Self(val))
    }
}

impl FromStr for TurkishId {
    type Err = TurkishIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        validate(bytes)?;
        Ok(bytes.into())
    }
}

#[cfg(test)]
mod tests;
