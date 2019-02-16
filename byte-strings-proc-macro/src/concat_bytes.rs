::cfg_if::cfg_if![ if #[cfg(feature = "proc-macro-hygiene")]
{
    #[cfg(any())]
    mod objective {
        macro_rules! concat_bytes {(
                $($input_exprs:expr),+ $(,)?
        ) => (
            b"_"
        )}
    }

    #[doc(hidden)]
    #[proc_macro]
    pub fn concat_bytes (
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

        let mut bytes: Vec<u8> = Vec::new();
        for expr in input_exprs {
            match expr {
                syn::Expr::Lit(
                    syn::ExprLit {
                        lit: syn::Lit::ByteStr(ref literal_bytes),
                        ..
                    }
                ) => {
                    bytes.extend_from_slice(&literal_bytes.value());
                },

                _ => {
                    throw!(expr.span()=>
                        "cannot concatenate a non byte string literal."
                    );
                },
            }
        };

        proc_macro::TokenStream::from(
            proc_macro::TokenTree::Literal(
                proc_macro::Literal::byte_string(&bytes)
            )
        )
    }
}
else
{
    #[cfg(any())]
    mod objective {
        macro_rules! const_concat_bytes {(
            const $const_literal_name:ident = concat_bytes!(
                $($input_exprs:expr),+ $(,)?
            ) ;
        ) => (
            const $const_literal_name: &[u8; _] = b"_";
        )}
    }

    struct ConstBConcat {
        const_literal_name: syn::Ident,
        input_exprs: CommaExprs,
    }

    impl Parse for ConstBConcat
    {
        fn parse (input: syn::parse::ParseStream) -> syn::parse::Result<Self>
        {
            macro_rules! parse_token {($tt:tt) => (
                input.parse::<syn::Token![$tt]>()
            )}

            parse_token!( const )?;
            let const_literal_name: syn::Ident = input.parse()?;
            parse_token!( = )?;
            input.parse::<kw::concat_bytes>()?;
            parse_token!( ! )?;
            let input_exprs: CommaExprs = syn::group::
                parse_parens(&input)?
                    .content
                    .parse_terminated(syn::Expr::parse)?
            ;
            parse_token!( ; )?;
            Ok(ConstBConcat {
                const_literal_name,
                input_exprs,
            })
        }
    }

    #[proc_macro]
    pub fn const_concat_bytes (
        input: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream
    {
        let ConstBConcat {
            const_literal_name,
            input_exprs,
        } = syn::parse_macro_input!(input);

        let mut bytes: Vec<u8> = Vec::new();

        let mut input_exprs = input_exprs.into_iter().peekable();
        let expr_span = match input_exprs.peek() {
            Some(expr) => expr.span().clone(),
            _ => throw!(
                "expected at least one argument"
            ),
        };

        for expr in input_exprs {
            match expr {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::ByteStr(ref literal_bytes),
                    ..
                }) => {
                    bytes.extend_from_slice(&literal_bytes.value());
                },

                _ => {
                    throw!(expr.span()=>
                        "cannot concatenate a non byte string literal"
                    )
                },
            }
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
                attrs: vec![],

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
