use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, 
    Token, Visibility, Ident, FnArg, ReturnType, Type, Block, braced, 
    punctuated::Punctuated,
    token::Comma,
    Result as SynResult
};

// 查询宏输入的解析结构
struct QueryInput {
    visibility: Visibility,      // 函数可见性 (pub, pub(crate) 等)
    fn_token: Token![fn],        // 'fn' 标记
    name: Ident,                 // 函数名
    paren_token: syn::token::Paren,
    args: Punctuated<FnArg, Comma>, // 函数参数
    return_type: ReturnType,     // 返回类型
    brace_token: syn::token::Brace,
    query: TokenStream2,               // SQL 查询语句
}

impl Parse for QueryInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let visibility = input.parse()?;
        let fn_token = input.parse()?;
        let name = input.parse()?;
        
        // 解析参数列表
        let content;
        let paren_token = syn::parenthesized!(content in input);
        let args = Punctuated::parse_terminated(&content)?;
        
        // 解析返回类型
        let return_type = input.parse()?;
        
        // 解析查询体
        let body;
        let brace_token = syn::braced!(body in input);
        
        // 收集SQL查询字符串
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

/// 实现 query! 宏处理函数
pub fn query_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as QueryInput);
    
    // 提取函数的各个部分
    let visibility = &input.visibility;
    let fn_name = &input.name;
    let args = &input.args;
    let query_str = &input.query;
    
    // 从返回类型提取模型类型
    let return_model_type = extract_return_model_type(&input.return_type);
    
    // 构建参数列表，用于方法签名
    let method_params = generate_method_params(args);
    
    // 构建参数名列表，用于 sql_params! 宏
    let param_names = args.iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        });
    
    // 生成结果代码
    let result = quote! {
        #visibility fn #fn_name(#method_params) -> sqlited::Result<#return_model_type> {
            let params = sqlited::sql_params!(<#return_model_type> {
                #(#param_names,)*
            });
            
            let query = sqlited::sql!(
                #query_str,
                &params
            );
            
            query.query_row(self.raw_connection(), #return_model_type::from_row)
        }
    };
    
    result.into()
}

// 从返回类型中提取模型类型
fn extract_return_model_type(return_type: &ReturnType) -> TokenStream2 {
    match return_type {
        ReturnType::Type(_, ty) => {
            match &**ty {
                Type::Path(type_path) => {
                    // 检查返回类型是否是 Result<ModelType>
                    if let Some(segment) = type_path.path.segments.last() {
                        if segment.ident == "Result" || segment.ident == "anyhow::Result" || segment.ident == "sqlited::Result" {
                            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(arg) = args.args.first() {
                                    return arg.to_token_stream();
                                }
                            }
                        } else {
                            // 直接返回类型
                            return type_path.to_token_stream();
                        }
                    }
                },
                _ => {}
            }
            
            // 如果无法解析，返回默认的 TokenStream
            quote! { () }
        },
        _ => quote! { () }
    }
}

// 生成方法参数列表
fn generate_method_params(args: &Punctuated<FnArg, Comma>) -> TokenStream2 {
    let mut method_params = quote! { &self };
    
    if !args.is_empty() {
        method_params = quote! { &self, #args };
    }
    
    method_params
}