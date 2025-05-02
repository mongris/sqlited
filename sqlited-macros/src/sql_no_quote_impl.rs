use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree as TokenTree2, Delimiter};
use quote::quote;
use syn::{
    parse::{Parser, ParseStream, Result as SynResult}, // 使用 syn 的 ParseStream
    Ident, LitStr, Token, Error, // 使用 syn 的 Ident, LitStr, Token, Error
};

use crate::{sql_check_impl, utils::convert_to_snake_name};

// 使用 syn::custom_keyword 来定义 SQL 关键字，以便更精确地解析
mod kw {
    syn::custom_keyword!(FROM);
    syn::custom_keyword!(JOIN); // 包括 LEFT JOIN, INNER JOIN 等的变化形式需要额外处理
    syn::custom_keyword!(UPDATE);
    syn::custom_keyword!(INTO);
    syn::custom_keyword!(AS);
    syn::custom_keyword!(SET);
    syn::custom_keyword!(DO);
    syn::custom_keyword!(LATERAL);
    syn::custom_keyword!(NOT);
    syn::custom_keyword!(MATERIALIZED);
    syn::custom_keyword!(WHERE);
    syn::custom_keyword!(NOTHING);
}

struct SqlBuilder {
    sql: String,
    needs_leading_space: bool,
    first_span: Option<Span>,
}

impl SqlBuilder {
    fn new() -> Self {
        SqlBuilder {
            sql: String::new(),
            needs_leading_space: false,
            first_span: None,
        }
    }

    fn set_span_if_none(&mut self, span: Span) {
        if self.first_span.is_none() {
            self.first_span = Some(span);
        }
    }

    fn ensure_space(&mut self) {
        if self.needs_leading_space && !self.sql.is_empty() && !self.sql.ends_with(' ') {
            self.sql.push(' ');
        }
    }

    fn trim_trailing_space(&mut self) {
        if self.sql.ends_with(' ') {
            self.sql.pop();
        }
    }

    // 核心方法：添加 token 字符串，并管理前后空格
    // space_before: 此 token 前是否需要空格
    // space_after: 此 token 后是否允许/需要空格
    fn push(&mut self, token_str: &str, space_before: bool, space_after: bool) {
        if space_before {
            self.ensure_space();
        } else {
            self.trim_trailing_space(); // 如果不需要前导空格，移除可能存在的尾随空格
        }
        self.sql.push_str(token_str);
        self.needs_leading_space = space_after;
    }

    // 添加原始 TokenTree，自动判断空格
    fn push_token(&mut self, token: &TokenTree2) {
        self.set_span_if_none(token.span());
        let token_str = token.to_string();

        match token {
            TokenTree2::Ident(_) => self.push(&token_str, true, true),
            TokenTree2::Literal(_) => self.push(&token_str, true, true),
            TokenTree2::Punct(p) => {
                let ch = p.as_char();
                match ch {
                    '.' => self.push(".", false, false),
                    ',' | ';' => self.push(&token_str, false, true), // 逗号/分号前不加空格，后加
                    '(' | '[' => self.push(&token_str, true, false), // 开括号前允许空格，后不加
                    ')' | ']' => self.push(&token_str, false, true), // 闭括号前不加空格，后允许
                    '?' => self.push("?", true, true), // 问号前后通常有空格
                    // 简单处理常见运算符
                    '=' | '+' | '-' | '*' | '/' | '<' | '>' => {
                         // 如果是多字符运算符（如 ::, >=），则不加空格
                         if token_str.len() == 1 {
                             self.push(&token_str, true, true);
                         } else {
                             // 例如 ::
                             self.push(&token_str, false, false);
                         }
                    }
                    _ => self.push(&token_str, true, true), // 其他标点符号默认前后加空格
                }
            }
            TokenTree2::Group(_) => {
                // Group 的处理在 parse_sql 函数中递归进行，这里理论上不应该直接 push_token Group
                // 但作为备用，假设它像标识符一样处理
                self.push(&token_str, true, true);
            }
        }
    }

    fn finalize(mut self) -> (String, Span) {
        if self.sql.ends_with(' ') {
            self.sql.pop();
        }
        (
            self.sql,
            self.first_span.unwrap_or_else(Span::call_site),
        )
    }
}

// 使用 syn 解析 TokenStream 并构建 SQL 字符串
fn parse_sql(input: ParseStream) -> SynResult<(String, Span)> {
    let mut builder = SqlBuilder::new();

    while !input.is_empty() {
        // 检查是否是定义的关键字
        if input.peek(kw::FROM) {
            let keyword = input.parse::<kw::FROM>()?;
            builder.set_span_if_none(keyword.span);
            builder.push("FROM", true, true); // FROM 前后需要空格
            parse_optional_table_name(input, &mut builder)?;
        } else if input.peek(kw::JOIN) {
            // 处理 JOIN (包括 LEFT JOIN, INNER JOIN 等)
            // 注意：syn::Ident 可以解析 "LEFT", "INNER" 等
            let join_keyword = input.parse::<Ident>()?; // 解析 JOIN 或 LEFT/INNER/RIGHT/FULL/CROSS
            builder.set_span_if_none(join_keyword.span());
            builder.push(&join_keyword.to_string().to_uppercase(), true, true);

            // 如果是 LEFT/INNER 等，后面还会有 JOIN 关键字
            if input.peek(kw::JOIN) {
                 input.parse::<kw::JOIN>()?; // 解析并消耗 JOIN 关键字
                 builder.push("JOIN", true, true);
            } else if join_keyword.to_string().to_uppercase() != "JOIN" {
                 // 如果解析的不是 JOIN 本身，且后面没有 JOIN，则可能语法错误或需要更复杂的解析
                 // 这里暂时假设 JOIN 总是跟着表名
            }
            parse_optional_table_name(input, &mut builder)?;
        } else if input.peek(kw::DO) {
            let do_keyword = input.parse::<kw::DO>()?;
            builder.set_span_if_none(do_keyword.span);
            builder.push("DO", true, true);

            if input.peek(kw::UPDATE) {
                // DO UPDATE 处理逻辑
                input.parse::<kw::UPDATE>()?; // 解析并消耗 UPDATE 关键字
                builder.push("UPDATE", true, true);
                // 检查 SET，避免误认表名
                if !input.peek(kw::SET) {
                    // 理论上 DO UPDATE 后面不应该直接跟表名，而是 SET
                    // 但为了安全，可以保留这个检查，或者根据实际语法调整
                    // parse_optional_table_name(input, &mut builder)?;
                    // 通常 DO UPDATE 后面就是 SET，所以这里可能不需要 parse_optional_table_name
                }
                // 让循环在下一次处理 SET
            } else if input.peek(kw::NOTHING) {
                // 处理 DO NOTHING
                input.parse::<kw::NOTHING>()?; // 解析并消耗 NOTHING 关键字
                builder.push("NOTHING", true, true);
                // DO NOTHING 后面通常没有其他东西了（在 ON CONFLICT 子句中）
            } else {
                // DO 后面跟着未知的内容，可能需要报错或按原样添加
                // 暂时按原样添加下一个 token
                if !input.is_empty() {
                        let token: TokenTree2 = input.parse()?;
                        builder.push_token(&token);
                }
            }
        } else if input.peek(kw::UPDATE) {
            let keyword = input.parse::<kw::UPDATE>()?;
            builder.set_span_if_none(keyword.span);
            builder.push("UPDATE", true, true);
            parse_optional_table_name(input, &mut builder)?;
        } else if input.peek(kw::INTO) {
            let keyword = input.parse::<kw::INTO>()?;
            builder.set_span_if_none(keyword.span);
            builder.push("INTO", true, true);
            parse_optional_table_name(input, &mut builder)?;
        }
        // 可以添加对其他关键字的处理，如 SET, WHERE 等
        else {
            // 检查是否是 '?<number>' 模式
            if input.peek(Token![?]) && input.peek2(syn::LitInt) {
                let q_mark = input.parse::<Token![?]>()?;
                let number: syn::LitInt = input.parse()?;
                let combined = format!("?{}", number.base10_digits()); // 组合成 "?1", "?2" 等

                builder.set_span_if_none(q_mark.span); // 使用问号的 span
                builder.push(&combined, true, true); // 添加组合后的占位符，前后允许空格
            } else { // 如果不是 '?<number>' 模式，则按原来的方式处理
                // 如果不是我们特别处理的关键字，则解析为通用的 TokenTree
                let token: TokenTree2 = input.parse()?;
                match token {
                    TokenTree2::Group(group) => {
                        // 对括号内的内容递归解析
                        let (start_delimiter, end_delimiter) = match group.delimiter() {
                            Delimiter::Parenthesis => ("(", ")"),
                            Delimiter::Bracket => ("[", "]"),
                            Delimiter::Brace => ("{", "}"), // SQL 中 {} 不常用，可能需要特殊处理
                            Delimiter::None => ("", ""),
                        };
                        builder.set_span_if_none(group.span());
                        builder.push(start_delimiter, true, false); // 开括号前允许空格，后不允许

                        // 解析括号内的流
                        let (inner_sql, _) = Parser::parse2(parse_sql, group.stream())?;
                        // 直接将内部解析结果追加，内部已处理空格
                        builder.sql.push_str(&inner_sql);
                        // 确保内部解析后，闭括号前没有多余空格
                        builder.needs_leading_space = false;

                        builder.push(end_delimiter, false, true); // 闭括号前不允许空格，后允许
                    }
                    _ => {
                        // 对于非关键字标识符、字面量、标点符号，使用 builder 的 push_token 处理
                        builder.push_token(&token);
                    }
                }
            }
        }
    }

    Ok(builder.finalize())
}

// 辅助函数：解析关键字后面的可选表名，并进行转换
fn parse_optional_table_name(input: ParseStream, builder: &mut SqlBuilder) -> SynResult<()> {
    // 查看下一个 token 是否是标识符 (可能是表名)
    // 同时处理 schema.Table 的情况
    if input.peek(Ident) && !input.peek2(Token![.]) && !input.peek2(Token![::]) {
        let table_ident: Ident = input.parse()?;
        let table_name = table_ident.to_string();

        // 检查是否可能是表名（例如，首字母大写）
        // 这个检查可以根据项目约定调整
        if table_name.chars().next().map_or(false, |c| c.is_uppercase()) {
            let snake_name = convert_to_snake_name(&table_name);
            builder.push(&snake_name, true, true); // 转换后的表名前后需要空格
        } else {
            // 如果不是驼峰，则按原样添加
            builder.push(&table_name, true, true);
        }
    } else if input.peek(Ident) && (input.peek2(Token![.]) || input.peek2(Token![::])) && input.peek3(Ident) {
         // 处理 schema.Table 或 schema::Table
         let schema_ident: Ident = input.parse()?;
         let punct: TokenTree2 = input.parse()?; // . or ::
         let table_ident: Ident = input.parse()?;

         builder.push(&schema_ident.to_string(), true, false); // schema 后不加空格
         builder.push_token(&punct); // . 或 :: 前后不加空格
         // 假设表名需要转换
         let table_name = table_ident.to_string();
         if table_name.chars().next().map_or(false, |c| c.is_uppercase()) {
             let snake_name = convert_to_snake_name(&table_name);
             builder.push(&snake_name, false, true); // 表名前不加空格，后加
         } else {
             builder.push(&table_name, false, true);
         }
    }
    // 如果后面不是标识符，则不处理，让主循环继续解析

    Ok(())
}

// 参数分割，返回 TokenStream
pub(crate) fn parse_sql_no_quotes(input: TokenStream) -> (Result<(String, Span), Error>, Option<TokenStream>, Span) {
    let mut all_tokens: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let first_span = all_tokens.first().map(|t| t.span().into()).unwrap_or_else(Span::call_site); // 使用 proc_macro2::Span

    let needs_special_handling = {
        let mut has_select = false;
        let mut has_update = false;
        let mut has_conflict = false;
        for t in all_tokens.iter() {
            if let proc_macro::TokenTree::Ident(i) = t {
                let upper = i.to_string().to_uppercase();
                if upper == "SELECT" { has_select = true; }
                else if upper == "UPDATE" { has_update = true; }
                else if upper == "CONFLICT" { has_conflict = true; }
            }
        }
        has_select || has_update || has_conflict
    };

    let comma_pos = if needs_special_handling {
        find_complex_parameter_separator(&all_tokens)
    } else {
        all_tokens.iter().position(|t| {
            if let proc_macro::TokenTree::Punct(p) = t { p.as_char() == ',' } else { false }
        })
    };

    let params = if let Some(pos) = comma_pos {
        let param_tokens: Vec<proc_macro::TokenTree> = all_tokens.drain(pos + 1..).collect();
        if !all_tokens.is_empty() && all_tokens.len() > pos {
            all_tokens.remove(pos); // 移除逗号
        }
        Some(TokenStream::from_iter(param_tokens))
    } else {
        None
    };

    let sql_stream = TokenStream::from_iter(all_tokens);
    let sql_string_result = Parser::parse(parse_sql, sql_stream);

    (sql_string_result, params, first_span)
}

#[allow(unused_assignments)]
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
                    // This simple depth tracking might be insufficient for complex cases
                    // but let's keep it for now. A full parser would be better.
                }
            }
            proc_macro::TokenTree::Ident(id) => {
                let upper = id.to_string().to_uppercase();
                if upper == "SELECT" { in_select_clause = true; }
                else if upper == "SET" && tokens.iter().take(i).any(|t| matches!(t, proc_macro::TokenTree::Ident(id) if id.to_string().to_uppercase() == "UPDATE")) {
                    in_update_set_clause = true;
                } else if upper == "CONFLICT" && i > 0 {
                    if let Some(proc_macro::TokenTree::Ident(prev_id)) = tokens.get(i.saturating_sub(1)) {
                        if prev_id.to_string().to_uppercase() == "ON" {
                            in_conflict_clause = true;
                            return None; // No params expected after ON CONFLICT
                        }
                    }
                }
            }
            proc_macro::TokenTree::Punct(p) => {
                 match p.as_char() {
                    '(' => paren_depth += 1,
                    ')' => paren_depth = paren_depth - 1,
                    _ => {}
                 }
            }
            _ => {}
        }
    }

    // No parameter separator needed for these SQL types based on simple analysis
    if in_conflict_clause || in_update_set_clause {
        return None;
    }

    // Second pass: find appropriate comma
    paren_depth = 0;
    let mut after_from_clause = false; // Simplified: only relevant for SELECT

    for (i, token) in tokens.iter().enumerate() {
        match token {
            proc_macro::TokenTree::Group(g) => {
                 if g.delimiter() == proc_macro::Delimiter::Parenthesis {
                    // We need to properly parse the content or track depth accurately.
                    // For now, let's assume top-level commas inside groups are not separators.
                    // A simple depth counter is used below.
                 }
            }
            proc_macro::TokenTree::Ident(id) => {
                if id.to_string().to_uppercase() == "FROM" {
                    after_from_clause = true;
                }
            }
            proc_macro::TokenTree::Punct(p) => {
                match p.as_char() {
                    '(' => paren_depth += 1,
                    ')' => paren_depth = paren_depth - 1,
                    ',' if paren_depth == 0 => {
                        // In SELECT statements, only consider commas after FROM clause (heuristic)
                        if !in_select_clause || after_from_clause {
                            return Some(i);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    None
}

// process_sql
pub(crate) fn process_sql(sql: &str, span: Span) -> std::result::Result<String, TokenStream> {
    // 按分号分割SQL语句
    let statements: Vec<&str> = sql
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut validated_statements = Vec::new();
    let error_span = span; // 使用传入的 span

    for stmt in statements {
        // 验证SQL语法
        if let Err(error) = sql_check_impl::check_sql_syntax(stmt, error_span) {
            return Err(error); // error 已经是 TokenStream
        }

        // 格式化SQL
        let formatted = match sql_check_impl::format_sql(stmt) {
            Ok(formatted) => formatted,
            Err(_) => stmt.to_string(), // 格式化失败则使用原始语句
        };
        validated_statements.push(formatted);
    }

    if validated_statements.is_empty() {
        Ok("".to_string())
    } else {
        let mut result = validated_statements.join(";\n");
        // 确保末尾有分号
        if !result.ends_with(';') {
            result.push(';');
        }
        Ok(result)
    }
}

// 主宏实现，使用新的解析流程
pub fn sql_no_quotes(input: TokenStream) -> TokenStream {
    // 1. 分割 SQL 和参数
    let (sql_string_result, params_token_stream_opt, span) = parse_sql_no_quotes(input);

    let sql_string = match sql_string_result {
        Ok((s, _span)) => s, // 解析成功，获取 SQL 字符串
        Err(e) => return e.to_compile_error().into(), // 解析失败，返回编译错误
    };

    // 2. 处理 SQL 字符串（验证、格式化
    let validated_sql = match process_sql(&sql_string, span) { // 使用 process_sql
        Ok(result) => result,
        Err(error_token_stream) => return error_token_stream, // 返回验证/格式化错误
    };

    // 3. 生成最终代码
    let output = if let Some(params) = params_token_stream_opt {
        let sql_lit = LitStr::new(&validated_sql, span); // 使用原始 span
        let params = TokenStream2::from(params);
        quote! {
            #sql_lit, // 加回逗号
            #params
        }
    } else {
        let sql_lit = LitStr::new(&validated_sql, span);
        quote! { #sql_lit }
    };

    output.into()
}