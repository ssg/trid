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
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
    validate(value).is_ok()
}

fn next<T>(t: &mut impl Iterator<Item = Option<T>>) -> Result<T, Err> {
    t.next().flatten().ok_or(Err::InvalidDigit)
}

/// Internal function to validate a given Turkish ID number.
fn validate(s: &str) -> Result<(), Err> {
    if s.len() != LENGTH {
        return Err(Err::InvalidLength);
    }

    let mut digits = s
        .chars()
        .map(|c| c.to_digit(10).and_then(|u| i32::try_from(u).ok()));
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

    let computed_final = (odd_sum + even_sum + first_checksum) % 10;
    if computed_final != final_checksum {
        return Err(Err::InvalidChecksum);
    }

    let mut computed_first = ((odd_sum * 7) - even_sum) % 10;
    if computed_first < 0 {
        computed_first += 10;
    }

    if computed_first != first_checksum {
        return Err(Err::InvalidChecksum);
    }

    Ok(())
}

impl Display for TurkishId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}

impl From<&Bytes> for TurkishId {
    fn from(value: &Bytes) -> Self {
        TurkishId(value.to_owned())
    }
}

impl From<&[u8]> for TurkishId {
    fn from(value: &[u8]) -> Self {
        TurkishId(value.try_into().expect("Invalid borrow passed"))
    }
}

impl FromStr for TurkishId {
    type Err = TurkishIdError;

    /// Returns `TurkishId` from a string if it's valid, otherwise returns `TurkishIdError`.
    ///
    /// # Arguments
    ///
    /// * `value` - The string that contains the ID number.
    ///
    /// # Returns
    ///
    /// * `Ok(TurkishId)` - If the ID number is valid.
    /// * `Err(TurkishIdError)` - If the ID number is invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use trid::TurkishId;
    /// use trid::TurkishIdError;
    ///
    /// let id : Result<TurkishId, TurkishIdError> = "19191919190".parse();
    /// assert_eq!(id, Ok("19191919190".parse::<TurkishId>().unwrap()));
    /// ```
    ///
    /// ```
    /// use trid::TurkishId;
    /// use trid::TurkishIdError;
    ///
    /// let result : Result<TurkishId, TurkishIdError> = "6558242278".parse();
    /// assert_eq!(result, Err(trid::TurkishIdError::InvalidLength));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate(s)?;
        Ok(s.as_bytes()
            .try_into()
            .expect("Internal error: validation should never fail"))
    }
}

#[cfg(test)]
mod tests;
