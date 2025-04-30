use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::LitStr;

use crate::{sql_check_impl, utils::convert_to_snake_name};


// struct ParenthesesState {
//     depth: usize,
// }

// static KEYWORDS: [&'static str; 11] = [
//     "LIMIT", "ORDER", "GROUP", "BY", "DESC", "ASC", "WHERE", "FROM", "SELECT", "JOIN", "HAVING",
// ];

// // 将TokenStream转换为SQL字符串
// fn tokens_to_sql(input: TokenStream, state: &mut ParenthesesState) -> (String, Span) {
//     let mut sql = String::new();
//     let mut first_span = None;
//     let mut last_was_ident = false;
//     // let mut last_was_number = false;
//     let mut last_was_keyword = false;
    
//     // 将标记流转换为SQL字符串
//     for token in input {
//         match token {
//             proc_macro::TokenTree::Ident(ident) => {
//                 if !sql.is_empty() && !sql.ends_with('(') && !sql.ends_with('.') {
//                     sql.push(' ');
//                 }

//                 let ident_str = ident.to_string();
//                 sql.push_str(&ident_str);

//                 let upper_ident = ident_str.to_uppercase();
//                 last_was_keyword = KEYWORDS.contains(&upper_ident.as_str());
                
//                 if first_span.is_none() {
//                     first_span = Some(Span::call_site());
//                 }
                
//                 last_was_ident = true;
//             },
//             proc_macro::TokenTree::Punct(punct) => {
//                 let punct_str = punct.to_string();
                
//                 // IMPORTANT CHANGE: Only treat commas as SQL parameter separators 
//                 // if they're outside ALL parentheses AND not in a SELECT clause
//                 if punct_str == "," && state.depth == 0 && !sql.trim_start().to_uppercase().contains("SELECT") {
//                     // If it's a comma parameter separator outside of SELECT, stop parsing SQL part
//                     break;
//                 } 
                
//                 // Special handling for commas within SELECT statements - add them normally
//                 if punct_str == "," {
//                     if !sql.is_empty() && !sql.ends_with(' ') {
//                         sql.push(' ');
//                     }
//                     sql.push_str(&punct_str);
//                     sql.push(' '); // Add space after comma for better formatting
//                 }
//                 // 特殊情况：处理 ? 后紧跟数字的情况（编号参数）
//                 else if punct_str == "?" {
//                     sql.push_str(&punct_str);
//                     last_was_ident = false;
//                     last_was_keyword = false;
//                 } else if punct_str == "=" || punct_str == "." || punct_str == "*" || punct_str == ";" {
//                     // 这些标点符号不需要前后空格
//                     sql.push_str(&punct_str);
//                     last_was_ident = false;
//                     last_was_keyword = false;
//                 } else {
//                     // 其他标点符号添加前置空格（如果前一个不是标点）
//                     if !sql.is_empty() && !sql.ends_with(' ') && last_was_ident {
//                         sql.push(' ');
//                     }
//                     sql.push_str(&punct_str);
//                     last_was_ident = false;
//                     last_was_keyword = false;
//                 }
//             },
//             proc_macro::TokenTree::Literal(lit) => {
//                 let lit_str = lit.to_string();
                
//                 // 检测是否为数字字面量
//                 let is_number = lit_str.chars().next().map_or(false, |c| c.is_digit(10));
                
//                 // 如果前一个标记是问号(?)，且当前是数字，不添加空格（处理 ?1, ?2 等情况）
//                 if sql.ends_with('?') && is_number {
//                     // 特殊情况1：问号后面直接跟数字 - 不添加空格（处理参数占位符）
//                     sql.push_str(&lit_str);
//                 } else if last_was_keyword && is_number {
//                     // 特殊情况2：关键字后面跟数字 - 确保有空格（如 LIMIT 1）
//                     if !sql.ends_with(' ') {
//                         sql.push(' ');
//                     }
//                     sql.push_str(&lit_str);
//                 } else if !sql.is_empty() && !sql.ends_with(' ') {
//                     // 一般情况：确保字面量前有空格
//                     sql.push(' ');
//                     sql.push_str(&lit_str);
//                 } else {
//                     sql.push_str(&lit_str);
//                 }
                
//                 last_was_ident = false;
//                 // last_was_number = is_number;
//                 last_was_keyword = false;
//             },
//             proc_macro::TokenTree::Group(group) => {
//                 let delimiter = match group.delimiter() {
//                     proc_macro::Delimiter::Parenthesis => {
//                       state.depth += 1;
//                       "("
//                     },
//                     proc_macro::Delimiter::Brace => "{",
//                     proc_macro::Delimiter::Bracket => "[",
//                     proc_macro::Delimiter::None => "",
//                 };
//                 sql.push_str(delimiter);
                
//                 let (inner_sql, _) = tokens_to_sql(group.stream(), state);
//                 sql.push_str(&inner_sql);
                
//                 let closing = match group.delimiter() {
//                     proc_macro::Delimiter::Parenthesis => {
//                       state.depth -= 1;
//                       ")"
//                     },
//                     proc_macro::Delimiter::Brace => "}",
//                     proc_macro::Delimiter::Bracket => "]",
//                     proc_macro::Delimiter::None => "",
//                 };
//                 sql.push_str(closing);
//                 last_was_keyword = false;
//             }
//         }
//     }
    
//     (sql, first_span.unwrap_or_else(Span::call_site))
// }

// 解析SQL tokens和可选的参数结构
pub(crate) fn parse_sql_no_quotes(input: TokenStream) -> (String, Option<TokenStream>, Span) {
    let mut all_tokens: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    
    // If it's a SELECT statement with multiple columns, we need special handling
    let sql_starts_with_select = all_tokens.iter()
        .take(10) // Look at first few tokens
        .any(|t| {
            if let proc_macro::TokenTree::Ident(i) = t {
                i.to_string().to_uppercase() == "SELECT"
            } else {
                false
            }
        });
    
    
    // If it's a SELECT statement, look for a parameter-separating comma more carefully
    let comma_pos = if sql_starts_with_select {
        find_parameter_separator_comma(&all_tokens)
    } else {
        // Original logic for non-SELECT statements
        all_tokens.iter().position(|t| {
            if let proc_macro::TokenTree::Punct(p) = t {
                p.to_string() == ","
            } else {
                false
            }
        })
    };
    
    // 提取参数部分
    let params = if let Some(pos) = comma_pos {
        let param_tokens: Vec<proc_macro::TokenTree> = all_tokens.drain(pos + 1..).collect();
        let param_stream = TokenStream::from_iter(param_tokens);
        Some(param_stream)
    } else {
        None
    };
    
    // 如果找到了逗号，从tokens中移除它
    if comma_pos.is_some() {
        if !all_tokens.is_empty() && all_tokens.len() > comma_pos.unwrap() {
            all_tokens.remove(comma_pos.unwrap());
        }
    }
    
    // 将剩余tokens转换为SQL字符串
    let sql_stream = TokenStream::from_iter(all_tokens);
    // let mut state = ParenthesesState {
    //   depth: 0,
    // };
    // let (sql, span) = tokens_to_sql(sql_stream, &mut state);

    let (spans, sql) = make_sql(sql_stream); 
    
    (sql, params, spans[0].1)
}

static KEYWORDS: [&str; 5] = ["FROM", "*FROM", "JOIN", "UPDATE", "INTO"];

// 将驼峰命名的表名转换为蛇形命名
fn transform_table_names(sql: &str) -> String {
    // 常见的 SQL 关键词后面通常跟表名
    
    let mut transformed = sql.to_string();
    let words: Vec<&str> = sql.split_whitespace().collect();
    
    // 遍历单词，查找关键词后面的可能表名
    for i in 0..words.len().saturating_sub(1) {
        let word = words[i];
        if KEYWORDS.contains(&word.to_uppercase().as_str()) {
            let potential_table = words[i + 1];
            
            // 检查是否是驼峰命名（首字母大写）
            if !potential_table.is_empty() && potential_table.chars().next().unwrap().is_uppercase() {
                // 使用项目已有的转换函数
                let snake_name = convert_to_snake_name(potential_table);
                
                // 替换原始表名，注意保留原始的大小写风格
                transformed = transformed.replace(potential_table, &snake_name);
            }
        }
    }
    
    transformed
}

// 处理SQL语句，验证并格式化每个语句
pub(crate) fn process_sql(sql: &str, span: Span) -> std::result::Result<String, TokenStream> {
    // 首先转换表名
    let sql_with_transformed_tables = transform_table_names(sql);

    // 按分号分割SQL语句
    let statements: Vec<&str> = sql_with_transformed_tables.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    let mut validated_statements = Vec::new();
    
    for stmt in statements {
        // 验证SQL语法
        if let Err(error) = sql_check_impl::check_sql_syntax(stmt, span.into()) {
            return Err(error);
        }

        // if let Err(error) = sql_check_impl::validate_placeholders(stmt, span) {
        //     return Err(error);
        // }
        
        // 格式化SQL
        let formatted = match sql_check_impl::format_sql(stmt) {
            Ok(formatted) => formatted,
            Err(_) => stmt.to_string(),
        };
        validated_statements.push(formatted);
    }

    if validated_statements.is_empty() {
      Ok("".to_string())
  } else {
      // Add semicolons to all statements
      let result = validated_statements
          .iter()
          .map(|stmt| stmt.trim())
          .collect::<Vec<&str>>()
          .join(";\n");
      
      // Add final semicolon if missing
      if result.trim_end().ends_with(';') {
          Ok(result)
      } else {
          Ok(result + ";")
      }
  }
    
    // Ok(validated_statements.join(";\n"))
}

// 主实现函数
pub fn sql_no_quotes(input: TokenStream) -> TokenStream {
    // 解析输入，提取SQL和参数
    let (sql_string, params, span) = parse_sql_no_quotes(input);
    
    // 处理并验证SQL语句
    let validated_sql = match process_sql(&sql_string, span) {
        Ok(result) => result,
        Err(error) => return error,
    };
    
    // 生成输出代码
    let output = if let Some(params) = params {
        // 带参数情况
        let sql_lit = LitStr::new(&validated_sql, proc_macro2::Span::call_site());
        let params = TokenStream2::from(params);
        quote! {
            #sql_lit
            #params
        }
    } else {
        // 无参数情况
        let sql_lit = LitStr::new(&validated_sql, proc_macro2::Span::call_site());
        quote! { #sql_lit }
    };
    
    output.into()
}


// Helper function to find a parameter separator comma in a SELECT statement
fn find_parameter_separator_comma(tokens: &[proc_macro::TokenTree]) -> Option<usize> {
    let mut paren_depth = 0;
    let mut in_from_clause = false;
    
    for (i, token) in tokens.iter().enumerate() {
        match token {
            proc_macro::TokenTree::Group(g) => {
                if g.delimiter() == proc_macro::Delimiter::Parenthesis {
                    paren_depth += 1;
                }
            },
            proc_macro::TokenTree::Ident(id) => {
                if id.to_string().to_uppercase() == "FROM" {
                    in_from_clause = true;
                }
            },
            proc_macro::TokenTree::Punct(p) => {
                let punct_str = p.to_string();
                
                // Track parenthesis depth
                if punct_str == "(" {
                    paren_depth += 1;
                } else if punct_str == ")" {
                    paren_depth = paren_depth - 1;
                }
                
                // If we're at depth 0 and after the FROM clause, a comma is likely a parameter separator
                if punct_str == "," && paren_depth == 0 && in_from_clause {
                    return Some(i);
                }
            },
            _ => {}
        }
    }
    
    None
}

fn make_sql(tokens: TokenStream) -> (Vec<(usize, Span)>, String) {
    let mut sql_tokens = vec![];
    flatten_stream(tokens, &mut sql_tokens);
    // Lookup of spans by offset at the end of the token
    let mut spans: Vec<(usize, Span)> = Vec::new();
    let mut sql = String::new();
    for (token_text, span) in sql_tokens {
        sql.push_str(&token_text);
        spans.push((sql.len(), span));
    }
    (spans, sql)
}

/// This method exists to normalize the representation of groups
/// to always include spaces between tokens. This is why we don't use the usual .to_string().
/// This allows our token search in token_at_offset to resolve
/// ambiguity of '(tokens)' vs. '( token )', due to sqlite requiring byte offsets
fn flatten_stream(tokens: TokenStream, result: &mut Vec<(String, Span)>) {
    for token_tree in tokens.into_iter() {
        match token_tree {
            TokenTree::Group(group) => {
                // push open delimiter
                result.push((open_delimiter(group.delimiter()), group.span()));
                // recurse
                flatten_stream(group.stream(), result);
                // push close delimiter
                result.push((close_delimiter(group.delimiter()), group.span()));
            }
            TokenTree::Ident(ident) => {
                result.push((format!("{} ", ident), ident.span()));
            }
            leaf_tree => result.push((leaf_tree.to_string(), leaf_tree.span())),
        }
    }
}

fn open_delimiter(delimiter: Delimiter) -> String {
    match delimiter {
        Delimiter::Parenthesis => "( ".to_string(),
        Delimiter::Brace => "[ ".to_string(),
        Delimiter::Bracket => "{ ".to_string(),
        Delimiter::None => "".to_string(),
    }
}

fn close_delimiter(delimiter: Delimiter) -> String {
    match delimiter {
        Delimiter::Parenthesis => " ) ".to_string(),
        Delimiter::Brace => " ] ".to_string(),
        Delimiter::Bracket => " } ".to_string(),
        Delimiter::None => "".to_string(),
    }
}