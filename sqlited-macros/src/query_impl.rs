use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    FnArg, GenericArgument, Ident, Result as SynResult, ReturnType, Token, Type, Visibility,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

// Query macro input parse structure
#[allow(dead_code)]
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

    // Determine return type pattern and characteristics
    let return_type_info = extract_return_type_info(&input.return_type);
    let (model_type, is_vec, is_tuple, is_unit) = (
        return_type_info.model_type,
        return_type_info.is_vec,
        return_type_info.is_tuple,
        return_type_info.is_unit,
    );

    // Build method params
    let method_params = generate_method_params(args);

    // Build param names for sql_params! macro
    let param_names = args
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .collect::<Vec<_>>();

    // Generate different code based on return type
    if is_unit { // Handle Result<()> case first
        quote! {
            #visibility fn #fn_name(#method_params) -> sqlited::Result<()> {
                let query = sqlited::sql_str!(#query_str);
                self.execute(query, sqlited::rq::params![#(#param_names),*])?;
                Ok(())
            }
        }
        .into()
    } else if is_vec {
        if is_tuple {
            // Collection of tuples
            let tuple_elements = extract_tuple_elements(&model_type);
            let indices = (0..tuple_elements.len()).collect::<Vec<_>>();

            quote! {
                #visibility fn #fn_name(#method_params) -> sqlited::Result<Vec<#model_type>> {
                    let query = sqlited::sql_str!(#query_str);
                    self.query(query, sqlited::rq::params![#(#param_names),*], |row| {
                        Ok((
                            #(row.get(#indices)?,)*
                        ))
                    })
                }
            }
            .into()
        } else if is_primitive_type(&model_type) {
            // Collection of primitives
            quote! {
                #visibility fn #fn_name(#method_params) -> sqlited::Result<Vec<#model_type>> {
                    let query = sqlited::sql_str!(#query_str);
                    self.query(query, sqlited::rq::params![#(#param_names),*], |row| row.get(0))
                }
            }
            .into()
        } else {
            // Collection of structs (original implementation)
            quote! {
                #visibility fn #fn_name(#method_params) -> sqlited::Result<Vec<#model_type>> {
                    let query = sqlited::sql_str!(#query_str);
                    self.query(query, sqlited::rq::params![#(#param_names),*], #model_type::from_row)
                }
            }.into()
        }
    } else {
        if is_tuple {
            // Single tuple
            let tuple_elements = extract_tuple_elements(&model_type);
            let indices = (0..tuple_elements.len()).collect::<Vec<_>>();

            quote! {
                #visibility fn #fn_name(#method_params) -> sqlited::Result<#model_type> {
                    let query = sqlited::sql_str!(#query_str);
                    self.query_row(query, sqlited::rq::params![#(#param_names),*], |row| {
                        Ok((
                            #(row.get(#indices)?,)*
                        ))
                    })
                }
            }
            .into()
        } else if is_primitive_type(&model_type) {
            // Single primitive value
            quote! {
                #visibility fn #fn_name(#method_params) -> sqlited::Result<#model_type> {
                    let query = sqlited::sql_str!(#query_str);
                    self.query_row(query, sqlited::rq::params![#(#param_names),*], |row| row.get(0))
                }
            }
            .into()
        } else {
            // Single struct (original implementation)
            quote! {
                #visibility fn #fn_name(#method_params) -> sqlited::Result<#model_type> {
                    let query = sqlited::sql_str!(#query_str);
                    self.query_row(query, sqlited::rq::params![#(#param_names),*], #model_type::from_row)
                }
            }.into()
        }
    }
}

// Struct to hold return type information
struct ReturnTypeInfo {
    model_type: TokenStream2,
    is_vec: bool,
    is_tuple: bool,
    is_unit: bool,
}

// Checks if a type is the unit type ()
fn is_unit_type(model_type: &TokenStream2) -> bool {
    let type_str = model_type.to_string().replace(" ", "");
    type_str == "()"
}

// Checks if a type is a primitive type
fn is_primitive_type(model_type: &TokenStream2) -> bool {
    let type_str = model_type.to_string();

    // Common primitive type names
    let primitives = [
        "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "char",
        "String", "str", "&str", "isize", "usize",
    ];

    primitives.iter().any(|&prim| type_str.contains(prim))
}

// Check if a TokenStream represents a tuple type
fn is_tuple_type(model_type: &TokenStream2) -> bool {
    let type_str = model_type.to_string();
    type_str.starts_with("(") && type_str.ends_with(")")
}

// Extract elements from a tuple type
fn extract_tuple_elements(tuple_type: &TokenStream2) -> Vec<TokenStream2> {
    let type_str = tuple_type.to_string();

    // Strip parentheses
    let content = type_str.trim_start_matches('(').trim_end_matches(')');

    // Split by comma - this is a simple approach, might need more robust parsing
    content
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<TokenStream2>()
                .unwrap_or_else(|_| quote!(Unknown))
        })
        .collect()
}

// Extract model type and return type information
fn extract_return_type_info(return_type: &ReturnType) -> ReturnTypeInfo {
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
                                    // Check for Vec<T> first
                                    if let GenericArgument::Type(Type::Path(tp)) = arg {
                                        if let Some(vec_segment) = tp.path.segments.first() {
                                            if vec_segment.ident == "Vec" {
                                                if let syn::PathArguments::AngleBracketed(inner_args) = &vec_segment.arguments {
                                                    if let Some(model_arg) = inner_args.args.first() {
                                                        let model_tokens = model_arg.to_token_stream();
                                                        return ReturnTypeInfo {
                                                            model_type: model_tokens.clone(),
                                                            is_vec: true,
                                                            is_tuple: is_tuple_type(&model_tokens),
                                                            is_unit: false, // Vec cannot be unit
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Handle single item T in Result<T>
                                    let model_tokens = arg.to_token_stream();
                                    let is_unit = is_unit_type(&model_tokens);
                                    return ReturnTypeInfo {
                                        model_type: model_tokens.clone(),
                                        is_vec: false,
                                        is_tuple: is_tuple_type(&model_tokens), // is_tuple_type now excludes ()
                                        is_unit,
                                    };
                                }
                            }
                        }
                    }
                    // Fallback for non-Result types (less likely for query!)
                    let model_tokens = type_path.to_token_stream();
                    return ReturnTypeInfo {
                        model_type: model_tokens.clone(),
                        is_vec: false,
                        is_tuple: false, // Assume non-Result direct types aren't tuples/unit for now
                        is_unit: false,
                    };
                }
                Type::Tuple(type_tuple) => {
                    // Handle direct () return type if needed, though Result<()> is standard
                    if type_tuple.elems.is_empty() {
                         return ReturnTypeInfo {
                            model_type: quote! { () },
                            is_vec: false,
                            is_tuple: false,
                            is_unit: true,
                        };
                    }
                }
                _ => {}
            }
        }
        ReturnType::Default => { // Handle -> () case (implies success or panic)
             return ReturnTypeInfo {
                model_type: quote! { () },
                is_vec: false,
                is_tuple: false,
                is_unit: true, // Treat as unit for execution context
            };
        }
    }
    // Default fallback
    ReturnTypeInfo {
        model_type: quote! { UnknownType },
        is_vec: false,
        is_tuple: false,
        is_unit: false,
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
