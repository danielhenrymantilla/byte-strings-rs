# byte-strings-rs

Rust byte strings manipulation, for a better and safer C FFI

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)][Repository]
[![Latest version](https://img.shields.io/crates/v/byte-strings.svg)][crates.io]
[![Documentation](https://docs.rs/byte_strings/badge.svg)][Documentation]
[![License](https://img.shields.io/crates/l/byte-strings.svg)](https://github.com/danielhenrymantilla/byte-strings-rs#license)

## Example

Featuring the `c_str!` macro to create **valid C string literals** with no
runtime cost!

```rust
mod puts {
    use ::std::{
        ffi::CStr,
        os::raw::{c_char, c_int},
    };

    /// C FFI
    extern "C" { fn puts (message: *const c_char) -> c_int; }

    /// Safe wrapper around C FFI
    pub fn safe (message: &'_ CStr) -> i32
    {
        unsafe {
            puts(message.as_ptr()) as i32
        }
    }
}
use self::puts::safe as safe_puts;

fn main ()
{
    use ::byte_strings::c_str;

    safe_puts(
        c_str!( // Simple and safe!
            "Hello, ",
            "World!",
        ) // No runtime error nor runtime cost!
    );
}
```

[Repository]: https://github.com/danielhenrymantilla/byte-strings-rs
[Documentation]: https://docs.rs/byte_strings/0.1.0/
[crates.io]: https://crates.io/crates/byte-strings
