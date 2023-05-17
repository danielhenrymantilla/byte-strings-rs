# `::byte-strings`

Rust zero-cost byte strings manipulation, for a better and safer FFI

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/byte-strings-rs)
[![Latest version](https://img.shields.io/crates/v/byte-strings.svg)](
https://crates.io/crates/byte-strings)
[![Documentation](https://docs.rs/byte-strings/badge.svg)](
https://docs.rs/byte-strings)
[![MSRV](https://img.shields.io/badge/MSRV-1.65.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![License](https://img.shields.io/crates/l/byte-strings.svg)](
https://github.com/danielhenrymantilla/byte-strings-rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/byte-strings-rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/byte-strings-rs/actions)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

## Example

Featuring the `c_str!` macro to create **valid C string literals** with
literally no runtime cost!

```rust
#[macro_use]
extern crate byte_strings;

/// Some lib
mod safe {
    use ::std::{
        ffi::CStr,
        os::raw::{c_char, c_int},
    };

    /// private unsafe C FFI
    mod ffi {
        use super::*;

        extern "C" {
            pub
            fn puts (_: *const c_char)
              -> c_int
            ;
        }
    }

    /// lib API: safe Rust wrapper => uses `CStr`
    pub
    fn puts (message: &'_ CStr)
      -> i32
    {
        unsafe {
            ffi::puts(message.as_ptr()) as i32
        }
    }
}

fn main ()
{
    safe::puts(c!("Hello, World!"));
}
```
