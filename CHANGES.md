# Changes

## 6.0.0

### Breaking changes

- `Error::InvalidDigit` is now `Error::InvalidCharacter(c)` which contains
  the character that's invalid.

- Invalid checksum errors are now split into `Error::InvalidFinalChecksum`
  and `Error::InvalidInitialChecksum`.

- `is_valid` is now marked with `#[must_use]`.

- The contents of TurkishId are now completely opaque to be able to keep the
  possibility to change the internal storage structure in the future. 
  To see its contents, use `to_string()` or a formatter.

## 5.0.0

### Breaking changes

- `TurkishIdError` is renamed to `Error` to conform to Rust semantics. Perhaps,
  I shouldn't have been so hesitant to make this package 1.0.0, huh :)
  
- Added `from_seq()` method that can generate a valid TurkishId from a given
  sequence number.

## 4.0.0

### Breaking changes

- Removed `TryFrom` impls for `&Bytes` and `&[u8]` types. The only conversion
  is possible from `&str` now. Changed the validation to use `&str` instead of
  `&[u8]`. The main reason I made this change is that it makes no sense to
  convert from a ASCII-encoded `&[u8]` not have it as `&str`. The conversion
  can alerady be done with `from_utf8()`, no need to repeat it there.

- Removed `Display` impl from `TurkishIdError` completely as deriving `Debug` already does it.
  This might mean that any code that relies on the fmt output of `TurkishIdError` might break.

## 3.1.1

### Fixes

- Fixed build break caused by inline source comments.

## 3.1.0

### Improvements

- Avoid panic in `Display` impl completely. The code is completely panic-free now.

## 3.0.0

### New features

- Now works without std (`no_std`-enabled), so feel free to use this in your favorite microcontroller :)

### Breaking changes

- Due to `no_std`, `Error` trait isn't supported by `TurkishIdError` anymore

## 2.0.0

### Breaking changes

- Implements `TryFrom` instead of `From`

### Improvements

- Faster validation

## 1.0.0

First stable release
