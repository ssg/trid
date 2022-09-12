# Changes

## 3.2.0

### Improvements

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
