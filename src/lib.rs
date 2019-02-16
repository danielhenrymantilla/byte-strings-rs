#![cfg_attr(feature = "nightly",
    feature(external_doc)
)]
#![cfg_attr(feature = "nightly",
    doc(include = "../README.md")
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "See [crates.io](https://crates.io/crates/byte-strings)"
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "for more info about this crate."
)]

::cfg_if::cfg_if![ if #[cfg(feature = "proc-macro-hygiene")]
{
    #[doc(hidden)]
    pub
    use ::byte_strings_proc_macro::{
        as_bytes as as_bytes_proc_macro,
        concat_bytes as concat_bytes_proc_macro,
        c_str as c_str_proc_macro,
    };


    /// Concatenates byte string literals into a single byte string literal
    ///
    /// This macro takes any number of comma-separated byte string literals,
    /// and evaluates to (a static reference to) a byte array made of all the
    /// bytes of the given byte string literals concatenated left-to-right.
    ///
    /// Hence the macro evaluates to the type `&'static [u8; N]`
    /// (where N is the total number of bytes), which can also "be seen as"
    /// (coerce to) a static byte slice (_i.e.,_ `&'static [u8]`).
    ///
    /// Arguments are **not** "byte-stringified".
    ///
    /// # Example
    ///
    /// ```rust,edition2018
    /// # #![feature(proc_macro_hygiene)]
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
        $($expr:expr),+ $(,)?
    ) => (
        $crate::concat_bytes_proc_macro!(
            $($expr),+
        )
    )}


    /// Evaluates the input string literal as a byte string literal.
    ///
    /// Hence the macro evaluates to the type `&'static [u8; N]`
    /// (where N is the total number of bytes), which can also "be seen as"
    /// (coerce to) a static byte slice (_i.e.,_ `&'static [u8]`).
    ///
    /// The input can be a byte string literal, in which case it is a no-op.
    ///
    /// # Example
    ///
    /// ```rust,edition2018
    /// # #![feature(proc_macro_hygiene)]
    /// use ::byte_strings::as_bytes;
    ///
    /// let bytes = as_bytes!("Hello, World!");
    /// assert_eq!(bytes, b"Hello, World!");
    /// ```
    ///
    /// ### Macro expansion:
    ///
    /// `as_bytes!("Hello, World!")`
    /// expands to
    /// `b"Hello, World!"`
    #[macro_export]
    macro_rules! as_bytes {(
        $str_literal:expr
    ) => (
        $crate::as_bytes_proc_macro!(
            $str_literal
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
    /// let Hello_W = c_str!("Hello, ", "W\0rld!");
    /// ```
    ///
    /// ### Macro expansion:
    ///
    /// `c_str!("Hello, ", "World!")`
    /// expands to
    /// `b"Hello, World!\0"`
    #[macro_export]
    macro_rules! c_str {(
        $($literal:expr),+ $(,)?
    ) => (
        $crate::c_str_proc_macro!( $($literal),+ )
    )}
}
else
{
    #[doc(hidden)]
    pub
    use ::byte_strings_proc_macro::{
        const_concat_bytes as const_concat_bytes_proc_macro,
        const_as_bytes as const_as_bytes_proc_macro,
        const_c_str as const_c_str_proc_macro,
    };


    /// Help rust disambiguate between an item and a statement
    #[doc(hidden)]
    #[macro_export]
    macro_rules! as_item {($item:item) => ($item)}

    /// Concatenates byte string literals into a single byte string literal
    ///
    /// This macro takes any number of comma-separated byte string literals,
    /// and evaluates to (a static reference to) a byte array made of all the
    /// bytes of the given byte string literals concatenated left-to-right.
    ///
    /// Hence the macro evaluates to the type `&'static [u8; N]`
    /// (where N is the total number of bytes), which can also "be seen as"
    /// (coerce to) a static byte slice (_i.e.,_ `&'static [u8]`).
    ///
    /// Arguments are **not** "byte-stringified".
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
    /// For those curious, `concat_bytes!(b"Hello, ", b"World!")`
    /// expands to:
    ///
    /// ```
    /// # macro_rules! ignore {{$($tt:tt)*} => (let _: &'static [u8; 13] = {$($tt)*};)}
    /// # ignore!
    /// {
    ///     const __byte_strings__concat_bytes: &'static [u8; 13usize] = b"Hello, World!";
    ///
    ///     __byte_strings__concat_bytes
    /// }
    /// ```
    ///
    /// This trick is needed to circumvent [the current restriction of
    /// procedural macros](
    /// https://github.com/rust-lang/rust/issues/54727)
    /// being able to expand to [items](
    /// https://doc.rust-lang.org/reference/items.html) only.
    #[macro_export]
    macro_rules! concat_bytes {(
        $($expr:expr),+ $(,)?
    ) => ({
        $crate::as_item! {
            $crate::const_concat_bytes_proc_macro! {
                const __byte_strings__concat_bytes = concat_bytes!( $($expr),+ );
            }
        }

        __byte_strings__concat_bytes
    })}


    /// Evaluates the input string literal as a byte string literal.
    ///
    /// Hence the macro evaluates to the type `&'static [u8; N]`
    /// (where N is the total number of bytes), which can also "be seen as"
    /// (coerce to) a static byte slice (_i.e.,_ `&'static [u8]`).
    ///
    /// The input can be a byte string literal, in which case it is a no-op.
    ///
    /// # Example
    ///
    /// ```rust,edition2018
    /// use ::byte_strings::as_bytes;
    ///
    /// let bytes = as_bytes!("Hello, World!");
    /// assert_eq!(bytes, b"Hello, World!");
    /// ```
    ///
    /// ### Macro expansion:
    ///
    /// For those curious, `as_bytes!("Hello, World!")`
    /// expands to:
    ///
    /// ```
    /// # macro_rules! ignore {{$($tt:tt)*} => (let _: &'static [u8; 13] = {$($tt)*};)}
    /// # ignore!
    /// {
    ///     const __byte_strings__as_bytes: &'static [u8; 13usize] = b"Hello, World!";
    ///
    ///     __byte_strings__as_bytes
    /// }
    /// ```
    ///
    /// This trick is needed to circumvent [the current restriction of
    /// procedural macros](
    /// https://github.com/rust-lang/rust/issues/54727)
    /// being able to expand to [items](
    /// https://doc.rust-lang.org/reference/items.html) only.
    #[macro_export]
    macro_rules! as_bytes {(
        $str_literal:expr
    ) => ({
        $crate::as_item! {
            $crate::const_as_bytes_proc_macro! {
                const __byte_strings__as_bytes = as_bytes!($str_literal);
            }
        }

        __byte_strings__as_bytes
    })}

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
    /// let Hello_W = c_str!("Hello, ", "W\0rld!");
    /// ```
    ///
    /// ### Macro expansion:
    ///
    /// For those curious, `c_str!(b"Hello, ", b"World!")`
    /// expands to:
    ///
    /// ```
    /// # macro_rules! ignore {{$($tt:tt)*} => ()}
    /// # ignore!
    /// {
    ///     union transmute {
    ///         src: &'static [u8],
    ///         dst: &'static ::std::ffi::CStr,
    ///     }
    ///
    ///     const transmute_is_sound_guard: [();
    ///         ::std::mem::size_of::<&'static [u8]>()
    ///     ] = [();
    ///         ::std::mem::size_of::<&'static ::std::ffi::CStr>()
    ///     ];
    ///
    ///     const __byte_strings__c_str: &'static ::std::ffi::CStr = unsafe {
    ///         (transmute { src: b"Hello, World!\0" }).dst
    ///     };
    ///
    ///     __byte_strings__c_str
    /// }
    /// ```
    ///
    /// This trick is needed to circumvent [the current restriction of
    /// procedural macros](
    /// https://github.com/rust-lang/rust/issues/54727)
    /// being able to expand to [items](
    /// https://doc.rust-lang.org/reference/items.html) only.
    ///
    /// Since [`::std::mem::transmute` is not a `const fn`](
    /// https://github.com/rust-lang/rust/issues/49450), `union` transmutation
    /// had to be used, although it allows using different sizes. To prevent
    /// Undefined Behaviour, a size-check guard using arrays was added.
    ///
    /// [C string]: ::std::ffi::CStr
    #[macro_export]
    macro_rules! c_str {(
        $($literal:expr),+ $(,)?
    ) => ({
        $crate::as_item! {
            $crate::const_c_str_proc_macro! {
                const __byte_strings__c_str = c_str!( $($literal),+ );
            }
        }

        __byte_strings__c_str
    })}
}];
