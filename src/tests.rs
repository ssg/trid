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
