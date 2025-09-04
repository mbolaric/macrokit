# Macrokit ⚙️

A collection of procedural derive macros for Rust.

[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/mbolaric/binary-data/actions/workflows/rust.yml/badge.svg?branch=master
[actions-url]: https://github.com/mbolaric/binary-data/actions/workflows/rust.yml?query=branch%3Amaster

## Features

`macrokit` currently provides two derive macros:

### 1. `#[derive(FromReprAsOption)]`

This is the recommended macro for safe, fallible conversions. It generates a `from_repr(value: T) -> Option<Self>` function, where `T` is the integer type from your `#[repr(T)]` attribute.

- **Safe:** Returns an `Option<Self>`, forcing you to handle cases where the integer value does not match any enum variant.
- **Flexible:** Does not require any specific fallback variant like `Unknown`.

### 2. `#[derive(FromReprWithUnknown)]`

This macro is for infallible conversions. It implements the standard `From<T>` trait, allowing you to use `.into()` and `From::from()` where `T` is the integer type from your `#[repr(T)]` attribute..

- **Convenient:** Provides ergonomic, infallible conversions.
- **Requires a Fallback:** Your enum **must** have a variant named `Unknown` which will be used if the integer value does not match any other variant.

## Usage

First, add `macrokit` to your project's `Cargo.toml`.

```toml
[dependencies]
macrokit = { path = "path/to/macrokit" }
```

### Example: `FromReprAsOption`

Use this when you want to handle invalid integer values explicitly.

```rust
use macrokit::FromReprAsOption;

#[derive(Debug, PartialEq, FromReprAsOption)]
#[repr(u8)]
pub enum Command {
    Read = 1,
    Write = 2,
}

assert_eq!(Command::from_repr(1), Some(Command::Read));
assert_eq!(Command::from_repr(3), None);
```

### Example: `FromReprWithUnknown`

Use this when you have a clear fallback value and want the convenience of the `From` trait.

```rust
use macrokit::FromReprWithUnknown;

#[derive(Debug, PartialEq, FromReprWithUnknown)]
#[repr(u8)]
pub enum Status {
    Active = 0,
    Inactive = 1,
    Unknown, // This variant is required
}

let status: Status = 1u8.into();
assert_eq!(status, Status::Inactive);

let unknown_status: Status = 99u8.into();
assert_eq!(unknown_status, Status::Unknown);
```
