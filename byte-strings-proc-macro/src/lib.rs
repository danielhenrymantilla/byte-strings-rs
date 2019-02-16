extern crate proc_macro;
use ::quote;
use ::syn::{
    self,
    spanned::Spanned,
};

#[cfg(not(feature = "proc-macro-hygiene"))]
/// trait
use ::syn::parse::Parse;

macro_rules! throw {
    ($span:expr => $message:expr) => (
        return proc_macro::TokenStream::from(
            quote::quote_spanned! {$span=>
                compile_error!($message)
            }
        )
    );

    ($message:expr) => (
        return proc_macro::TokenStream::from(
            quote::quote! {
                compile_error!($message)
            }
        )
    );
}

mod kw {
    ::syn::custom_keyword!{
        concat_bytes
    }
    ::syn::custom_keyword!{
        as_bytes
    }
    ::syn::custom_keyword!{
        identity_non_null
    }
    ::syn::custom_keyword!{
        c_str
    }
}

type CommaExprs = syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>;

include!{
    "concat_bytes.rs"
}

include!{
    "as_bytes.rs"
}

include!{
    "identity_non_null.rs"
}

include!{
    "c_str.rs"
}
