#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
)]
// Fix rendering of `<details><summary>` within bulleted lists:
// Credit for this marvelous hack go to: https://github.com/rust-lang/cargo/issues/331#issuecomment-479847157
#![doc(html_favicon_url = "\">
<style>
summary {
    display: list-item;
}
</style>
<meta name=\"")]

pub use concat_bytes as as_bytes;

/// Concatenates (byte) string literals into a single byte string literal.
///
/// This macro takes any number of comma-separated (byte) string literals,
/// and evaluates to (a static reference to) a byte array made of all the
/// bytes of the given byte string literals concatenated left-to-right.
///
/// Hence the macro evaluates to the type `&'static [u8; N]`
/// (where N is the total number of bytes), which can also "be seen as"
/// (coerce to) a static byte slice (_i.e.,_ `&'static [u8]`).
///
/// # Example
///
/// ```rust,edition2018
/// use ::byte_strings::concat_bytes;
///
/// let bytes = concat_bytes!(b"Hello, ", b"World!");
/// assert_eq!(bytes, b"Hello, World!");
/// ```
///
/// ### Macro expansion:
///
/// `concat_bytes!(b"Hello, ", b"World!")`
/// expands to
/// `b"Hello, World!"`
#[macro_export]
macro_rules! concat_bytes {(
    $($expr:expr),* $(,)?
) => (
    $crate::__::concat_bytes!(
        [$crate]
        $($expr),*
    )
)}

/// Converts into a valid [C string] at compile time (no runtime cost)
///
/// This macro takes any number of comma-separated byte string literals,
/// or string literals,
/// and evaluates to (a static reference to) a [C string]
/// made of all the bytes of the given literals concatenated left-to-right,
/// **with an appended null byte terminator**.
///
/// Hence the macro evaluates to the type `&'static ::std::ffi::CStr`.
///
/// # Example
///
/// ```rust,edition2018
/// use ::byte_strings::c_str;
///
/// assert_eq!(
///     c_str!("Hello, ", "World!"),
///     ::std::ffi::CStr::from_bytes_with_nul(b"Hello, World!\0").unwrap(),
/// )
/// ```
///
/// # Compilation error
///
/// For the [C string] to be what should be expected,
/// **the arguments cannot contain any null byte**.
/// Else the compilation will fail.
///
/// # Counter example
///
/// ```rust,compile_fail
/// # use ::byte_strings::c_str;
/// // error: input literals cannot contain null bytes
/// let hello_w = c_str!("Hello, ", "W\0rld!");
/// ```
///
/// ### Macro expansion:
///
/// ```rust
/// const _: &str = stringify! {
/// c_str!("Hello, ", "World!")
/// # };
/// ```
///
/// expands to
///
/// ```rust
/// unsafe {
///     ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"Hello, World!\0")
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! c_str {(
    $($literal:expr),* $(,)?
) => (
    $crate::__::c_str!(
        [$crate]
        $($literal),*
    )
)}

/// Shorthand alias.
pub use c_str as c;

// macro internals
#[doc(hidden)] /** Not part of the public API */ pub
mod __ {
    pub use {
        ::byte_strings_proc_macros::*,
        ::core,
        ::std,
    };
}

#[cfg(feature = "const-friendly")]
#[path = "const.rs"]
pub mod const_;

#[cfg_attr(feature = "ui-tests",
    cfg_attr(all(), doc = include_str!("compile_fail_tests.md")),
)]
mod _compile_fail_tests {}
