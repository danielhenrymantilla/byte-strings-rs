/// Some lib
mod safe {
    use ::std::{
        ffi::CStr,
        os::raw::{c_char, c_int},
    };

    /// private unsafe C FFI
    mod ffi { use super::*; extern "C" {
        pub fn puts (_: *const c_char) -> c_int;
    }}

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
