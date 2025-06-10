use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::{braced, parse::{Parse, ParseStream}, Expr, Ident, Result, Token, Type};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

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
    
    // Collect field names
    let field_names: Vec<_> = fields.iter().map(|kv| &kv.key).collect();

    // Create temporary variable identifiers
    let temp_vars: Vec<_> = field_names.iter().map(|name| {
        format_ident!("_temp_{}", name)
    }).collect();

    // Generate Rc bindings with potential literal conversion
    let rc_bindings = fields.iter().map(|kv| {
        let name = &kv.key;
        let value_expr = &kv.value;
        let temp_var = format_ident!("_temp_{}", name);

        // Check the expression and generate conversion code if it's a string literal
        // or Some("string literal")
        let converted_expr = match value_expr {
            Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => {
                quote_spanned! {lit_str.span()=> #lit_str.to_string() }
            }
            Expr::Call(syn::ExprCall { func, args, .. }) => {
                let is_some = match func.as_ref() {
                    Expr::Path(expr_path) => expr_path.path.is_ident("Some"),
                    _ => false,
                };
                if is_some && args.len() == 1 {
                    if let Some(Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(inner_lit_str), .. })) = args.first() {
                        quote_spanned! {inner_lit_str.span()=> Some(#inner_lit_str.to_string()) }
                    } else {
                        quote! { #value_expr }
                    }
                } else {
                    quote! { #value_expr }
                }
            }
            _ => {
                quote! { #value_expr }
            }
        };

        // Wrap the (potentially converted) expression in Rc::new
        // 使用 value_expr 的 span 来定位可能的类型错误源头
        quote_spanned! {value_expr.span()=>
            let #temp_var = std::rc::Rc::new(#converted_expr);
        }
    });

    // Generate the type check block using quote_spanned!
    let type_check_assignments = fields.iter().map(|kv| {
        let name = &kv.key;
        let value_expr = &kv.value; // Use the original value expression for span
        let temp_var = format_ident!("_temp_{}", name);

        // Use the span of the user-provided value expression for the .into() call
        // 这样，如果 .into() 失败，错误会指向用户输入的具体值
        quote_spanned! {value_expr.span() =>
            _model.#name = (*#temp_var).clone().into();
        }
    });

    // 生成结果代码
    Ok(quote! {
        {
            // 使用 Rc 包装所有值，确保可以多次引用
            #( #rc_bindings )*

            #[allow(unused_variables, unreachable_code, unused_must_use)]
            {
                // The type check block now uses spans from user input
                if false {
                    let mut _model = <#model_type>::default();
                    #( #type_check_assignments )* // Use the generated assignments
                }
            }
            
            // 创建一个新的 WithoutId 实例
            let mut result = sqlited::WithoutId::<#model_type>::new();
            
            // 设置每个字段的值
            #(
                {
                    let value = (*#temp_vars).clone();
                    let boxed_value: Box<dyn sqlited::ToSql> = Box::new(value);
                    result.inner.insert(
                        stringify!(#field_names).to_lowercase(),
                        boxed_value
                    );
                }
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