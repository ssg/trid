[![Build & Test](https://github.com/ssg/trid/actions/workflows/rust.yml/badge.svg)](https://github.com/ssg/trid/actions/workflows/rust.yml)

# trid - Turkish Citizenship ID Number crate
This is my first ever written Rust code derived from my own [TurkishId](https://github.com/ssg/TurkishId) package for .NET. I'm trying to use existing code as an excuse to learn about Rust. Despite constant tackling with 
error messages, Rust has been extremely impressive so far. (How cool are doc-tests!?)

# Usage
The package provides an `is_valid(value: &str)` function for validating ID numbers and a `TurkishId` 
struct to encompass a Turkish citizenship ID number, so you don't need to validate it constantly.

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

let turkish_id : TurkishId = "12345678901".parse()?;
```

# License
Apache License Version 2.0, see LICENSE file for details.
