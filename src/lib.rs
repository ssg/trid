use std::convert::TryInto;
use std::str;
use std::str::FromStr;

const TURKISHID_LENGTH: usize = 11;

/// Turkish citizenship ID number.
#[derive(Eq, Debug)]
pub struct TurkishId {
    value: [u8; TURKISHID_LENGTH],
}

/// Represents the parser error for a given Turkish citizenship ID number.
#[derive(Debug, PartialEq)]
pub enum TurkishIdError {
    InvalidLength,
    InvalidChecksum,
}

impl TurkishId {
    /// Create a new `TurkishId` from a string.
    ///
    /// # Arguments
    ///
    /// * `value` - The string that contains the ID number.
    ///
    /// # Example
    /// ```rust
    /// use trid::TurkishId;
    ///
    /// let id = TurkishId::new("76558242278");
    pub fn new(value: &str) -> Self {
        if !is_valid(&value) {
            panic!("Invalid TurkishId");
        }
        Self::new_internal(&value)
    }

    fn new_internal(value: &str) -> Self {
        TurkishId {
            value: value
                .as_bytes()
                .try_into()
                .expect("Internal error: incorrect length"),
        }
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
    if digits.len() != TURKISHID_LENGTH {
        return false;
    }

    let mut invalid = false;
    let mut odd_sum = digit(digits[0], &mut invalid);
    if invalid || odd_sum == 0 {
        return false;
    }

    let mut even_sum = 0;
    for i in (1..9 as usize).step_by(2) {
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

impl PartialEq for TurkishId {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl ToString for TurkishId {
    fn to_string(&self) -> String {
        str::from_utf8(&self.value)
            .expect("Invalid value")
            .to_string()
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
    /// let id : Result<TurkishId, TurkishIdError> = "76558242278".parse();
    /// assert_eq!(id, Ok(trid::TurkishId::new("76558242278")));
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
        if s.len() != TURKISHID_LENGTH {
            return Err(TurkishIdError::InvalidLength);
        }

        if !is_valid(&s) {
            return Err(TurkishIdError::InvalidChecksum);
        }

        Ok(Self::new_internal(&s))
    }
}
