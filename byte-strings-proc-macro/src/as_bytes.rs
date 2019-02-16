::cfg_if::cfg_if![ if #[cfg(feature = "proc-macro-hygiene")]
{
    #[cfg(any())]
    mod objective {
        macro_rules! as_bytes {(
                $input_str_literal:expr
        ) => (
            b"_"
        )}
    }

    #[doc(hidden)]
    #[proc_macro]
    pub fn as_bytes (
        input: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream
    {
        let input_expr = syn::parse_macro_input!(input as syn::Expr);

        match input_expr {
            // if the input expression already was a byte string literal,
            // return it as is.
            syn::Expr::Lit(
                syn::ExprLit {
                    lit: syn::Lit::ByteStr(_),
                    ..
                }
            ) => proc_macro::TokenStream::from(quote::quote! {
                input_expr
            }),

            // else if the input expression is a string literal, create a
            // byte string literal out of its bytes.
            syn::Expr::Lit(
                syn::ExprLit {
                    lit: syn::Lit::Str(input_str_literal),
                    attrs: _,
                }
            ) => {
                proc_macro::TokenStream::from(
                    proc_macro::TokenTree::Literal(
                        proc_macro::Literal::byte_string(
                            input_str_literal.value().as_bytes()
                        )
                    )
                )
            },

            // else, the macro was misused
            _ => throw!(input_expr.span()=>
                "expected a string literal (or a byte string literal)"
            ),
        }
    }
}
else
{
    #[cfg(any())]
    mod objective {
        macro_rules! const_as_bytes {(
            const $const_literal_name:ident = as_bytes!(
                $input_str_literal:expr
            );
        ) => (
            const $const_literal_name: &[u8; _] = b"_";
        )}
    }

    struct ConstAsBytes {
        const_literal_name: syn::Ident,
        input_str_literal: syn::Expr,
    }

    impl Parse for ConstAsBytes
    {
        fn parse (input: syn::parse::ParseStream) -> syn::parse::Result<Self>
        {
            macro_rules! parse_token {[$tt:tt] => (
                input.parse::<syn::Token![$tt]>()?
            )}

            parse_token![ const ];
            let const_literal_name: syn::Ident = input.parse()?;
            parse_token![ =     ];
            input.parse::<kw::as_bytes>()?;
            parse_token![ !     ];

            let input_str_literal: syn::Expr = syn::group::
                parse_parens(&input)?
                    .content
                    .parse()?
            ;
            parse_token![ ;     ];
            Ok(ConstAsBytes {
                const_literal_name,
                input_str_literal,
            })
        }
    }

    #[proc_macro]
    pub fn const_as_bytes (
        input: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream
    {
        let ConstAsBytes {
            const_literal_name,
            input_str_literal: input_expr,
        } = syn::parse_macro_input!(input);

        let expr_span = input_expr.span().clone();

        let (bytes, attrs) = match input_expr {
            // if the input expression already was a byte string literal,
            // return it as is.
            syn::Expr::Lit(
                syn::ExprLit {
                    lit: syn::Lit::ByteStr(_),
                    ..
                }
            ) => return proc_macro::TokenStream::from(quote::quote! {
                const #const_literal_name: &'static str = #input_expr;
            }),

            // else if the input expression is a string literal, extract the
            // inner bytes (to create a byte string literal out of them).
            syn::Expr::Lit(
                syn::ExprLit {
                    lit: syn::Lit::Str(input_str_literal),
                    attrs,
                }
            ) => (input_str_literal.value().into_bytes(), attrs),

            // else, the macro was misused
            _ => throw!(input_expr.span()=>
                "expected a string literal (or a byte string literal)"
            ),
        };

        let len = syn::Expr::
            Lit(syn::ExprLit {
                attrs: vec![],

                lit: syn::Lit::Int(syn::LitInt::new(
                    // value
                    bytes.len() as u64,

                    // suffix
                    syn::IntSuffix::Usize,

                    // Span
                    expr_span.clone(),
                ))
            })
        ;

        let bytes = syn::Expr::
            Lit(syn::ExprLit {
                attrs,

                lit: syn::Lit::ByteStr(syn::LitByteStr::new(
                    // value
                    &bytes,

                    // Span
                    expr_span.clone(),
                ))
            })
        ;

        proc_macro::TokenStream::from(quote::quote! {
            const #const_literal_name: &'static [u8; #len] = #bytes;
        })
    }
}];
