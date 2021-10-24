#![cfg(feature = "const-friendly")]
#[macro_use]
extern crate byte_strings;

#[test]
fn concat_bytes() {
    const GREETING: &str = "Hello";
    const MESSAGE: &[u8; 13] = const_concat_bytes!(GREETING, ", World!");
    assert_eq!(MESSAGE, b"Hello, World!");
}


#[test]
fn concat() {
    const GREETING: &str = "Hello";
    const MESSAGE: &str = const_concat!(GREETING, ", World!");
    assert_eq!(MESSAGE, "Hello, World!");
}

#[test]
fn c_str() {
    use ::byte_strings::const_;

    const MESSAGE: &str = "Hello, World!";
    const C_MESSAGE: &const_::CStr = const_::c_str!(MESSAGE);
    assert_eq!(
        C_MESSAGE.to_bytes_with_nul(),
        b"Hello, World!\0",
    );
}
