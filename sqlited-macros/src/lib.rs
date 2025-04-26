use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;

mod sql_check_impl;
mod sql_impl;
mod sql_no_quote_impl;
mod sql_params_impl;
mod table_impl;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    sql_impl::sql(input)
}

#[proc_macro]
pub fn sql_str(input: TokenStream) -> TokenStream {
    sql_no_quote_impl::sql_no_quotes(input)
}
#[proc_macro]
pub fn sql_params(input: TokenStream) -> TokenStream {
    sql_params_impl::sql_params(input)
}

/// 定义数据库表
///
/// 这个宏允许定义数据库表，可以包含多种字段约束：
///
/// - `#[autoincrement]`：字段将是自增主键
/// - `#[primary_key]`：字段将是主键（非自增）
/// - `#[unique]`：字段值必须唯一
/// - `#[default("value")]`：设置默认值
/// - `#[check("expression")]`：添加 CHECK 约束
/// - `#[not_null]`：显式设置非空约束
/// - `#[foreign_key("ref_table", "ref_column")]`：添加外键约束
/// - `#[foreign_key("ref_table", "ref_column", "ON DELETE", "ON UPDATE")]`：带级联动作的外键约束
///
/// 还支持表级约束和索引：
///
/// - `#[constraint("constraint expression")]`：添加表级约束
/// - `#[index("index_name", "column1, column2")]`：创建索引
/// - `#[unique_index("index_name", "column1, column2")]`：创建唯一索引
///
/// # 示例
///
/// ```rust
/// #[table]
/// #[index("idx_user_name", "name")]
/// #[unique_index("idx_user_email", "email")]
/// struct User {
///     #[autoincrement]
///     id: i32,
///     #[unique]
///     username: String,
///     #[unique]
///     email: Option<String>,
///     #[check("age >= 18")]
///     age: i32,
/// }
///
///
/// #[table]
/// #[constraint("FOREIGN KEY (author_id) REFERENCES user(id) ON DELETE CASCADE")]
/// struct Post {
///     #[autoincrement]
///     id: i32,
///     title: String,
///     content: String,
///     #[foreign_key("user", "id", "CASCADE", "CASCADE")]
///     author_id: i32,
/// }
///
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn table(_attr: TokenStream, input: TokenStream) -> TokenStream {
    table_impl::table(input)
}

/// Marks a field as an auto-incrementing primary key.
/// 
/// # Example
/// 
/// ```
/// use sqlited_derive::autoincrement;
/// 
/// struct User {
///     #[autoincrement]
///     id: i32,
///     name: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn autoincrement(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 不需要解析，直接返回原始 TokenStream
    // 或者添加一些信息然后返回
    
    // 将 TokenStream 转换为 TokenStream2，以便使用 quote
    let input = proc_macro2::TokenStream::from(item);
    
    // 使用 quote 生成新的代码，包含原始代码和一个标记
    let output = quote! {
        #[doc = "autoincrement"]  // 添加一个文档注释作为标记
        #input
    };
    
    // 转换回 TokenStream 并返回
    output.into()
}

/// Marks a field as a primary key (non-autoincrementing).
#[proc_macro_attribute]
pub fn primary_key(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a field as having a UNIQUE constraint.
#[proc_macro_attribute]
pub fn unique(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Adds a CHECK constraint to a field.
///
/// # Example
///
/// ```
/// use sqlited_derive::check;
///
/// struct User {
///     #[check("age >= 18")]
///     age: i32,
/// }
/// ```
#[proc_macro_attribute]
pub fn check(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 这里直接返回原始项，因为我们只需要标记
    item
}

/// Explicitly marks a field as NOT NULL.
#[proc_macro_attribute]
pub fn not_null(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Sets a default value for a field.
///
/// # Example
///
/// ```
/// use sqlited_derive::default;
///
/// struct User {
///     #[default("now")]
///     created_at: UtcDateTime,
/// }
/// ```
#[proc_macro_attribute]
pub fn default(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Sets a default value for a field (alternative to `default` to avoid conflicts with Rust's built-in `default`).
// #[proc_macro_attribute]
// pub fn db_default(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     item
// }

/// Adds a foreign key constraint to a field.
///
/// # Example
///
/// ```
/// use sqlited_derive::foreign_key;
///
/// struct Post {
///     #[foreign_key("users", "id", "CASCADE", "CASCADE")]
///     author_id: i32,
/// }
/// ```
#[proc_macro_attribute]
pub fn foreign_key(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Adds a table-level constraint.
///
/// # Example
///
/// ```
/// use sqlited_derive::constraint;
///
/// #[constraint("UNIQUE(first_name, last_name)")]
/// struct User {
///     id: i32,
///     first_name: String,
///     last_name: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn constraint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Adds an index to a table.
///
/// # Example
///
/// ```
/// use sqlited_derive::index;
///
/// #[index("idx_user_name", "name")]
/// struct User {
///     id: i32,
///     name: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn index(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Adds a unique index to a table.
///
/// # Example
///
/// ```
/// use sqlited_derive::unique_index;
///
/// #[unique_index("idx_user_email", "email")]
/// struct User {
///     id: i32,
///     email: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn unique_index(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// 定义表结构迁移
/// 
/// # 迁移类型
/// 
/// * `add_column` - 添加列：`#[migration("add_column", "column_name")]`
/// * `rename_column` - 重命名列：`#[migration("rename_column", "old_name", "new_name")]`
/// * `modify_column` - 修改列类型：`#[migration("modify_column", "column_name")]`
/// * `drop_column` - 删除列：`#[migration("drop_column", "column_name")]`
/// * `add_index` - 添加索引：`#[migration("add_index", "index_name", "column_name", "UNIQUE")]` (UNIQUE 是可选的)
/// * `drop_index` - 删除索引：`#[migration("drop_index", "index_name")]`
/// * `custom` - 自定义 SQL：`#[migration("custom", "migration_name", "up_sql", "down_sql")]`
/// 
/// # 示例
/// 
/// ```
/// #[table]
/// #[migration("add_column", "bio")]
/// struct User {
///     #[autoincrement]
///     id: i32,
///     name: String,
///     email: String,
///     bio: String,  // 新增的列
/// }
/// 
/// #[table]
/// #[migration("modify_column", "active")]
/// struct Post {
///     #[autoincrement]
///     id: i32,
///     title: String,
///     content: String,
///     #[default("1")]
///     active: bool,  // 从String类型改为bool
/// }
/// ```
#[proc_macro_attribute]
pub fn migration(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}