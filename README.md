# byte-strings-rs

Rust zero-cost byte strings manipulation, for a better and safer FFI

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)][Repository]
[![Latest version](https://img.shields.io/crates/v/byte-strings.svg)][crates.io]
[![Documentation](https://docs.rs/byte_strings/badge.svg)][Documentation]
[![License](https://img.shields.io/crates/l/byte-strings.svg)](https://github.com/danielhenrymantilla/byte-strings-rs#license)

## Example

Featuring the `c_str!` macro to create **valid C string literals** with literally no
runtime cost!

```rust
/// Some lib
mod safe {
    use ::std::{
        ffi::CStr,
        os::raw::{c_char, c_int},
    };

    /// private unsafe C FFI
    mod ffi { use super::*; extern "C" {
        pub fn puts (_: *const c_char) -> c_int;
    }}

    /// lib API: safe Rust wrapper => uses `CStr`
    pub fn puts (message: &'_ CStr) -> i32
    {
        unsafe {
            ffi::puts(message.as_ptr()) as i32
        }
    }
}

fn main ()
{
    use ::byte_strings::c_str;

    dbg!(safe::puts(
        c_str!(
            "Hello, ",
            "World!",
        ) // No runtime error, no runtime cost
    ));
}
```

[Repository]: https://github.com/danielhenrymantilla/byte-strings-rs
[Documentation]: https://docs.rs/byte_strings
[crates.io]: https://crates.io/crates/byte-strings
