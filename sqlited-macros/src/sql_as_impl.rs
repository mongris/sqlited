use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Ident, LitStr, Meta, Result, Variant,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

use crate::utils::find_closest_match;

struct SqlAsArgs {
    style: Ident,
}

impl Parse for SqlAsArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let style = input.parse()?;
        Ok(SqlAsArgs { style })
    }
}

// 提取变体的属性信息
struct VariantAttribute {
    ident: syn::Ident,
    custom_value: Option<String>,
}
// 从变体属性中提取 sql_as_value (用于 string 风格)
fn extract_sql_as_string_value(variant: &Variant) -> Option<String> {
    for attr in &variant.attrs {
        if attr.path().is_ident("sql_as_value") {
            match &attr.meta {
                Meta::List(list) => {
                    if let Ok(lit) = list.parse_args::<LitStr>() {
                        return Some(lit.value());
                    }
                }
                _ => {}
            }
        }
    }
    None
}

// 新增：从变体属性中提取 sql_as_value (用于 int 风格)
fn extract_sql_as_int_literal(variant: &Variant) -> std::result::Result<Option<syn::LitInt>, syn::Error> {
    for attr in &variant.attrs {
        if attr.path().is_ident("sql_as_value") {
            match &attr.meta {
                Meta::List(list) => {
                    // 尝试解析为 LitInt
                    if let Ok(lit_int) = list.parse_args::<syn::LitInt>() {
                        return Ok(Some(lit_int));
                    }
                    // 如果不是 LitInt，但属性存在，则报错
                    let parsed_lit_for_error = list.parse_args::<syn::Lit>()?;
                    return Err(syn::Error::new(parsed_lit_for_error.span(), "Expected an integer literal for sql_as_value when using int style"));
                }
                _ => { // 例如 #[sql_as_value = 123]
                    return Err(syn::Error::new(attr.meta.path().get_ident().unwrap().span(), "Expected #[sql_as_value(...)] format for integer values"));
                }
            }
        }
    }
    Ok(None)
}

// 处理枚举所有变体的属性 (用于 string 风格)
fn process_enum_variants_for_string_style(
    variants: &syn::punctuated::Punctuated<Variant, syn::token::Comma>,
) -> Vec<VariantAttribute> {
    variants
        .iter()
        .map(|variant| {
            let ident = variant.ident.clone();
            let custom_value = extract_sql_as_string_value(variant);

            VariantAttribute {
                ident,
                custom_value,
            }
        })
        .collect()
}

pub fn sql_as(attr: TokenStream, input: TokenStream) -> TokenStream {
    // 解析属性参数
    let args = parse_macro_input!(attr as SqlAsArgs);

    // 获取序列化风格(json, binary, string, int)
    let style = &args.style;
    let style_str = style.to_string();

    // 解析输入为 DeriveInput
    let input = parse_macro_input!(input as DeriveInput);

    // 获取类型名称、可见性、泛型参数等
    let type_name = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;

    match &input.data {
        Data::Struct(data_struct) => {

            // 定义有效的序列化风格
            let valid_styles = ["json", "jsonb", "binary", "borsh"];

            // 验证风格是否有效
            if !valid_styles.contains(&style_str.as_str()) {
                let suggestion = find_closest_match(&style_str, &valid_styles);
                let error_msg = if let Some(suggested) = suggestion {
                    format!(
                        "Invalid serialization style '{}'. Did you mean '{}'?",
                        style_str, suggested
                    )
                } else {
                    format!(
                        "Invalid serialization style '{}'. Valid styles are: {}",
                        style_str,
                        valid_styles.join(", ")
                    )
                };

                return syn::Error::new(style.span(), error_msg)
                    .to_compile_error()
                    .into();
            }

            // 从原始结构体中获取字段
            let fields = &data_struct.fields;

            // 保留原始结构体属性（除了sql_as本身）
            let struct_attrs = input
                .attrs
                .iter()
                .filter(|attr| !attr.path().is_ident("sql_as"))
                .collect::<Vec<_>>();

            // 根据字段类型构建不同的结构体定义
            let struct_def = match fields {
                Fields::Named(named_fields) => {
                    let fields = named_fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        let vis = &f.vis;
                        let attrs = f
                            .attrs
                            .iter()
                            .filter(|attr| !attr.path().is_ident("sql_as"))
                            .collect::<Vec<_>>();

                        quote! {
                            #(#attrs)*
                            #vis #name: #ty
                        }
                    });

                    quote! {
                        struct #type_name #generics {
                            #(#fields),*
                        }
                    }
                }

                Fields::Unnamed(unnamed_fields) => {
                    let fields = unnamed_fields
                        .unnamed
                        .iter()
                        .map(|f| {
                            let ty = &f.ty;
                            let vis = &f.vis;
                            let attrs = f
                                .attrs
                                .iter()
                                .filter(|attr| !attr.path().is_ident("sql_as"))
                                .collect::<Vec<_>>();

                            quote! {
                                #(#attrs)*
                                #vis #ty
                            }
                        })
                        .collect::<Vec<_>>();

                    quote! {
                        struct #type_name #generics (#(#fields),*);
                    }
                }
                Fields::Unit => {
                    quote! {
                        struct #type_name #generics;
                    }
                }
            };

            // 检查是否是新类型模式
            let is_newtype = if let Fields::Unnamed(unnamed_fields) = fields {
                unnamed_fields.unnamed.len() == 1
            } else {
                false
            };

            // 如果是新类型，生成 Deref, DerefMut 和 From 实现
            let (deref_impl, from_impl) = if is_newtype {
                if let Fields::Unnamed(unnamed_fields) = fields {
                    let inner_type = &unnamed_fields.unnamed.first().unwrap().ty;

                    let deref = quote! {
                        impl #generics std::ops::Deref for #type_name #generics {
                            type Target = #inner_type;

                            #[inline(always)]
                            fn deref(&self) -> &Self::Target {
                                &self.0
                            }
                        }

                        impl #generics std::ops::DerefMut for #type_name #generics {
                            #[inline(always)]
                            fn deref_mut(&mut self) -> &mut Self::Target {
                                &mut self.0
                            }
                        }
                    };

                    // Add From implementations for newtype
                    let from = quote! {
                        impl #generics From<#type_name #generics> for #inner_type {
                            #[inline(always)]
                            fn from(val: #type_name #generics) -> Self {
                                val.0
                            }
                        }

                        impl #generics From<#inner_type> for #type_name #generics {
                            #[inline(always)]
                            fn from(val: #inner_type) -> Self {
                                #type_name(val)
                            }
                        }
                    };

                    (deref, from) // Return both Deref/DerefMut and From impls
                } else {
                    (quote! {}, quote! {}) // Should not happen if is_newtype is true
                }
            } else {
                (quote! {}, quote! {}) // Not a newtype
            };

            // 根据 style_str 选择派生的 traits
            let derive_traits = if style_str == "borsh" {
                quote! { #[derive(Default, Clone, Debug, PartialEq, sqlited::borsh::BorshSerialize, sqlited::borsh::BorshDeserialize)] }
            } else {
                quote! { #[derive(Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)] }
            };

            // 生成扩展代码
            let expanded = if is_newtype {
                // 对于新类型，添加 #[repr(transparent)]
                quote! {
                    #(#struct_attrs)*
                    #derive_traits
                    #[repr(transparent)]  // 为新类型添加 repr(transparent) 属性
                    #vis #struct_def

                    // 如果是新类型，添加 Deref, DerefMut 和 From 实现
                    #deref_impl
                    #from_impl

                    sqlited::sqld!(#style #type_name);
                }
            } else {
                // 对于非新类型，使用标准实现
                quote! {
                    #(#struct_attrs)*
                    #derive_traits
                    #vis #struct_def

                    sqlited::sqld!(#style #type_name);
                }
            };

            expanded.into()
        }

        Data::Enum(data_enum) => {

            // 定义有效的序列化风格
            let valid_styles = ["json", "jsonb", "binary", "string", "int", "borsh"];

            // 验证风格是否有效
            if !valid_styles.contains(&style_str.as_str()) {
                let suggestion = find_closest_match(&style_str, &valid_styles);
                let error_msg = if let Some(suggested) = suggestion {
                    format!(
                        "Invalid serialization style '{}'. Did you mean '{}'?",
                        style_str, suggested
                    )
                } else {
                    format!(
                        "Invalid serialization style '{}'. Valid styles are: {}",
                        style_str,
                        valid_styles.join(", ")
                    )
                };

                return syn::Error::new(style.span(), error_msg)
                    .to_compile_error()
                    .into();
            }

            // 获取枚举变体
            let variants = &data_enum.variants;

            // 重建枚举变体，保留原始属性
            let variant_definitions = variants.iter().map(|v| {
                let name = &v.ident;
                let fields = &v.fields;

                // 保留变体的原始属性（除了sql_as_value）
                let var_attrs = v
                    .attrs
                    .iter()
                    .filter(|attr| !attr.path().is_ident("sql_as_value"))
                    .collect::<Vec<_>>();

                match fields {
                    Fields::Named(named_fields) => {
                        let fields = named_fields.named.iter().map(|f| {
                            let field_name = &f.ident;
                            let field_type = &f.ty;
                            let field_vis = &f.vis;

                            quote! {
                                #field_vis #field_name: #field_type
                            }
                        });

                        quote! {
                            #(#var_attrs)*
                            #name {
                                #(#fields),*
                            }
                        }
                    }
                    Fields::Unnamed(unnamed_fields) => {
                        let fields = unnamed_fields.unnamed.iter().map(|f| {
                            let field_type = &f.ty;
                            let field_vis = &f.vis;

                            quote! {
                                #field_vis #field_type
                            }
                        });

                        quote! {
                            #(#var_attrs)*
                            #name(#(#fields),*)
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            #(#var_attrs)*
                            #name
                        }
                    }
                }
            });

            // 保留原始枚举属性（除了sql_as本身）
            let enum_attrs = input
                .attrs
                .iter()
                .filter(|attr| !attr.path().is_ident("sql_as"))
                .collect::<Vec<_>>();

            // 根据 style_str 选择派生的 traits
            let derive_traits_for_enum = if style_str == "borsh" {
                 // 对于 borsh 枚举，Default 可能需要手动实现或根据具体情况决定
                quote! { #[derive(Default, Clone, Debug, PartialEq, sqlited::borsh::BorshSerialize, sqlited::borsh::BorshDeserialize)] }
            } else if style_str == "string" || style_str == "int" {
                // string 和 int 风格的枚举通常是 C-like，可以安全地派生 Default, Serialize, Deserialize
                quote! { #[derive(Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)] }
            } else { // json, jsonb, binary
                quote! { #[derive(Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)] }
            };

            // 特别处理 "string" 风格的枚举
            if style_str == "string" {
                // 生成字符串序列化的变体映射
                let variant_attributes = process_enum_variants_for_string_style(variants); // 修改调用

                let variant_mappings = variant_attributes.iter().map(|variant_attr| {
                    let variant_name = &variant_attr.ident;
                    let value = variant_attr
                        .custom_value
                        .clone()
                        .unwrap_or_else(|| variant_name.to_string());

                    quote! {
                        #variant_name => #value
                    }
                });

                // 生成枚举的扩展代码，附带字符串映射
                let expanded = quote! {
                    #(#enum_attrs)*
                    #derive_traits_for_enum
                    #[repr(u8)]
                    #vis enum #type_name #generics {
                        #(#variant_definitions),*
                    }

                    sqlited::sqld!(
                        enum #type_name {
                            #(#variant_mappings),*
                        }
                    );
                };

                expanded.into()
            } else if style_str == "int" { // 新增对 "int" 风格的处理
                let mut variant_mappings_for_int = Vec::new();
                let mut next_implicit_value: i64 = 0;

                for variant in variants {
                    let variant_name = &variant.ident;
                    let current_value_literal: proc_macro2::Literal;

                    match extract_sql_as_int_literal(variant) {
                        Ok(Some(lit_int)) => {
                            match lit_int.base10_parse::<i64>() {
                                Ok(val) => {
                                    current_value_literal = proc_macro2::Literal::i64_unsuffixed(val);
                                    next_implicit_value = val + 1;
                                }
                                Err(e) => {
                                    return syn::Error::new(lit_int.span(), format!("Invalid integer in sql_as_value: {}", e))
                                        .to_compile_error()
                                        .into();
                                }
                            }
                        }
                        Ok(None) => {
                            // 没有 sql_as_value，使用隐式值
                            current_value_literal = proc_macro2::Literal::i64_unsuffixed(next_implicit_value);
                            next_implicit_value += 1;
                        }
                        Err(e) => {
                            return e.to_compile_error().into();
                        }
                    }
                    variant_mappings_for_int.push(quote! { #variant_name => #current_value_literal });
                }

                let expanded = quote! {
                    #(#enum_attrs)*
                    // 对于 int 枚举，Default, Serialize, Deserialize 可能需要用户根据具体情况调整或自定义
                    #derive_traits_for_enum
                    #vis enum #type_name #generics {
                        #(#variant_definitions),*
                    }

                    sqlited::sqld!(
                        enum_int #type_name { // 使用 "enum_int" 作为新的关键字
                            #(#variant_mappings_for_int),*
                        }
                    );
                };
                expanded.into()
            } else {
                // json, jsonb, binary, borsh (borsh 会在这里处理，因为它不匹配 string 或 int)
                let expanded = quote! {
                    #(#enum_attrs)*
                    #derive_traits_for_enum
                    #[repr(u8)]
                    #vis enum #type_name #generics {
                        #(#variant_definitions),*
                    }

                    sqlited::sqld!(#style #type_name);
                };

                expanded.into()
            }
        }
        _ => {
            // 返回不支持项类型的错误
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "sql_as can only be applied to structs or enums",
            )
            .to_compile_error()
            .into();
        }
    }
}
