use super::*;
use std::collections::HashSet;

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
    "10000000146",
];

const INVALID_NUMBERS: &[(&str, Error)] = &[
    ("04948892948", Error::InvalidDigit),    // first digit zero
    ("14948892946", Error::InvalidChecksum), // last checksum INVALID
    ("14948892937", Error::InvalidChecksum), // first checksum INVALID
    // non numeric chars
    ("A4948892948", Error::InvalidDigit),
    ("7B558242278", Error::InvalidDigit),
    ("80C76431508", Error::InvalidDigit),
    ("767D5508630", Error::InvalidDigit),
    ("9079E350894", Error::InvalidDigit),
    ("43473F24496", Error::InvalidDigit),
    ("566733G2584", Error::InvalidDigit),
    ("2926080H600", Error::InvalidDigit),
    ("93212606I04", Error::InvalidDigit),
    ("352014085J8", Error::InvalidDigit),
    ("3520140853K", Error::InvalidDigit),
    // spaces
    (" 7655824227", Error::InvalidDigit),
    ("5582422781 ", Error::InvalidDigit),
    // uneven length
    ("", Error::InvalidLength),
    ("7", Error::InvalidLength),
    ("76", Error::InvalidLength),
    ("76558", Error::InvalidLength),
    ("765582", Error::InvalidLength),
    ("7655824", Error::InvalidLength),
    ("76558242", Error::InvalidLength),
    ("765582422", Error::InvalidLength),
    ("7655824227", Error::InvalidLength),
    ("765582422781", Error::InvalidLength),
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

#[test]
fn hashset_compatible() {
    let mut set = HashSet::new();
    let num = VALID_NUMBERS[0].parse::<TurkishId>().unwrap();
    set.insert(num);
    let num2 = VALID_NUMBERS[0].parse::<TurkishId>().unwrap();
    set.insert(num2);
    assert_eq!(num2, num);
    assert_eq!(1, set.len());
}

#[test]
fn display_returnsthesamerepresentation() {
    for number in VALID_NUMBERS {
        let id: TurkishId = number.parse().unwrap();
        let idstr = format!("{id}");
        assert_eq!(idstr, *number);
    }
}
