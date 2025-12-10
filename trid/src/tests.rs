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
    ("04948892948", Error::FirstDigitIsZero), // first digit zero
    ("14948892946", Error::InvalidFinalChecksum), // last checksum INVALID
    ("14948892937", Error::InvalidInitialChecksum), // first checksum INVALID
    // non numeric chars
    ("A4948892948", Error::InvalidCharacter('A')),
    ("7B558242278", Error::InvalidCharacter('B')),
    ("80C76431508", Error::InvalidCharacter('C')),
    ("767D5508630", Error::InvalidCharacter('D')),
    ("9079E350894", Error::InvalidCharacter('E')),
    ("43473F24496", Error::InvalidCharacter('F')),
    ("566733G2584", Error::InvalidCharacter('G')),
    ("2926080H600", Error::InvalidCharacter('H')),
    ("93212606I04", Error::InvalidCharacter('I')),
    ("352014085J8", Error::InvalidCharacter('J')),
    ("3520140853K", Error::InvalidCharacter('K')),
    // spaces
    (" 7655824227", Error::InvalidCharacter(' ')),
    ("5582422781 ", Error::InvalidCharacter(' ')),
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

const OUT_OF_RANGE_SEQUENCES: &[u32] = &[0, 99_999_999, 1_000_000_001, u32::MAX];

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
        let id = TurkishId::from_str(number).unwrap();
        let idstr = format!("{id}");
        assert_eq!(idstr, *number);
    }
}

#[test]
fn from_seq_produces_valid_numbers() {
    for number in VALID_NUMBERS {
        let seq: u32 = number[..9].parse().unwrap();
        let id = TurkishId::from_seq(seq).unwrap();
        assert_eq!(*number, id.to_string());
    }
}

#[test]
fn from_seq_out_of_range_values_return_error() {
    for seq in OUT_OF_RANGE_SEQUENCES {
        let result = TurkishId::from_seq(*seq);
        assert_eq!(result.err(), Some(FromSeqError::OutOfRange));
    }
}
