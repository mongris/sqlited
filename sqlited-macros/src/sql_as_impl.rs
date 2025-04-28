use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Data, DeriveInput, Fields, Ident, LitStr, Meta, Result, Variant};
use quote::quote;

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

// 从变体属性中提取 sql_as_value
fn extract_sql_as_value(variant: &Variant) -> Option<String> {
    for attr in &variant.attrs {
        if attr.path().is_ident("sql_as_value") {
            match &attr.meta {
                Meta::List(list) => {
                    if let Ok(lit) = list.parse_args::<LitStr>() {
                        return Some(lit.value());
                    }
                },
                _ => {}
            }
        }
    }
    None
}

// 处理枚举所有变体的属性
fn process_enum_variants(variants: &syn::punctuated::Punctuated<Variant, syn::token::Comma>) -> Vec<VariantAttribute> {
    variants.iter().map(|variant| {
        let ident = variant.ident.clone();
        let custom_value = extract_sql_as_value(variant);
        
        VariantAttribute {
            ident,
            custom_value,
        }
    }).collect()
}

pub fn sql_as(attr: TokenStream, input: TokenStream) -> TokenStream {
    // 解析属性参数
    let args = parse_macro_input!(attr as SqlAsArgs);
    
    // 获取序列化风格（json, binary, string）
    let style = &args.style;
    let style_str = style.to_string();
    
    // 定义有效的序列化风格
    let valid_styles = ["json", "binary", "string"];
    
    // 验证风格是否有效
    if !valid_styles.contains(&style_str.as_str()) {
        let suggestion = find_closest_match(&style_str, &valid_styles);
        let error_msg = if let Some(suggested) = suggestion {
            format!("Invalid serialization style '{}'. Did you mean '{}'?", style_str, suggested)
        } else {
            format!("Invalid serialization style '{}'. Valid styles are: {}", 
                style_str, valid_styles.join(", "))
        };
        
        return syn::Error::new(style.span(), error_msg).to_compile_error().into();
    }
    
    // 解析输入为 DeriveInput
    let input = parse_macro_input!(input as DeriveInput);
    
    // 获取类型名称、可见性、泛型参数等
    let type_name = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;
    
    match &input.data {
        Data::Struct(data_struct) => {
            // 从原始结构体中获取字段
            let fields = &data_struct.fields;
            
            // 重新构建结构体和提取字段定义
            let field_definitions = match fields {
                Fields::Named(named_fields) => {
                    let fields = named_fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        let vis = &f.vis;
                        // 保留字段的原始属性（除了sql_as相关的）
                        let attrs = f.attrs.iter()
                            .filter(|attr| !attr.path().is_ident("sql_as"))
                            .collect::<Vec<_>>();
                        
                        quote! {
                            #(#attrs)*
                            #vis #name: #ty
                        }
                    });
                    
                    quote! {
                        {
                            #(#fields),*
                        }
                    }
                },
                Fields::Unnamed(unnamed_fields) => {
                    let fields = unnamed_fields.unnamed.iter().enumerate().map(|(i, f)| {
                        // let index = syn::Index::from(i);
                        let ty = &f.ty;
                        let vis = &f.vis;
                        // 保留字段的原始属性（除了sql_as相关的）
                        let attrs = f.attrs.iter()
                            .filter(|attr| !attr.path().is_ident("sql_as"))
                            .collect::<Vec<_>>();
                        
                        quote! {
                            #(#attrs)*
                            #vis #ty
                        }
                    });
                    
                    quote! {
                        (
                            #(#fields),*
                        )
                    }
                },
                Fields::Unit => {
                    quote! {}
                }
            };
            
            // 保留原始结构体属性（除了sql_as本身）
            let struct_attrs = input.attrs.iter()
                .filter(|attr| !attr.path().is_ident("sql_as"))
                .collect::<Vec<_>>();
            
            // 生成扩展代码
            let expanded = quote! {
                #(#struct_attrs)*
                #[derive(Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
                #vis struct #type_name #generics #field_definitions
                
                sqlited::sqld!(#style #type_name);
            };
            
            expanded.into()
        },
        Data::Enum(data_enum) => {
            // 获取枚举变体
            let variants = &data_enum.variants;
            
            // 处理枚举变体属性
            let variant_attributes = process_enum_variants(variants);
            
            // 重建枚举变体，保留原始属性
            let variant_definitions = variants.iter().map(|v| {
                let name = &v.ident;
                let fields = &v.fields;
                
                // 保留变体的原始属性（除了sql_as_value）
                let var_attrs = v.attrs.iter()
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
                    },
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
                    },
                    Fields::Unit => {
                        quote! {
                            #(#var_attrs)*
                            #name
                        }
                    }
                }
            });
            
            // 保留原始枚举属性（除了sql_as本身）
            let enum_attrs = input.attrs.iter()
                .filter(|attr| !attr.path().is_ident("sql_as"))
                .collect::<Vec<_>>();
            
            // 特别处理 "string" 风格的枚举
            if style_str == "string" {
                // 生成字符串序列化的变体映射
                let variant_mappings = variant_attributes.iter().map(|variant_attr| {
                    let variant_name = &variant_attr.ident;
                    let value = variant_attr.custom_value.clone()
                        .unwrap_or_else(|| variant_name.to_string());
                    
                    quote! {
                        #variant_name => #value
                    }
                });
                
                // 生成枚举的扩展代码，附带字符串映射
                let expanded = quote! {
                    #(#enum_attrs)*
                    #[derive(Default, Copy, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
            } else {
                // 对于 json 和 binary，使用标准方法
                let expanded = quote! {
                    #(#enum_attrs)*
                    #[derive(Default, Copy, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
                    #vis enum #type_name #generics {
                        #(#variant_definitions),*
                    }
                    
                    sqlited::sqld!(#style #type_name);
                };
                
                expanded.into()
            }
        },
        _ => {
            // 返回不支持项类型的错误
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "sql_as can only be applied to structs or enums"
            ).to_compile_error().into();
        }
    }
}