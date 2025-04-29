use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{
    Block, FnArg, GenericArgument, Ident, Result as SynResult, ReturnType, Token, Type, Visibility,
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

// Query macro input parse structure
struct QueryInput {
    visibility: Visibility,
    fn_token: Token![fn],
    name: Ident,
    paren_token: syn::token::Paren,
    args: Punctuated<FnArg, Comma>,
    return_type: ReturnType,
    brace_token: syn::token::Brace,

    query: TokenStream2,
}

impl Parse for QueryInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let visibility = input.parse()?;
        let fn_token = input.parse()?;
        let name = input.parse()?;

        // Parse parameter list
        let content;
        let paren_token = syn::parenthesized!(content in input);
        let args = Punctuated::parse_terminated(&content)?;

        // Parse return type
        let return_type = input.parse()?;

        // Parse query body
        let body;
        let brace_token = syn::braced!(body in input);

        // Collect SQL query string
        let query_tokens: TokenStream2 = body.parse()?;
        let query = query_tokens;

        Ok(QueryInput {
            visibility,
            fn_token,
            name,
            paren_token,
            args,
            return_type,
            brace_token,
            query,
        })
    }
}

/// Implement query! macro processor
pub fn query_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as QueryInput);

    // Extract function parts
    let visibility = &input.visibility;
    let fn_name = &input.name;
    let args = &input.args;
    let query_str = &input.query;

    // Determine if we're returning a collection
    let (model_type, is_vec) = extract_model_type_and_collection(&input.return_type);

    // Build method params
    let method_params = generate_method_params(args);

    // Build param names for sql_params! macro
    let param_names = args.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                return Some(pat_ident.ident.clone());
            }
        }
        None
    }).collect::<Vec<_>>();

    // Generate code based on whether returning a single item or a collection
    if is_vec {
        // Multiple results
        quote! {
            #visibility fn #fn_name(#method_params) -> sqlited::Result<Vec<#model_type>> {
                let query = sqlited::sql_str!(#query_str);
                self.query(query, sqlited::rq::params![#(#param_names),*], #model_type::from_row)
            }
        }
        .into()
    } else {
        // Single result
        quote! {
            #visibility fn #fn_name(#method_params) -> sqlited::Result<#model_type> {
                let query = sqlited::sql_str!(#query_str);
                self.query_row(query, sqlited::rq::params![#(#param_names),*], #model_type::from_row)
            }
        }
        .into()
    }
}

// Extract model type and whether it's a Vec or single item
fn extract_model_type_and_collection(return_type: &ReturnType) -> (TokenStream2, bool) {
    match return_type {
        ReturnType::Type(_, ty) => {
            match &**ty {
                Type::Path(type_path) => {
                    // Check if return type is Result<T> or similar
                    if let Some(segment) = type_path.path.segments.last() {
                        if segment.ident == "Result"
                            || segment.ident == "anyhow::Result"
                            || segment.ident == "sqlited::Result"
                        {
                            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(arg) = args.args.first() {
                                    let type_str = quote!(#arg).to_string();

                                    // Check if it's a Vec<T>
                                    if type_str.starts_with("Vec < ")
                                        || type_str.starts_with("Vec<")
                                    {
                                        // Extract inner type from Vec<T>
                                        match arg {
                                            GenericArgument::Type(generic_type) => {
                                                if let Type::Path(inner_path) = generic_type {
                                                    if let Some(vec_segment) =
                                                        inner_path.path.segments.first()
                                                    {
                                                        if vec_segment.ident == "Vec" {
                                                            if let syn::PathArguments::AngleBracketed(inner_args) = &vec_segment.arguments {
                                                              if let Some(model_type) = inner_args.args.first() {
                                                                  return (model_type.to_token_stream(), true);
                                                              }
                                                          }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }

                                        // Try parsing more directly
                                        let s = type_str.trim();
                                        let idx_start = s.find('<').unwrap_or(0) + 1;
                                        let idx_end = s.rfind('>').unwrap_or(s.len());
                                        let inner_type = &s[idx_start..idx_end].trim();

                                        let inner_type_stream: TokenStream2 = inner_type
                                            .parse()
                                            .unwrap_or_else(|_| quote!(UnknownType));

                                        return (inner_type_stream, true);
                                    }

                                    // Single item
                                    return (arg.to_token_stream(), false);
                                }
                            }
                        }
                    }

                    // Direct return type (no Result wrapper)
                    let type_str = quote!(#type_path).to_string();
                    let is_vec = type_str.starts_with("Vec < ") || type_str.starts_with("Vec<");

                    if is_vec {
                        // Try to extract inner type
                        let s = type_str.trim();
                        let idx_start = s.find('<').unwrap_or(0) + 1;
                        let idx_end = s.rfind('>').unwrap_or(s.len());
                        let inner_type = &s[idx_start..idx_end].trim();

                        let inner_type_stream: TokenStream2 =
                            inner_type.parse().unwrap_or_else(|_| quote!(UnknownType));

                        return (inner_type_stream, true);
                    }

                    return (type_path.to_token_stream(), false);
                }
                _ => {}
            }

            // Default if we can't parse it
            (quote! { UnknownType }, false)
        }
        _ => (quote! { UnknownType }, false),
    }
}

// Generate method parameters
fn generate_method_params(args: &Punctuated<FnArg, Comma>) -> TokenStream2 {
    let mut method_params = quote! { &self };

    if !args.is_empty() {
        method_params = quote! { &self, #args };
    }

    method_params
}
