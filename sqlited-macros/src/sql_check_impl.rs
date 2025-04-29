use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{
    // parse_macro_input, 
    // LitStr, 
    Error
};
use std::sync::OnceLock;
use r2d2_sqlite::SqliteConnectionManager;

// SQLite validator singleton
static VALIDATOR: OnceLock<r2d2::Pool<SqliteConnectionManager>> = OnceLock::new();

fn get_validator() -> &'static r2d2::Pool<SqliteConnectionManager> {
    VALIDATOR.get_or_init(|| {
        let manager = SqliteConnectionManager::memory();
        r2d2::Pool::new(manager).unwrap()
    })
}

// Main function to validate SQL syntax
pub fn check_sql_syntax(sql: &str, span: Span) -> Result<(), TokenStream> {
    // Get database connection from pool
    let conn = match get_validator().get() {
        Ok(conn) => conn,
        Err(err) => {
            let error = format!("Failed to get SQLite connection: {}", err);
            return Err(Error::new(span, error).to_compile_error().into());
        }
    };

    // Validate SQL syntax
    if let Err(err) = conn.prepare(sql) {
        let error_msg = err.to_string();
        
        // Ignore errors related to missing tables/schema
        if error_msg.contains("no such table:") || 
           error_msg.contains("no such column:") || 
           error_msg.contains("no such collation") ||
           error_msg.contains("unable to open database file") {
            return Ok(());
        }
        
        let formatted_error = format!("SQL 语法错误: {}\n查询语句: {}", error_msg, sql);
        return Err(Error::new(span, formatted_error).to_compile_error().into());
    }
    
    Ok(())
}

// pub fn validate_placeholders(sql: &str, span: Span) -> Result<(), TokenStream> {
//     let mut chars = sql.chars().peekable();
//     let mut in_string = false;
//     let mut string_delimiter = ' ';
    
//     while let Some(c) = chars.next() {
//         // Handle string literals (to avoid checking ? inside strings)
//         if (c == '\'' || c == '"') && (chars.peek().copied() != Some('\'') && chars.peek().copied() != Some('"')) {
//             if !in_string {
//                 in_string = true;
//                 string_delimiter = c;
//             } else if c == string_delimiter {
//                 in_string = false;
//             }
//             continue;
//         }
        
//         // Skip ? inside string literals
//         if in_string {
//             continue;
//         }
        
//         // Check for ? that isn't wrapped in parentheses
//         if c == '?' {
//             // Look backward for opening parenthesis
//             let before_text = &sql[..sql.char_indices().find(|(i, _)| *i == sql.find(c).unwrap()).unwrap().0];
//             let has_opening = before_text.trim_end().ends_with('(');
            
//             // Look forward for closing parenthesis
//             let after_idx = sql.find(c).unwrap() + 1;
//             let after_text = &sql[after_idx..];
//             let has_closing = after_text.trim_start().starts_with(')');
            
//             if !has_opening || !has_closing {
//                 let error_message = "SQL placeholder '?' must be wrapped in parentheses like '(?)'";
//                 return Err(Error::new(span, error_message).to_compile_error().into());
//             }
//         }
//     }
    
//     Ok(())
// }

// Format SQL for better readability
pub fn format_sql(sql: &str) -> Result<String, ()> {
    println!("Formatting SQL: {}", sql);
    Ok(sqlformat::format(
        sql,
        &sqlformat::QueryParams::None,
        Default::default()
    ))
}

// // SQL check macro implementation
// pub fn sql_check(input: TokenStream) -> TokenStream {
//     let sql_str = parse_macro_input!(input as LitStr);
//     let sql = sql_str.value();
    
//     if let Err(err) = check_sql_syntax(&sql, sql_str.span()) {
//         return err;
//     }
//     // if let Err(err) = validate_placeholders(&sql, sql_str.span()) {
//     //     return err;
//     // }
    
//     // Return the original SQL
//     let formatted = match format_sql(&sql) {
//         Ok(fmt) => fmt,
//         Err(_) => sql,
//     };
    
//     let lit = LitStr::new(&formatted, proc_macro2::Span::call_site());
//     quote::quote!(#lit).into()
// }