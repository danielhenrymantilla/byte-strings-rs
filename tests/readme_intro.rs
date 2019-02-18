/// Some lib
mod safe {
    use ::std::{
        ffi::CStr,
        os::raw::{c_char, c_int},
    };

    /// private unsafe C FFI
    mod ffi { use super::*; pub unsafe extern "C"
        fn puts (_: *const c_char) -> c_int { 0 }
    }

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

#[test]
fn main_simple_and_safe ()
{
    use ::byte_strings::c_str;

    safe::puts(
        c_str!(
            "Hello, ",
            "World!",
        ) // No runtime error, no runtime cost
    );
}

#[test]
fn main_no_c_str_macro ()
{
    use ::std::ffi::CString;

    safe::puts(
        &CString::new( // runtime cost  (+ slightly error prone if inner null)
            concat!(
                "Hello, ",
                "World!",
            ).as_bytes()
        ).unwrap()
    );
}

#[test]
#[should_panic]
fn main_no_c_str_macro_2 ()
{
    use ::std::ffi::CStr;

    safe::puts(
        CStr::from_bytes_with_nul( // error prone! (inner null or no null)
            concat!(
                "Hello, ",
                "World!",
            ).as_bytes()
        ).unwrap()
    );
}

#[test]
#[should_panic]
fn main_no_c_str_macro_bad_copy_paste ()
{
    use ::std::ffi::CStr;

    safe::puts(
        CStr::from_bytes_with_nul( // error prone! (inner null or no null)
            concat!(
                "Hello,\0",
                "World!\0",
            ).as_bytes()
        ).unwrap()
    );
}
