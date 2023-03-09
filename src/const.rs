/*! `const`-friendly equivalents of the top-level crates. This requires the `const-friendly` crate feature to be enabled.

<details><summary><code>const</code>-friendly?</summary>

The default / naïve implementation of the macros of this crate (ab)used the
proc-macro capabilities to inspect the contents / values of a given (byte)
string literal. But besides that capability, (procedural) macros can't do any
other form of semantic evaluation.

This means that when fed a `const`ant expression evaluating to a valid (byte)
string literal,

  - such as a `const`:

    ```rust ,compile_fail
    #[macro_use]
    extern crate byte_strings;

    fn main ()
    {
        const FOO: &str = "foo";

        // FAILS with something along the lines of "expected a literal"
        const FOO_BYTES: &[u8] = as_bytes!(FOO);
    }
    ```

  - or some macro <sub><sup>(besides `concat!`, `stringify!`, which are
    *syntactically* detected and thus get to feature ad-hoc polyfilled
    support)</sup></sub>:

    ```rust ,compile_fail
    #[macro_use]
    extern crate byte_strings;

    # mod third_party_lib { pub(in super) use stringify as their_macro; }
    #
    fn main ()
    {
        // FAILS with something along the lines of "expected a literal"
        let _ = as_bytes!(third_party_lib::their_macro!(...));
    }
    ```

</details>

The macros of this module have been written to use `const fn`s as much as
possible, so as to use the **semantic evaluation built within the compiler**
rather than the limited syntactical evaluation of classic macros. This allows
the macros in this module to be able to support any kind of `const`-evaluatable
(byte) string:

```rust
use ::byte_strings::const_::{c_str, CStr};

const MESSAGE: &str = "Hello, World!";
const C_MESSAGE: &CStr = c_str!(MESSAGE); // OK!
```
*/

#[doc(hidden)] /** Not part of the public API **/ pub
mod __ {
    #![allow(nonstandard_style)]
    pub use ::core;

    pub
    struct const_<T> /* = */ (
        pub T,
    );

    impl<'lt> const_<&'lt str> {
        pub
        const
        fn as_bytes (self)
          -> &'lt [u8]
        {
            self.0.as_bytes()
        }
    }

    impl<'lt> const_<&'lt [u8]> {
        pub
        const
        fn as_bytes (self)
          -> &'lt [u8]
        {
            self.0
        }
    }

    impl<'lt, const N: usize> const_<&'lt [u8; N]> {
        pub
        const
        fn as_bytes (self)
          -> &'lt [u8]
        {
            self.0
        }
    }

    #[repr(C)]
    pub
    struct Contiguous<_0, _1> /* = */ (
        pub _0,
        pub _1,
    );

    pub
    const
    fn c_strlen (bytes: &[u8])
      -> usize
    {
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'\0' {
                break;
            }
            i += 1;
        }
        i
    }

    pub
    struct c_strlen<const N: usize> {}
}

/// [`const`-friendly][crate::const_] version of [`as_bytes!`][crate::as_bytes].
///
/// ```rust
/// # fn main () {}
/// #[macro_use]
/// extern crate byte_strings;
///
/// const MESSAGE: &str = "Hello, World!";
/// const MESSAGE_BYTES: &[u8] = const_as_bytes!(MESSAGE);
/// ```
#[macro_export]
macro_rules! const_as_bytes {( $s:expr $(,)? ) => ({
    const __BYTES: &'static [u8] = $crate::const_::__::const_($s).as_bytes();
    unsafe {
        $crate::__::core::mem::transmute::<
            *const u8,
            &'static [u8; __BYTES.len()],
        >(
            __BYTES.as_ptr()
        )
    }
})}
#[doc(inline)] pub use const_as_bytes as as_bytes;

/// [`const`-friendly][crate::const_] version of [`concat!`][::core::concat].
///
/// ```rust
/// # fn main () {}
/// #[macro_use]
/// extern crate byte_strings;
///
/// const GREETING: &str = "Hello";
/// const MESSAGE: &str = const_concat!(GREETING, ", World!");
/// ```
#[macro_export]
macro_rules! const_concat {(
    $($s:expr),* $(,)?
) => (
    unsafe {
        $crate::__::core::str::from_utf8_unchecked(
            $crate::const_concat_bytes!(
                $(
                    <$crate::__::core::primitive::str>::as_bytes($s)
                ),*
            )
        )
    }
)}
#[doc(inline)] pub use const_concat as concat;

/// `const`-constructible equivalent of [`::std::ffi::CStr`].
#[repr(transparent)]
pub struct CStr([u8]);

impl CStr {
    /// # Safety
    ///
    /// Same requirements as [`CStr::from_bytes_with_nul_unchecked()`][1]
    ///
    /// [1]: `::std::ffi::CStr::from_bytes_with_nul_unchecked`
    pub
    const
    unsafe
    fn from_bytes_with_nul_unchecked (bytes: &'_ [u8])
      -> &'_ CStr
    {
        // Safety: `#[repr(transparent)]` ensures same layout.
        ::core::mem::transmute(bytes)
    }
}

impl ::core::ops::Deref for CStr {
    type Target = ::std::ffi::CStr;

    #[inline]
    fn deref (self: &'_ CStr)
      -> &'_ ::std::ffi::CStr
    {
        unsafe {
            // Safety:
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(&self.0)
        }
    }
}

/// [`const`-friendly][crate::const_] version of
/// [`c_str!`][crate::c_str].
///
/// ```rust
/// use ::byte_strings::const_;
///
/// const MESSAGE: &str = "Hello, World!";
/// const C_MESSAGE: &const_::CStr = const_::c_str!(MESSAGE);
/// ```
///
/// Inner null bytes are still rejected at compile time:
///
/// ```rust ,compile_fail
/// use ::byte_strings::const_;
///
/// const MESSAGE: &str = "Hell\0, World!";
/// const C_MESSAGE: &const_::CStr = const_::c_str!(MESSAGE); // Error.
/// ```
#[macro_export]
macro_rules! const_cstr {() => ( $crate::cstr!() ); (
    $($s:expr),* $(,)?
) => ({
    const BYTES: &[$crate::__::core::primitive::u8] = {
        $crate::const_concat_bytes!($($s ,)*)
    };
    /// Assert lack of inner null bytes.
    const _: $crate::const_::__::c_strlen::<{
        BYTES.len() - if BYTES[BYTES.len() - 1] == b'\0' { 1 } else { 0 }
    }> = $crate::const_::__::c_strlen::<{
        $crate::const_::__::c_strlen(BYTES)
    }> {};
    unsafe {
        $crate::const_::CStr::from_bytes_with_nul_unchecked(
            // Append a null terminator if needed.
            if BYTES[BYTES.len() - 1] == b'\0' {
                BYTES
            } else {
                $crate::const_concat_bytes!(BYTES, b"\0")
            }
        )
    }
})}
#[doc(inline)] pub use const_cstr as c_str;

/// [`const`-friendly][crate::const_] version of
/// [`concat_bytes!`][crate::concat_bytes].
///
/// ```rust
/// # fn main () {}
/// #[macro_use]
/// extern crate byte_strings;
///
/// const GREETING: &str = "Hello";
/// const MESSAGE: &[u8; 13] = const_concat_bytes!(GREETING, ", World!");
/// ```
#[macro_export]
macro_rules! const_concat_bytes {
    () => (b"");
    (
        $single:expr $(,)?
    ) => (
        $crate::const_as_bytes!($single)
    );

    (
        $first:expr $(,
        $rest:expr)+ $(,)?
    ) => (
        $crate::__concat_bytes_two!(
            $crate::const_as_bytes!($first),
            $crate::const_concat_bytes!($($rest),+),
        )
    );
}
#[doc(inline)] pub use const_concat_bytes as concat_bytes;

#[doc(hidden)] /** Not part of the public API */ #[macro_export]
macro_rules! __concat_bytes_two {(
    $left:expr,
    $right:expr $(,)?
) => ({
    const LEFT: &'static [$crate::__::core::primitive::u8] = $left;
    const __RIGHT: &'static [$crate::__::core::primitive::u8] = {
        mod __ {
            pub const RIGHT: &[$crate::__::core::primitive::u8] = $right;
        }
        __::RIGHT
    };
    unsafe {
        use $crate::const_::__::{Contiguous, core::{self, primitive::*, mem}};
        const LEFT_LEN: usize = LEFT.len();
        const LEFT_BYTES: &'static [u8; LEFT_LEN] = unsafe {
            mem::transmute(LEFT.as_ptr())
        };
        const RIGHT_LEN: usize = __RIGHT.len();
        const RIGHT_BYTES: &'static [u8; RIGHT_LEN] = unsafe {
            mem::transmute(__RIGHT.as_ptr())
        };
        const CONCAT_CONTIGUOUS: (
            &'static Contiguous<
                [u8; LEFT_LEN],
                [u8; RIGHT_LEN],
            >
        ) = &Contiguous(
            *LEFT_BYTES,
            *RIGHT_BYTES,
        );
        const CONCAT_BYTES: &'static [u8; LEFT_LEN + RIGHT_LEN] = unsafe {
            mem::transmute(CONCAT_CONTIGUOUS)
        };
        CONCAT_BYTES
    }
})}
