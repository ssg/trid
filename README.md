[![Crates.io](https://img.shields.io/crates/v/trid)](https://crates.io/crates/trid)
[![Build & Test](https://github.com/ssg/trid/actions/workflows/rust.yml/badge.svg)](https://github.com/ssg/trid/actions/workflows/rust.yml)

# trid - Turkish Citizenship ID Number crate

This is my first ever written Rust code derived from my own [TurkishId](https://github.com/ssg/TurkishId)
package for .NET. I'm trying to use existing code as an excuse to learn about Rust. Despite constant tackling
with error messages, Rust has been extremely impressive so far. (How cool are doc-tests!?)

# Usage

The crate provides `TurkishId` type that represents a valid Turkish ID number. It can be instantiated
from a string using its `parse()` method, or directly converted from a `u8` slice using `from()` or
`try_from()` methods. The type guarantess that it never contains an invalid number, so there's no need
to validate a `TurkishId` type.

The crate also provides `is_valid(value: &str)` function for validating ID numbers.

# Examples

Validate a Turkish citizenship ID number:

```rust
if trid::is_valid("12345678901") {
    // yayyy!
}
```

Try parsing a string into `TurkishId`:

```rust
use trid::TurkishId;

let id : TurkishId = "12345678901".parse()?;
```

# License

Apache License Version 2.0, see LICENSE file for details.
