use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, FnArg, GenericArgument, Ident, LitStr, Result as SynResult, ReturnType, Token, Type, Visibility
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

        let content;
        let paren_token = syn::parenthesized!(content in input);
        let args = Punctuated::parse_terminated(&content)?;

        let return_type = input.parse()?;

        let body_content;
        let brace_token = syn::braced!(body_content in input);
        // 将大括号内的所有内容解析为一个 TokenStream2
        let query: TokenStream2 = body_content.parse()?;

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
    let parsed_input = parse_macro_input!(input as QueryInput);

    let visibility = &parsed_input.visibility;
    let fn_name = &parsed_input.name;
    let args = &parsed_input.args;
    let query_str = parsed_input.query;

    // let sql_query_as_string = parsed_input.query.to_string();
    // let query_lit_str_token = LitStr::new(&sql_query_as_string, parsed_input.query.span());
    // let query_lit_str_token = sqlited::sql_str!(parsed_input.query);

    let return_type_info = extract_return_type_info(&parsed_input.return_type);
    let (model_type, is_vec, is_tuple, is_unit) = (
        return_type_info.model_type,
        return_type_info.is_vec,
        return_type_info.is_tuple,
        return_type_info.is_unit,
    );
let method_params_with_types = generate_method_params_with_types(args);
    let param_idents = generate_param_idents(args);

    let params_holder_construction = quote! {
        let __params_holder_result: sqlited::Result<sqlited::StaticParamsHolder> = (|| {
            let mut __rusqlite_params_vec: Vec<Box<dyn sqlited::rq::ToSql>> = Vec::new();
            #(
                {
                    // 使用 sqlited::ToSql::to_sql 避免作用域问题
                    let to_sql_output = sqlited::ToSql::to_sql(&#param_idents)
                        .map_err(|e| sqlited::SqlitedError::ToSqlConversionError(Box::new(e)))?;
                    let value_for_rusqlite: sqlited::rq::types::Value = match to_sql_output {
                        sqlited::rq::types::ToSqlOutput::Borrowed(val_ref) => match val_ref {
                            sqlited::rq::types::ValueRef::Null => sqlited::rq::types::Value::Null,
                            sqlited::rq::types::ValueRef::Integer(i) => sqlited::rq::types::Value::Integer(i),
                            sqlited::rq::types::ValueRef::Real(r) => sqlited::rq::types::Value::Real(r),
                            sqlited::rq::types::ValueRef::Text(t) => sqlited::rq::types::Value::Text(String::from_utf8_lossy(t).into_owned()),
                            sqlited::rq::types::ValueRef::Blob(b) => sqlited::rq::types::Value::Blob(b.to_vec()),
                        },
                        sqlited::rq::types::ToSqlOutput::Owned(val) => val,
                        _ => sqlited::rq::types::Value::Null, // Default or error
                    };
                    __rusqlite_params_vec.push(Box::new(value_for_rusqlite));
                }
            )*
            Ok(sqlited::StaticParamsHolder::new(__rusqlite_params_vec))
        })();
        let __params_holder = __params_holder_result?;
    };

    let generated_code = if is_unit {
        quote! {
            #visibility fn #fn_name<'s_self>(self: &'s_self Self, #method_params_with_types) -> sqlited::Result<()> {
                #params_holder_construction
                let query = sqlited::sql_str!(#query_str);
                self.get_conn()?.execute2(query, __params_holder)?;
                Ok(())
            }
        }
    } else if is_vec {
        if is_tuple {
            let tuple_elements = extract_tuple_elements(&model_type);
            let indices = (0..tuple_elements.len()).map(syn::Index::from);
            quote! {
                #visibility fn #fn_name<'s_self>(self: &'s_self Self, #method_params_with_types) -> sqlited::Result<Vec<#model_type>> {
                    #params_holder_construction
                    let query = sqlited::sql_str!(#query_str);
                    self.get_conn()?.query2(query, __params_holder, |row: &sqlited::row::Row| {
                        Ok((
                            #(row.get::<_, #tuple_elements>(#indices)?),*
                        ))
                    })
                }
            }
        } else if is_primitive_type(&model_type) {
            quote! {
                #visibility fn #fn_name<'s_self>(self: &'s_self Self, #method_params_with_types) -> sqlited::Result<Vec<#model_type>> {
                    #params_holder_construction
                    let query = sqlited::sql_str!(#query_str);
                    self.get_conn()?.query2(query, __params_holder, |row: &sqlited::row::Row| row.get::<_, #model_type>(0))
                }
            }
        } else { // Collection of structs
            quote! {
                #visibility fn #fn_name<'s_self>(self: &'s_self Self, #method_params_with_types) -> sqlited::Result<Vec<#model_type>> {
                    #params_holder_construction
                    let query = sqlited::sql_str!(#query_str);
                    self.get_conn()?.query2(query, __params_holder, #model_type::from_row)
                }
            }
        }
    } else { // Single item
        if is_tuple {
            let tuple_elements = extract_tuple_elements(&model_type);
            let indices = (0..tuple_elements.len()).map(syn::Index::from);
            quote! {
                #visibility fn #fn_name<'s_self>(self: &'s_self Self, #method_params_with_types) -> sqlited::Result<#model_type> {
                    #params_holder_construction
                    let query = sqlited::sql_str!(#query_str);
                    self.get_conn()?.query_row2(query, __params_holder, |row: &sqlited::row::Row| {
                        Ok((
                            #(row.get::<_, #tuple_elements>(#indices)?),*
                        ))
                    })
                }
            }
        } else if is_primitive_type(&model_type) {
            quote! {
                #visibility fn #fn_name<'s_self>(self: &'s_self Self, #method_params_with_types) -> sqlited::Result<#model_type> {
                    #params_holder_construction
                    let query = sqlited::sql_str!(#query_str);
                    self.get_conn()?.query_row2(query, __params_holder, |row: &sqlited::row::Row| row.get::<_, #model_type>(0))
                }
            }
        } else { // Single struct
            quote! {
                #visibility fn #fn_name<'s_self>(self: &'s_self Self, #method_params_with_types) -> sqlited::Result<#model_type> {
                    #params_holder_construction
                    let query = sqlited::sql_str!(#query_str);
                    self.get_conn()?.query_row2(query, __params_holder, #model_type::from_row)
                }
            }
        }
    };

    generated_code.into()
}

fn generate_method_params_with_types(args: &Punctuated<FnArg, Comma>) -> TokenStream2 {
    quote! { #args }
}

fn generate_param_idents(args: &Punctuated<FnArg, Comma>) -> Vec<Ident> {
    args.iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .collect()
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
    let type_str = model_type.to_string().replace(" ", "");

    // Common primitive type names
    let primitives = [
        "i8", "i16", "i32", "i64",
        "u8", "u16", "u32", "u64",
        "f32", "f64",
        "bool",
        "char",
        "String", "&str",
        "isize", "usize",
        // 可以根据需要添加其他被视为基本类型的，例如日期/时间类型或 BLOB
        "Vec<u8>", // 精确匹配 Vec<u8>
        // 如果你的 UtcDateTime 或 Timestamp 可以直接从单个 SQL 值转换，也可以加入
        "UtcDateTime",
        "Timestamp",
    ];

    // 进行精确匹配
    primitives.iter().any(|&prim| type_str == prim)
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
