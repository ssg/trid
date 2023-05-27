[![Crates.io](https://img.shields.io/crates/v/trid)](https://crates.io/crates/trid)
[![Build & Test](https://github.com/ssg/trid/actions/workflows/rust.yml/badge.svg)](https://github.com/ssg/trid/actions/workflows/rust.yml)

# trid - Turkish Citizenship ID Number crate

This is my first ever written Rust code derived from my own [TurkishId](https://github.com/ssg/TurkishId)
package for .NET. I'm trying to use existing code as an excuse to learn about Rust. Despite constant tackling with error messages, Rust has been extremely impressive so far. (How cool are doc-tests!?)

# Usage

## parse

The crate provides `TurkishId` type that represents a valid Turkish ID number. It can be instantiated from a string using the `parse()` method of `str` type. `TurkishId` type guarantess that it never contains an invalid Turkish ID number, so there's no need to validate a `TurkishId` once parsed. It can always be passed around safely.

## is_valid

You can just verify whether a string contains a valid Turkish ID or not by calling `is_valid(value: &str)` function.

## from_seq

If you want to generate a Turkish ID from scratch, you can use `from_seq(seq: u32)` function.

# Internals

The type occupies 11 bytes in memory and kept as ASCII representation of the number in order
to make string display conversions fast. The other alternative would be to have it in a 40-bit number which would complicate the string representation.

# Examples

Validate a Turkish citizenship ID number:

```rust
fn main() {
    if trid::is_valid("12345678901") {
        // yayyy!
    }
}
```

Try parsing a string into `TurkishId`:

```rust
use trid::TurkishId;

fn main() {
    let id : TurkishId = "12345678901".parse()?;
}
```

Generate infinite number of random but valid Turkish IDs:

```rust
use rand::Rng;
use trid::TurkishId;

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let seq: u32 = rng.gen_range(100_000_000..1_000_000_000);
        println!("{}", TurkishId::from_seq(seq).unwrap());
    }
}
```

# License

Apache License Version 2.0, see LICENSE file for details.
