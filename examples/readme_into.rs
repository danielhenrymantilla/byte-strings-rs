fn main ()
{
    use ::byte_strings::c_str;

    dbg!(safe_puts(
        c_str!(
            "Hello, ",
            "World!",
        ) // No runtime error, no runtime cost
    ));
}

use self::puts::safe_puts;
mod puts {
    use ::std::{
        ffi::CStr,
        os::raw::{c_char, c_int},
    };

    /// C FFI
    extern "C" { fn puts (message: *const c_char) -> c_int; }

    /// Safe wrapper around C FFI
    pub fn safe_puts (message: &'_ CStr) -> i32
    {
        unsafe {
            puts(message.as_ptr()) as i32
        }
    }
}
