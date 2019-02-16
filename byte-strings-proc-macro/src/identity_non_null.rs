::cfg_if::cfg_if![ if #[cfg(feature = "proc-macro-hygiene")]
{
    #[cfg(any())]
    mod objective {
        macro_rules! identity_non_null {(
                $input_bytestr_literal:expr
        ) => (
            if $input_bytestr_literal.contains(&0) {
                compile_error!("input contains a null byte")
            } else {
                $input_bytestr_literal
            }
        )}
    }

    #[doc(hidden)]
    #[proc_macro]
    pub fn identity_non_null (
        input: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream
    {
        let input_expr = syn::parse_macro_input!(input as syn::Expr);

        match input_expr {
            // if the input expression is a byte string literal,
            // return it as is iff it contains no null byte, else fail
            syn::Expr::Lit(
                syn::ExprLit {
                    lit: syn::Lit::ByteStr(ref bytes),
                    ..
                }
            ) => {
                if bytes.value().contains(&0) {
                    throw!(input_expr.span()=>
                        "input contains a null byte"
                    );
                } else {
                    proc_macro::TokenStream::from(quote::quote! {
                        input_expr
                    })
                }
            },

            // else, the macro was misused
            _ => throw!(input_expr.span()=>
                "expected a byte string literal"
            ),
        }
    }
}
else
{
    #[cfg(any())]
    mod objective {
        macro_rules! const_identity_non_null {(
            const $const_literal_name:ident = identity_non_null!(
                $input_bytestr_literal:expr
            );
        ) => (
            const $const_literal_name: &[u8; _] = b"_";
        )}
    }

    struct ConstIdentityNonNull {
        const_literal_name: syn::Ident,
        input_bytestr_literal: syn::Expr,
    }

    impl Parse for ConstIdentityNonNull
    {
        fn parse (input: syn::parse::ParseStream) -> syn::parse::Result<Self>
        {
            macro_rules! parse_token {[$tt:tt] => (
                input.parse::<syn::Token![$tt]>()?
            )}

            parse_token![ const ];
            let const_literal_name: syn::Ident = input.parse()?;
            parse_token![ =     ];
            input.parse::<kw::identity_non_null>()?;
            parse_token![ !     ];

            let input_bytestr_literal: syn::Expr = syn::group::
                parse_parens(&input)?
                    .content
                    .parse()?
            ;
            parse_token![ ;     ];
            Ok(ConstIdentityNonNull {
                const_literal_name,
                input_bytestr_literal,
            })
        }
    }

    #[proc_macro]
    pub fn const_identity_non_null (
        input: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream
    {
        let ConstIdentityNonNull {
            const_literal_name,
            input_bytestr_literal: input_expr,
        } = syn::parse_macro_input!(input);

        match input_expr {
            // if the input expression is a byte string literal,
            // return it as is iff it contains no null byte, else fail
            syn::Expr::Lit(
                syn::ExprLit {
                    lit: syn::Lit::ByteStr(ref bytes),
                    ..
                }
            ) => {
                if bytes.value().contains(&0) {
                    throw!(input_expr.span()=>
                        "input contains a null byte"
                    );
                } else {
                    let len = syn::Expr::
                        Lit(syn::ExprLit {
                            attrs: vec![],

                            lit: syn::Lit::Int(syn::LitInt::new(
                                // value
                                bytes.value().len() as u64,

                                // suffix
                                syn::IntSuffix::Usize,

                                // Span
                                input_expr.span().clone(),
                            ))
                        })
                    ;

                    proc_macro::TokenStream::from(quote::quote! {
                        const #const_literal_name
                            : &'static [u8; #len]
                            = #input_expr
                        ;
                    })
                }
            },

            // else, the macro was misused
            _ => throw!(input_expr.span()=>
                "expected a byte string literal"
            ),
        }
    }
}];
