use std::str;
use std::str::FromStr;
use std::convert::{TryInto, TryFrom};

pub const TURKISHID_LENGTH : usize = 11;
pub type IdInner = [u8; TURKISHID_LENGTH];

/// Turkish citizenship ID number.
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub struct TurkishId {
    id: IdInner,
}

/// Represents the parser error for a given Turkish citizenship ID number.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TurkishIdError {
    InvalidLength,
    InvalidChecksum,
}

fn checksum(id: IdInner) -> bool {
    checksum_util(id).is_some()
}

fn checksum_util(id: IdInner) -> Option<()> {
    let mut digits = id.into_iter();

    let mut odd_sum = digit(digits.next().unwrap())?;
    if odd_sum == 0 {
        return None;
    }

    let mut even_sum = 0;
    for _ in 0..4 {
        even_sum += digit(digits.next().unwrap())?;
        odd_sum += digit(digits.next().unwrap())?;
    }

    let first_checksum = digit(digits.next().unwrap())?;
    let final_checksum = digit(digits.next().unwrap())?;

    let computed_final = (odd_sum + even_sum + first_checksum) % 10;
    if computed_final != final_checksum {
        return None;
    }

    let mut computed_first = ((odd_sum * 7) - even_sum) % 10;
    if computed_first < 0 {
        computed_first += 10;
    }

    if computed_first == first_checksum {
        Some(())
    } else {
        None
    }
}

fn digit(byte: u8) -> Option<i32> {
    let b = (byte as u8) - ('0' as u8);
    if (0..=9).contains(&b) {
        Some(b as _)
    } else {
        None
    }
}

impl std::fmt::Display for TurkishId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.id).unwrap())
    }
}

impl TryFrom<IdInner> for TurkishId {
    type Error = TurkishIdError;

    fn try_from(id: IdInner) -> Result<Self, Self::Error> { 
        if checksum(id) {
            Ok(TurkishId { id })
        } else {
            Err(TurkishIdError::InvalidChecksum)
        }
    }
}

impl TryFrom<&str> for TurkishId {
    type Error = TurkishIdError;

    fn try_from(s: &str) -> Result<Self, Self::Error> { 
        s.parse()
    }
}

impl FromStr for TurkishId {
    type Err = TurkishIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let array: IdInner = match s.as_bytes().try_into() {
            Ok(v) => v,
            Err(_) => return Err(TurkishIdError::InvalidLength),
        };

        Self::try_from(array)
   }
}