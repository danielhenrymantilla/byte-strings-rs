#![cfg_attr(feature = "proc-macro-hygiene",
    feature(proc_macro_hygiene)
)]

#[test]
fn concat_bytes ()
{
    use ::byte_strings::concat_bytes;

    assert_eq!(
        concat_bytes!(b"Hello, ", b"World!"),
        b"Hello, World!",
    );
}

#[test]
fn as_bytes ()
{
    use ::byte_strings::as_bytes;

    assert_eq!(
        as_bytes!("Hello, World!"),
        b"Hello, World!",
    );
}

#[test]
fn c_str ()
{
    use ::byte_strings::c_str;

    assert_eq!(
        c_str!("Hello, World!"),
        ::std::ffi::CStr::from_bytes_with_nul(b"Hello, World!\0").unwrap(),
    )
}
