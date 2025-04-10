use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::LitStr;

use crate::sql_no_quote_impl::{parse_sql_no_quotes, process_sql};
use crate::sql_params_impl::sql_params;

pub fn sql(input: TokenStream) -> TokenStream {
    // 解析输入，提取SQL和参数
    let (sql_string, params, span) = parse_sql_no_quotes(input);

    // 处理并验证SQL语句
    let validated_sql = match process_sql(&sql_string, span) {
        Ok(result) => result,
        Err(error) => return error,
    };

    // 生成SQL字符串字面量
    let sql_lit = LitStr::new(&validated_sql, proc_macro2::Span::call_site());

    // 处理参数
    match params {
        // 有参数的情况
        Some(params_tokens) => {
            let params_ts = TokenStream2::from(params_tokens.clone());
            // 尝试检测&params格式
            if let Ok(expr) = syn::parse2::<syn::Expr>(params_ts.clone()) {
              if let syn::Expr::Reference(_) = expr {
                  // 处理&params格式
                  return quote! {
                      {
                          crate::SqlQuery {
                              query: #sql_lit.to_string(),
                              params:(#params_ts).to_boxed_vec()
                          }
                      }
                  }.into();
              }
          }
          let params = TokenStream2::from(sql_params(params_tokens));
            quote! {
                {
                    // 创建包含SQL和参数的Query对象
                    crate::SqlQuery {
                        query: #sql_lit.to_string(),
                        params: (#params).to_boxed_vec()
                    }
                }
            }
            .into()
        }
        // 无参数的情况
        None => {
            // 生成仅含SQL的Query对象
            quote! {
                {

                    // 创建只包含SQL的Query对象（空参数）
                    crate::SqlQuery {
                        query: #sql_lit.to_string(),
                        params: Vec::new()
                    }
                }
            }
            .into()
        }
    }
}
