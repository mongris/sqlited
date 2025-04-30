use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::quote;
use syn::LitStr;

use crate::{sql_check_impl, utils::convert_to_snake_name};


struct ParenthesesState {
    depth: usize,
}

// 常见的 SQL 关键词后面通常跟表名
static KEYWORDS: [&str; 4] = ["FROM", "JOIN", "UPDATE", "INTO"];

// SQL 关键字列表 - 这些不应该被转换
// static SQL_KEYWORDS: [&str; 91] = [
//     "SELECT", "FROM", "WHERE", "AND", "OR", "INSERT", "UPDATE", "DELETE",
//     "JOIN", "LEFT", "RIGHT", "INNER", "OUTER", "FULL", "CROSS", "ON",
//     "GROUP", "BY", "HAVING", "ORDER", "LIMIT", "OFFSET", "UNION", "ALL",
//     "AS", "DISTINCT", "CASE", "WHEN", "THEN", "ELSE", "END", "IN", "EXISTS",
//     "NOT", "NULL", "IS", "LIKE", "BETWEEN", "ASC", "DESC", "VALUES", "SET",
//     "INTO", "DEFAULT", "PRIMARY", "KEY", "FOREIGN", "REFERENCES", "UNIQUE",
//     "CHECK", "CONSTRAINT", "CASCADE", "RESTRICT", "NO", "ACTION", "INDEX",
//     "CREATE", "ALTER", "DROP", "TABLE", "COLUMN", "ADD", "MODIFY", "RENAME",
//     "TO", "DATABASE", "SCHEMA", "VIEW", "FUNCTION", "PROCEDURE", "TRIGGER",
//     "RETURNING", "CONFLICT", "DO", "NOTHING", "INSTEAD", "OF", "FOR", "EACH",
//     "ROW", "STATEMENT", "EXECUTE", "PROCEDURE", "FUNCTION", "LANGUAGE",
//     "BEGIN", "COMMIT", "ROLLBACK", "TRANSACTION", "SAVEPOINT", "RELEASE"
// ];

// 将TokenStream转换为SQL字符串
fn tokens_to_sql(input: TokenStream, state: &mut ParenthesesState) -> (String, Span) {
    let mut sql = String::new();
    let mut first_span = None;
    // let mut last_was_ident = false;
    // let mut last_was_keyword = false;

    // 将标记流转换为SQL字符串
    for token in input {
        match token {
            proc_macro::TokenTree::Ident(ident) => {
                if !sql.is_empty() && !sql.ends_with('(') && !sql.ends_with('.') && !sql.ends_with(' ') {
                    sql.push(' ');
                }

                let ident_str = ident.to_string();
                sql.push_str(&ident_str);

                // let upper_ident = ident_str.to_uppercase();
                // last_was_keyword = KEYWORDS.contains(&upper_ident.as_str());

                if first_span.is_none() {
                    first_span = Some(Span::call_site());
                }

                // last_was_ident = true;
            },
            proc_macro::TokenTree::Punct(punct) => {
                let punct_str = punct.to_string();

                // REMOVED THE BREAK LOGIC FOR COMMA
                // The separation is handled in parse_sql_no_quotes

                // Handle spacing around punctuation
                if punct_str == "," || punct_str == ";" {
                    // Add space after comma/semicolon
                    sql.push_str(&punct_str);
                    sql.push(' ');
                } else if punct_str == "." {
                     // No spaces around dot
                     // Remove trailing space if present before adding dot
                     if sql.ends_with(' ') {
                         sql.pop();
                     }
                     sql.push_str(&punct_str);
                } else if punct_str == "(" {
                    // No space before opening parenthesis
                    sql.push_str(&punct_str);
                } else if punct_str == ")" {
                     // No space before closing parenthesis
                     // Remove trailing space if present before adding parenthesis
                     if sql.ends_with(' ') {
                         sql.pop();
                     }
                     sql.push_str(&punct_str);
                } else if punct_str == "?" {
                     // Space before ? unless previous was ( or ,
                     if !sql.is_empty() && !sql.ends_with('(') && !sql.ends_with(',') && !sql.ends_with(' ') {
                         sql.push(' ');
                     }
                     sql.push_str(&punct_str);
                } else if punct_str == "=" {
                     // Spaces around equals sign
                     if !sql.ends_with(' ') { sql.push(' '); }
                     sql.push_str(&punct_str);
                     sql.push(' ');
                }
                 else {
                    // Default: add space before if needed
                    if !sql.is_empty() && !sql.ends_with('(') && !sql.ends_with(' ') {
                        sql.push(' ');
                    }
                    sql.push_str(&punct_str);
                }

                // last_was_ident = false;
                // last_was_keyword = false;
            },
            proc_macro::TokenTree::Literal(lit) => {
                let lit_str = lit.to_string();

                // 检测是否为数字字面量
                let is_number = lit_str.chars().next().map_or(false, |c| c.is_digit(10));

                // Handle spacing before literal
                if sql.ends_with('?') && is_number {
                    // No space after ? for numbered placeholders like ?1
                    sql.push_str(&lit_str);
                } else if !sql.is_empty() && !sql.ends_with('(') && !sql.ends_with(' ') && !sql.ends_with('.') {
                     sql.push(' ');
                     sql.push_str(&lit_str);
                }
                 else {
                    sql.push_str(&lit_str);
                }

                // last_was_ident = false;
                // last_was_keyword = false;
            },
            proc_macro::TokenTree::Group(group) => {
                 // Handle spacing before group delimiter
                 if !sql.is_empty() && !sql.ends_with(' ') && !sql.ends_with('(') {
                     sql.push(' ');
                 }

                let (start_delimiter, end_delimiter) = match group.delimiter() {
                    proc_macro::Delimiter::Parenthesis => ("(", ")"),
                    proc_macro::Delimiter::Brace => ("{", "}"),
                    proc_macro::Delimiter::Bracket => ("[", "]"),
                    proc_macro::Delimiter::None => ("", ""),
                };

                if start_delimiter == "(" { state.depth += 1; }
                sql.push_str(start_delimiter);

                // Recursively process inner tokens
                let (inner_sql, _) = tokens_to_sql(group.stream(), state);
                sql.push_str(&inner_sql.trim()); // Trim inner SQL to avoid extra spaces

                if end_delimiter == ")" { state.depth = state.depth.saturating_sub(1); }
                 // Remove trailing space before closing delimiter if present
                 if sql.ends_with(' ') {
                     sql.pop();
                 }
                sql.push_str(end_delimiter);

                // last_was_keyword = false;
                // last_was_ident = false; // Reset ident flag after group
            }
        }
    }
    // Trim final result to remove potential trailing space
    (sql.trim().to_string(), first_span.unwrap_or_else(Span::call_site))
}


// 解析SQL tokens和可选的参数结构
pub(crate) fn parse_sql_no_quotes(input: TokenStream) -> (String, Option<TokenStream>, Span) {
    let mut all_tokens: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    
    // Determine if we need special handling
    let needs_special_handling = {
        let mut has_select = false;
        let mut has_update = false;
        let mut has_conflict = false;
        
        for t in all_tokens.iter() {
            if let proc_macro::TokenTree::Ident(i) = t {
                let upper = i.to_string().to_uppercase();
                if upper == "SELECT" {
                    has_select = true;
                } else if upper == "UPDATE" {
                    has_update = true;
                } else if upper == "CONFLICT" {
                    has_conflict = true;
                }
            }
        }
        
        has_select || has_update || has_conflict
    };
    
    // Find parameter separator comma position
    let comma_pos = if needs_special_handling {
        find_complex_parameter_separator(&all_tokens)
    } else {
        // Simple case - find first comma at top level
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
    let mut state = ParenthesesState {
        depth: 0,
    };
    let (sql, span) = tokens_to_sql(sql_stream, &mut state);

    // println!("Parsed SQL: {}", sql);
    // println!("Parsed Params: {:?}", params);
    
    (sql, params, span)
}


// helper function to find parameter separator in complex SQL statements
fn find_complex_parameter_separator(tokens: &[proc_macro::TokenTree]) -> Option<usize> {
    let mut paren_depth = 0;
    let mut in_select_clause = false;
    let mut in_update_set_clause = false;
    let mut in_conflict_clause = false;
    
    // First pass: identify SQL structure
    for (i, token) in tokens.iter().enumerate() {
        match token {
            proc_macro::TokenTree::Group(g) => {
                if g.delimiter() == proc_macro::Delimiter::Parenthesis {
                    paren_depth += 1;
                }
            },
            proc_macro::TokenTree::Ident(id) => {
                let upper = id.to_string().to_uppercase();
                if upper == "SELECT" {
                    in_select_clause = true;
                } else if upper == "SET" && tokens.iter().take(i).any(|t| {
                    if let proc_macro::TokenTree::Ident(id) = t {
                        id.to_string().to_uppercase() == "UPDATE"
                    } else {
                        false
                    }
                }) {
                    in_update_set_clause = true;
                } else if upper == "CONFLICT" && i > 0 {
                    // Check if preceded by "ON"
                    if let Some(prev) = tokens.get(i-1) {
                        if let proc_macro::TokenTree::Ident(prev_id) = prev {
                            if prev_id.to_string().to_uppercase() == "ON" {
                                in_conflict_clause = true;
                                // Important: Once we detect an ON CONFLICT clause, we return None
                                // to indicate no parameter separator should be found
                                return None;
                            }
                        }
                    }
                }
            },
            proc_macro::TokenTree::Punct(p) => {
                let punct_char = p.to_string();
                if punct_char == "(" {
                    paren_depth += 1;
                } else if punct_char == ")" {
                    paren_depth = paren_depth - 1;
                }
            },
            _ => {}
        }
    }

    // No parameter separator needed for these SQL types
    if in_conflict_clause || in_update_set_clause {
        return None;
    }
    
    // Second pass: find appropriate comma
    paren_depth = 0;
    let mut after_from_clause = false;
    
    for (i, token) in tokens.iter().enumerate() {
        match token {
            proc_macro::TokenTree::Group(g) => {
                if g.delimiter() == proc_macro::Delimiter::Parenthesis {
                    // Skip parameter counting in parentheses
                    continue;
                }
            },
            proc_macro::TokenTree::Ident(id) => {
                if id.to_string().to_uppercase() == "FROM" {
                    after_from_clause = true;
                }
            },
            proc_macro::TokenTree::Punct(p) => {
                let punct_char = p.to_string();
                
                if punct_char == "(" {
                    paren_depth += 1;
                } else if punct_char == ")" {
                    paren_depth = paren_depth - 1;
                } else if punct_char == "," && paren_depth == 0 {
                    // In SELECT statements, only consider commas after FROM clause
                    if !in_select_clause || (in_select_clause && after_from_clause) {
                        return Some(i);
                    }
                }
            },
            _ => {}
        }
    }
    
    None
}

// 将驼峰命名的表名转换为蛇形命名
fn transform_table_names(sql: &str) -> String {
    let mut transformed = sql.to_string();
    let words: Vec<&str> = sql.split_whitespace().collect();
    
    // 遍历单词，查找关键词后面的可能表名
    for i in 0..words.len().saturating_sub(1) {
        let word = words[i];
        if KEYWORDS.contains(&word.to_uppercase().as_str()) {
            let pre_keyword = words[i -1];
            let potential_table = words[i + 1];
            
            // 检查是否是驼峰命名（首字母大写）
            if (pre_keyword.is_empty() || pre_keyword != "DO") && !potential_table.is_empty() && potential_table.chars().next().unwrap().is_uppercase() {
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

    // println!("Transformed SQL: {}", sql_with_transformed_tables);

    // 按分号分割SQL语句
    let statements: Vec<&str> = sql_with_transformed_tables.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    let mut validated_statements = Vec::new();
    
    for stmt in statements {
        // 验证SQL语法
        if let Err(error) = sql_check_impl::check_sql_syntax(stmt, span) {
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
// fn find_parameter_separator_comma(tokens: &[proc_macro::TokenTree]) -> Option<usize> {
//     let mut paren_depth = 0;
//     let mut in_from_clause = false;
    
//     for (i, token) in tokens.iter().enumerate() {
//         match token {
//             proc_macro::TokenTree::Group(g) => {
//                 if g.delimiter() == proc_macro::Delimiter::Parenthesis {
//                     paren_depth += 1;
//                 }
//             },
//             proc_macro::TokenTree::Ident(id) => {
//                 if id.to_string().to_uppercase() == "FROM" {
//                     in_from_clause = true;
//                 }
//             },
//             proc_macro::TokenTree::Punct(p) => {
//                 let punct_str = p.to_string();
                
//                 // Track parenthesis depth
//                 if punct_str == "(" {
//                     paren_depth += 1;
//                 } else if punct_str == ")" {
//                     paren_depth = paren_depth - 1;
//                 }
                
//                 // If we're at depth 0 and after the FROM clause, a comma is likely a parameter separator
//                 if punct_str == "," && paren_depth == 0 && in_from_clause {
//                     return Some(i);
//                 }
//             },
//             _ => {}
//         }
//     }
    
//     None
// }