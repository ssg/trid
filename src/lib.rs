use std::{
    convert::TryInto,
    fmt::{Display, Formatter},
    str,
    str::FromStr,
};

pub const LENGTH: usize = 11;

/// Turkish citizenship ID number.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct TurkishId([u8; LENGTH]);

/// Represents the parser error for a given Turkish citizenship ID number.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TurkishIdError {
    InvalidLength,
    InvalidChecksum,
}

impl TurkishId {
    fn new(value: &str) -> Self {
        TurkishId(
            value
                .as_bytes()
                .try_into()
                .expect("Internal error: incorrect length"),
        )
    }
}

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
    let digits = value.as_bytes();
    if digits.len() != LENGTH {
        return false;
    }

    let mut invalid = false;
    let mut odd_sum = digit(digits[0], &mut invalid);
    if invalid || odd_sum == 0 {
        return false;
    }

    let mut even_sum = 0;
    for i in (1..9).step_by(2) {
        even_sum += digit(digits[i], &mut invalid);
        odd_sum += digit(digits[i + 1], &mut invalid);
    }

    let first_checksum = digit(digits[9], &mut invalid);
    let final_checksum = digit(digits[10], &mut invalid);
    if invalid {
        return false;
    }

    let computed_final = (odd_sum + even_sum + first_checksum) % 10;
    if computed_final != final_checksum {
        return false;
    }

    let mut computed_first = ((odd_sum * 7) - even_sum) % 10;
    if computed_first < 0 {
        computed_first += 10;
    }

    computed_first == first_checksum
}

fn digit(byte: u8, invalid: &mut bool) -> i32 {
    let b = (byte as i32) - ('0' as i32);
    if b > 9 {
        *invalid |= true;
        return -1;
    }
    b
}

impl Display for TurkishId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}

impl From<&[u8; LENGTH]> for TurkishId {
    fn from(value: &[u8; LENGTH]) -> Self {
        TurkishId(value.to_owned())
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
        if s.len() != LENGTH {
            return Err(TurkishIdError::InvalidLength);
        }

        if !is_valid(s) {
            return Err(TurkishIdError::InvalidChecksum);
        }

        Ok(Self::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_NUMBERS: &[&str] = &[
        "19191919190",
        "76558242278",
        "80476431508",
        "76735508630",
        "90794350894",
        "43473624496",
        "10000000146",
        "56673392584",
        "29260807600",
        "93212606504",
        "35201408508",
        "64404737702",
    ];

    const INVALID_NUMBERS: &[(&str, Err)] = &[
        ("04948892948", Err::InvalidDigit),    // first digit zero
        ("14948892946", Err::InvalidChecksum), // last checksum INVALID
        ("14948892937", Err::InvalidChecksum), // first checksum INVALID
        // non numeric chars
        ("A4948892948", Err::InvalidDigit),
        ("7B558242278", Err::InvalidDigit),
        ("80C76431508", Err::InvalidDigit),
        ("767D5508630", Err::InvalidDigit),
        ("9079E350894", Err::InvalidDigit),
        ("43473F24496", Err::InvalidDigit),
        ("566733G2584", Err::InvalidDigit),
        ("2926080H600", Err::InvalidDigit),
        ("93212606I04", Err::InvalidDigit),
        ("352014085J8", Err::InvalidDigit),
        ("3520140853K", Err::InvalidDigit),
        // spaces
        (" 7655824227", Err::InvalidDigit),
        ("5582422781 ", Err::InvalidDigit),
        // uneven length
        ("7", Err::InvalidLength),
        ("76", Err::InvalidLength),
        ("76558", Err::InvalidLength),
        ("765582", Err::InvalidLength),
        ("7655824", Err::InvalidLength),
        ("76558242", Err::InvalidLength),
        ("765582422", Err::InvalidLength),
        ("7655824227", Err::InvalidLength),
        ("765582422781", Err::InvalidLength),
    ];

    #[test]
    fn is_valid_validnumbers_returns_true() {
        for number in VALID_NUMBERS {
            assert!(is_valid(number));
        }
    }

    #[test]
    fn is_valid_invalidnumbers_returns_false() {
        for (number, _) in INVALID_NUMBERS {
            assert!(!is_valid(number));
        }
    }

    #[test]
    fn parse_invalidnumbers_returns_correct_error() {
        for (number, error) in INVALID_NUMBERS {
            assert_eq!(*error, number.parse::<TurkishId>().err().unwrap());
        }
    }
}
