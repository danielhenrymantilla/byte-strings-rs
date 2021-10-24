// Try to mess up with the crates' namespace.
#[macro_use]
extern crate byte_strings as core;
extern crate core as byte_strings;

#[test]
fn basic ()
{
    let _: &'static [u8; 0] = concat_bytes!();
    let _: &'static [u8; 0] = concat_bytes!("");
    assert_eq!(
        as_bytes!("Hi"), b"Hi",
    );

    use ::std::ffi::CStr as CStr_;
    let c: &'static CStr_ = c!();
    assert_eq!(c.to_bytes_with_nul(), b"\0");
    let c: &'static CStr_ = c!("");
    assert_eq!(c.to_bytes_with_nul(), b"\0");
    let c: &'static CStr_ = c!("\0");
    assert_eq!(c.to_bytes_with_nul(), b"\0");
}

#[test]
fn nested ()
{
    assert_eq!(
        concat_bytes!("Hello, ", "World!"),
        b"Hello, World!",
    );
    assert_eq!(
        as_bytes!(concat!("Hello, ", "World!")),
        b"Hello, World!",
    );
    assert_eq!(
        as_bytes!(concat!("Hello, ", "World"), stringify!(!)),
        b"Hello, World!",
    );
}

#[test]
fn c_str ()
{
    let static_bytes = |c: &'static ::std::ffi::CStr| c.to_bytes_with_nul();
    assert_eq!(static_bytes(c!("Hell")), b"Hell\0");
    assert_eq!(static_bytes(c!("Hell\0")), b"Hell\0");
    assert_eq!(static_bytes(c!("Hell", "\0")), b"Hell\0");
    assert_eq!(static_bytes(c!("Hell", "o!")), b"Hello!\0");
}
