use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, Token, Type, Ident, Result, braced};
use syn::punctuated::Punctuated;

// 键值对结构
struct KeyValue {
    key: Ident,
    #[allow(dead_code)]
    // 冒号令牌可选，用于支持字段简写
    colon_token: Option<Token![:]>,
    // 值表达式
    value: syn::Expr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        // 解析字段名标识符
        let key: Ident = input.parse()?;
        
        // 检查是否存在冒号，用于确定是否为字段简写形式
        if input.peek(Token![:]) {
            // 标准形式: field: value
            let colon_token: Token![:] = input.parse()?;
            let value: syn::Expr = input.parse()?;
            
            Ok(KeyValue {
                key,
                colon_token: Some(colon_token),
                value,
            })
        } else {
            // 字段简写形式: field
            // 创建一个与字段名相同的表达式
            let key_str = key.to_string();
            let value = syn::parse_str::<syn::Expr>(&key_str)
                .expect("无法将字段名转换为表达式");
                
            Ok(KeyValue {
                key,
                colon_token: None,
                value,
            })
        }
    }
}

// 使用 do_parse 函数作为自定义解析器
fn do_parse(input: ParseStream) -> Result<TokenStream2> {
    // 解析模型类型
    let model_type = if input.peek(Token![<]) {
        // <Model> 格式
        let _lt_token: Token![<] = input.parse()?;
        let model_type: Type = input.parse()?;
        let _gt_token: Token![>] = input.parse()?;
        model_type
    } else {
        // Model 格式
        input.parse()?
    };
    
    // 解析大括号内容
    let content;
    let _brace_token = braced!(content in input);
    
    // 解析所有键值对
    let fields = Punctuated::<KeyValue, Token![,]>::parse_terminated(&content)?;
    
    // 收集字段名和值
    let field_names: Vec<_> = fields.iter().map(|kv| &kv.key).collect();
    let field_values: Vec<_> = fields.iter().map(|kv| &kv.value).collect();
    
    // 生成结果代码
    Ok(quote! {
        {
            #[allow(unused_variables, unreachable_code, unused_must_use)]
            {
                // 创建一个类型检查块，它不会在运行时执行
                if false {
                    // 使用模型实例进行类型检查
                    let mut _model = <#model_type>::default();
                    
                    // 检查每个字段
                    #(
                        // 尝试对字段赋值，通过 Rust 的类型系统检查类型兼容性
                        // 如果类型不兼容，编译时会报错并显示期望的类型
                        _model.#field_names = #field_values;
                    )*
                }
            }
            
            // 创建一个新的 WithoutId 实例
            let mut result = sqlited::WithoutId::<#model_type>::new();
            
            // 设置每个字段的值
            #(
                result.set(stringify!(#field_names), #field_values);
            )*
            
            // 创建静态参数持有者
            let ordered_field_names = vec![#(stringify!(#field_names).to_string()),*];
            result.create_static_params_for_fields(&ordered_field_names)
        }
    })
}

pub(crate) fn sql_params(input: TokenStream) -> TokenStream {
    // 使用 with 语法将 do_parse 作为解析器传入
    syn::parse_macro_input!(input with do_parse).into()
}