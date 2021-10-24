use super::*;

/// Same as [`input`], but for an expect `($crate)` initial parameter
/// to be robust to crate re-exports.
pub(in crate)
struct Input /* = */ (
    pub(in crate) TokenStream2,
    pub(in crate) Vec<u8>,
);

impl Parse for Input {
    fn parse (input: ParseStream<'_>)
      -> Result<Input>
    {
        let crate_; bracketed!(crate_ in input);
        Ok(Input(crate_.parse().unwrap(), input.parse::<InputBytes>()?.0))
    }
}

/// A sequence of:
///   - (byte) string literals;
///
///   - or macro invocations of `as_bytes/c{,_str}/concat{,_bytes}!`
///     which are recursively fed such a sequence;
///
///   - or `stringify!` invocations.
struct InputBytes /* = */ (
    Vec<u8>,
);

impl Parse for InputBytes {
    fn parse (input: ParseStream<'_>)
      -> Result<InputBytes>
    {
        macro_rules! supported_macros {(
            $($macro:ident),* $(,)?
        ) => (
            mod kw {
            $(
                ::syn::custom_keyword!($macro);
            )*
                ::syn::custom_keyword!(stringify);
            }
            let mut ret = vec![0_u8; 0];
            while input.is_empty().not() {
                let snoopy = input.lookahead1();
                match () {
                    | _case if snoopy.peek(LitStr) => {
                        let s = input.parse::<LitStr>().unwrap();
                        ret.append(&mut Vec::from(s.value()));
                    },
                    | _case if snoopy.peek(LitByteStr) => {
                        let s = input.parse::<LitByteStr>().unwrap();
                        ret.append(&mut Vec::from(s.value()));
                    },
                $(
                    | _case if snoopy.peek(kw::$macro) => {
                        let _: kw::$macro = input.parse().unwrap();
                        let _: Token![!] = input.parse()?;
                        let contents = input.parse::<::proc_macro2::Group>()?.stream();
                        let Self(ref mut bytes) = parse2(contents)?;
                        ret.append(bytes);
                    },
                )*
                    | _case if snoopy.peek(kw::stringify) => {
                        let _: kw::stringify = input.parse().unwrap();
                        let _: Token![!] = input.parse()?;
                        let contents = input.parse::<::proc_macro2::Group>()?.stream();
                        ret.append(&mut Vec::from(contents.to_string()));
                    },
                    | _default => return Err(snoopy.error()),
                }
                let _: Option<Token![,]> = input.parse()?;
            }
            return Ok(Self(ret));
        )}
        supported_macros!(as_bytes, c, c_str, concat, concat_bytes);
    }
}
