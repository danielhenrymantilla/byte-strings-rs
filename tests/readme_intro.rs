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

#[test]
fn main_simple_and_safe ()
{
    use ::byte_strings::c_str;

    safe_puts(
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

    safe_puts(
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

    safe_puts(
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

    safe_puts(
        CStr::from_bytes_with_nul( // error prone! (inner null or no null)
            concat!(
                "Hello,\0",
                "World!\0",
            ).as_bytes()
        ).unwrap()
    );
}
