//! Crate not intended for direct use.
//! Use https:://docs.rs/byte-strings instead.
// Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template
#![allow(nonstandard_style, unused_imports)]

use ::core::{
    mem,
    ops::Not as _,
};
use ::proc_macro::{
    TokenStream,
};
use ::proc_macro2::{
    Span,
    TokenStream as TokenStream2,
    TokenTree as TT,
};
use ::quote::{
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use ::syn::{*,
    parse::{Parse, Parser, ParseStream},
    punctuated::Punctuated,
    Result, // Explicitly shadow it
    spanned::Spanned,
};

use input_bytes::Input;
mod input_bytes;

#[proc_macro] pub
fn concat_bytes (
    input: TokenStream,
) -> TokenStream
{
    concat_bytes_impl(input.into())
    //  .map(|ret| { println!("{}", ret); ret })
        .unwrap_or_else(|err| {
            let mut errors =
                err .into_iter()
                    .map(|err| Error::new(
                        err.span(),
                        format_args!("`#[byte_strings::concat_bytes]`: {}", err),
                    ))
            ;
            let mut err = errors.next().unwrap();
            errors.for_each(|cur| err.combine(cur));
            err.to_compile_error()
        })
        .into()
}

fn concat_bytes_impl (
    input: TokenStream2,
) -> Result<TokenStream2>
{
    let Input(_, ref mut bytes) = parse2(input)?;
    let byte_string_literal = LitByteStr::new(bytes, Span::call_site());
    Ok(byte_string_literal.into_token_stream())
}

#[proc_macro] pub
fn c_str (
    input: TokenStream,
) -> TokenStream
{
    c_str_impl(input.into())
    //  .map(|ret| { println!("{}", ret); ret })
        .unwrap_or_else(|err| {
            let mut errors =
                err .into_iter()
                    .map(|err| Error::new(
                        err.span(),
                        format_args!("`#[byte_strings::c_str]`: {}", err),
                    ))
            ;
            let mut err = errors.next().unwrap();
            errors.for_each(|cur| err.combine(cur));
            err.to_compile_error()
        })
        .into()
}

fn c_str_impl (
    input: TokenStream2,
) -> Result<TokenStream2>
{
    let Input(ref crate_, ref mut bytes) = parse2(input)?;
    match bytes.iter().position(|&b| b == b'\0') {
        | Some(i) if i < bytes.len() - 1 => {
            // Not the last byte: error!
            return Err(Error::new(
                Span::call_site(),
                format!("Inner null byte at index {}", i),
            ));
        },
        | None => {
            // No terminating null byte: add it!
            bytes.reserve_exact(1);
            bytes.push(b'\0');
        },
        | Some(_last_byte) => {
            // Terminating null byte already present: nothing to do.
        },
    }
    let byte_string_literal = LitByteStr::new(bytes, Span::call_site());
    Ok(quote!(
        {
            let stmt_expr_attr_workaround;
            #[allow(unused_unsafe)] {
                stmt_expr_attr_workaround = unsafe {
                    #crate_::__::std::ffi::CStr::from_bytes_with_nul_unchecked(
                        #byte_string_literal
                    )
                };
            }
            stmt_expr_attr_workaround
        }
    ))
}
