::cfg_if::cfg_if![ if #[cfg(feature = "proc-macro-hygiene")]
{
    #[cfg(any())]
    mod objective {
        macro_rules! c_str {(
                $( $input_str_literal:expr ),+ $(,)?
        ) => (
            unsafe {
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    $crate::concat_bytes!(
                        $(
                            $string_literal ,
                        )+
                        b"\0",
                    )
                )
            }
        )}
    }

    #[doc(hidden)]
    #[proc_macro]
    pub fn c_str (
        input: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream
    {
        use ::syn::parse::Parser;

        let input_exprs = match CommaExprs::parse_terminated.parse(input) {
            Ok(input_exprs) => input_exprs,

            Err(err) => throw!(err.span()=>
                "Could not parse a comma-separated sequence of expressions"
            ),
        };

        let mut input_exprs = input_exprs.into_iter().peekable();
        let expr_span = match input_exprs.peek() {
            Some(expr) => expr.span().clone(),
            _ => throw!(
                "expected at least one argument"
            ),
        };

        let mut bytes: Vec<u8> = Vec::new();

        for expr in input_exprs {
            match expr {
                syn::Expr::Lit(
                    syn::ExprLit {
                        lit: syn::Lit::ByteStr(ref bytestr_literal),
                        ..
                    }
                ) => {
                    match &bytestr_literal.value()[..] { literal_bytes => {
                        if literal_bytes.contains(&0) {
                            throw!(expr.span()=>
                                "input literals cannot contain null bytes"
                            );
                        }
                        bytes.extend_from_slice(literal_bytes);
                    }}
                },
                syn::Expr::Lit(
                    syn::ExprLit {
                        lit: syn::Lit::Str(ref str_literal),
                        ..
                    }
                ) => {
                    match str_literal.value().as_bytes() { literal_bytes => {
                        if literal_bytes.contains(&0) {
                            throw!(expr.span()=>
                                "input literals cannot contain null bytes"
                            );
                        }
                        bytes.extend_from_slice(literal_bytes);
                    }}
                },

                _ => throw!(expr.span()=>
                    "expected a string literal (or a byte string literal)"
                ),
            }
        };

        bytes.reserve_exact(1); bytes.push(0);

        let lit = syn::Lit::
            ByteStr(syn::LitByteStr::new(
                &bytes,
                expr_span,
            ))
        ;
        let bytes = syn::Expr::
            Lit(syn::ExprLit { attrs: Vec::default(), lit })
        ;

        proc_macro::TokenStream::from(quote::quote!{
            unsafe {
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    #bytes
                )
            }
        })
    }
}
else
{
    #[cfg(any())]
    mod objective {
        macro_rules! const_c_str {(
            const $const_literal_name:ident = c_str!(
                $input_expr:expr
            );
        ) => (
            const $const_literal_name: &[u8; _] = c_str!(input_expr);
        )}
    }

    struct ConstCStr {
        const_literal_name: syn::Ident,
        input_exprs: CommaExprs,
    }

    impl Parse for ConstCStr
    {
        fn parse (input: syn::parse::ParseStream) -> syn::parse::Result<Self>
        {
            macro_rules! parse_token {($tt:tt) => (
                input.parse::<syn::Token![$tt]>()
            )}

            parse_token!( const )?;
            let const_literal_name: syn::Ident = input.parse()?;
            parse_token!( = )?;
            input.parse::<kw::c_str>()?;
            parse_token!( ! )?;
            let input_exprs: CommaExprs = syn::group::
                parse_parens(&input)?
                    .content
                    .parse_terminated(syn::Expr::parse)?
            ;
            parse_token!( ; )?;
            Ok(ConstCStr {
                const_literal_name,
                input_exprs,
            })
        }
    }

    #[proc_macro]
    pub fn const_c_str (
        input: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream
    {
        let ConstCStr {
            const_literal_name,
            input_exprs,
        } = syn::parse_macro_input!(input);

        let mut input_exprs = input_exprs.into_iter().peekable();
        let expr_span = match input_exprs.peek() {
            Some(expr) => expr.span().clone(),
            _ => throw!(
                "expected at least one argument"
            ),
        };

        let mut bytes: Vec<u8> = Vec::new();

        for expr in input_exprs {
            match expr {
                syn::Expr::Lit(
                    syn::ExprLit {
                        lit: syn::Lit::ByteStr(ref bytestr_literal),
                        ..
                    }
                ) => {
                    match &bytestr_literal.value()[..] { literal_bytes => {
                        if literal_bytes.contains(&0) {
                            throw!(expr.span()=>
                                "input literals cannot contain null bytes"
                            );
                        }
                        bytes.extend_from_slice(literal_bytes);
                    }}
                },
                syn::Expr::Lit(
                    syn::ExprLit {
                        lit: syn::Lit::Str(ref str_literal),
                        ..
                    }
                ) => {
                    match str_literal.value().as_bytes() { literal_bytes => {
                        if literal_bytes.contains(&0) {
                            throw!(expr.span()=>
                                "input literals cannot contain null bytes"
                            );
                        }
                        bytes.extend_from_slice(literal_bytes);
                    }}
                },

                _ => throw!(expr.span()=>
                    "expected a string literal (or a byte string literal)"
                ),
            }
        };

        bytes.reserve_exact(1); bytes.push(0);

        let bytes = syn::Expr::
            Lit(syn::ExprLit {
                attrs: Vec::default(),

                lit: syn::Lit::ByteStr(syn::LitByteStr::new(
                    // value
                    &bytes,

                    // Span
                    expr_span,
                ))
            })
        ;

        proc_macro::TokenStream::from(quote::quote! {
            union transmute {
                src: &'static [u8],
                dst: &'static ::std::ffi::CStr,
            }

            const transmute_is_sound_guard: [();
                ::std::mem::size_of::<&'static [u8]>()
            ] = [();
                ::std::mem::size_of::<&'static ::std::ffi::CStr>()
            ];

            const #const_literal_name: &'static ::std::ffi::CStr = unsafe {
                (transmute { src: #bytes }).dst
            };
        })
    }
}];
