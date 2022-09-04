# Changes

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
