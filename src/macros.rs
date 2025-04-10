use crate::connection::SqliteConnection;
/// Structure that holds a SQL query and its parameters
pub struct SqlQuery {
    pub query: String,
    pub params: Vec<Box<dyn rusqlite::ToSql>>,
}

// Custom Debug implementation since dyn rusqlite::ToSql doesn't implement Debug
impl std::fmt::Debug for SqlQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlQuery")
            .field("query", &self.query)
            .field("params_count", &self.params.len())
            .finish()
    }
}

impl SqlQuery {
    /// Creates a new SQL query with no parameters
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            params: Vec::new(),
        }
    }

    /// Execute the query on the provided connection
    pub fn execute(&self, conn: &SqliteConnection) -> rusqlite::Result<usize> {
        let param_refs: Vec<&dyn rusqlite::ToSql> = self
            .params
            .iter()
            .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
            .collect();
        conn.execute(&self.query, param_refs.as_slice())
    }

    /// Query multiple rows and map each to a value using the provided function
    pub fn query_map<T, F>(&self, conn: &SqliteConnection, f: F) -> rusqlite::Result<Vec<T>>
    where
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = conn.raw_connection().prepare(&self.query)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = self
            .params
            .iter()
            .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
            .collect();
        let rows = stmt.query_map(param_refs.as_slice(), f)?;
        rows.collect()
    }

    /// Query a single row and map it to a value using the provided function
    pub fn query_row<T, F>(&self, conn: &SqliteConnection, f: F) -> rusqlite::Result<T>
    where
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = conn.raw_connection().prepare(&self.query)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = self
            .params
            .iter()
            .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
            .collect();
        stmt.query_row(param_refs.as_slice(), f)
    }
}

/// Counts the number of columns in a table definition
#[macro_export]
macro_rules! count_columns {
    () => { 0 };
    ($col:ident) => { 1 };
    ($col:ident, $($rest:ident),*) => { 1 + $crate::count_columns!($($rest),*) };
}

/// Helper macro to process default values, particularly for datetime fields
#[macro_export]
macro_rules! process_default_value {
    ($value:expr, $type_name:expr) => {{
        match $type_name {
            "sqlited::types::datetime::UtcDateTime" => {
                match $value {
                    "now" => {
                        // Handle datetime('now') for TEXT fields
                        concat!(" DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))")
                    }
                    _ => {
                        // Handle other default values
                        concat!(" DEFAULT (", $value, ")")
                    }
                }
            }
            "sqlited::types::timestamp::Timestamp" => {
                match $value {
                    "now" => {
                        // Handle 'now' for TEXT fields
                        concat!(" DEFAULT (strftime('%s', 'now'))")
                    }
                    _ => {
                        // Handle other default values
                        concat!(" DEFAULT (", $value, ")")
                    }
                }
            }
            _ => {
                // For other types, use the default value as is
                concat!(" DEFAULT ", $value)
            }
        }
    }};
}

/// Macro for creating SQLite tables
#[macro_export]
macro_rules! table {
    // 基本版本 - 不带自增 ID
    ($name:ident {
        $(
            $(#[default($default_value:expr)])?
            $column:ident: $type:ty
        ),* $(,)?
    }) => {
        use $crate::prelude::*;

        pub struct $name {
            $(pub $column: $type),*
        }

        // 为模型类型实现 Default trait
        impl Default for $name {
            fn default() -> Self {
                $name {
                    $($column: Default::default()),*
                }
            }
        }

        impl WithoutIdTableInfo for $name {
            fn non_id_field_names() -> Vec<&'static str> {
                vec![
                    $(stringify!($column)),*
                ]
            }
        }

        impl $name {
            pub fn create_table_sql() -> String {
                let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (\n", stringify!($name).to_lowercase());
                $(
                    sql.push_str(&format!("    {} {}{}",
                                          stringify!($column).to_lowercase(),
                                          <$type>::sql_type_name(),
                                          $(process_default_value!($default_value, std::any::type_name::<$type>()))?
                                          ));
                    sql.push_str(",\n");
                )*
                sql.pop(); // Remove last comma
                sql.pop(); // Remove last newline
                sql.push_str("\n);");
                sql
            }

            /// Generate SQL for inserting a record
            pub fn insert() -> String {
                let cols = vec![$(stringify!($column).to_lowercase()),*].join(", ");
                let placeholders = vec!["?"; $crate::count_columns!($($column),*)].join(", ");
                format!("INSERT INTO {} ({}) VALUES ({})",
                        stringify!($name).to_lowercase(),
                        cols,
                        placeholders)
            }

            /// Generate SQL for updating a record by its ID
            /// Assumes the first field in the struct is the ID field
            pub fn update() -> String {
                let cols = vec![$(stringify!($column).to_lowercase()),*];
                let table_name = stringify!($name).to_lowercase();
                let id_field = cols[0].clone();

                let set_clause = cols.iter().skip(1)
                    .map(|col| format!("{} = ?", col))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("UPDATE {} SET {} WHERE {} = ?", table_name, set_clause, id_field)
            }

            /// Generate SQL for deleting a record by its ID
            /// Assumes the first field in the struct is the ID field
            pub fn delete() -> String {
                let table_name = stringify!($name).to_lowercase();
                let id_field = stringify!($($column),*).split(',').next().unwrap().trim().to_lowercase();

                format!("DELETE FROM {} WHERE {} = ?", table_name, id_field)
            }

            /// Generate SQL for querying all records
            pub fn query() -> String {
                let cols = vec![$(stringify!($column).to_lowercase()),*].join(", ");
                format!("SELECT {} FROM {}", cols, stringify!($name).to_lowercase())
            }

            /// Generate SQL for querying a record by its ID
            /// Assumes the first field in the struct is the ID field
            pub fn query_by_id() -> String {
                let cols = vec![$(stringify!($column).to_lowercase()),*];
                let table_name = stringify!($name).to_lowercase();
                let id_field = cols[0].clone();

                format!(
                    "SELECT {} FROM {} WHERE {} = ?",
                    cols.join(", "),
                    table_name,
                    id_field
                )
            }

            pub fn to_params(&self) -> Vec<&dyn rusqlite::ToSql> {
                vec![
                    $(&self.$column as &dyn rusqlite::ToSql),*
                ]
            }

            /// Get parameters for update operation (all fields except ID, followed by ID)
            pub fn to_update_params(&self) -> Vec<&dyn rusqlite::ToSql> {
                #[allow(unused_mut)]
                let mut result = vec![];
                $(
                    result.push(&self.$column as &dyn rusqlite::ToSql);
                )*

                // Remove the first element (ID field) and add it at the end for the WHERE clause
                if !result.is_empty() {
                    let id = result.remove(0);
                    result.push(id);
                }

                result
            }

            /// 获取表的所有字段名（不包含 ID 字段）
            pub fn non_id_field_names() -> Vec<&'static str> {
                vec![
                    $(stringify!($column)),*
                ]
            }

            /// 检查字段名是否存在于模型中
            pub const fn has_field(field_name: &str) -> bool {
                // 由于常量函数中不能使用许多标准库功能，我们必须手动实现字符串比较
                // 将待比较的字段名与所有字段一一比较
                let field_names = [$(stringify!($column)),*];

                let mut i = 0;
                while i < field_names.len() {
                    // 将字段名转换为小写进行比较
                    if $crate::macros::manual_str_eq_ignore_case(field_names[i], field_name) {
                        return true;
                    }
                    i += 1;
                }

                false
            }
        }
    };

    // 带自增 ID 的版本
    ($name:ident {
        #[autoincrement]
        $id_column:ident: $id_type:ty,
        $(
            $(#[default($default_value:expr)])?
            $column:ident: $type:ty
        ),* $(,)?
    }) => {
        use $crate::prelude::*;

        pub struct $name {
            pub $id_column: $id_type,
            $(pub $column: $type),*
        }

        // 为模型类型实现 Default trait
        impl Default for $name {
            fn default() -> Self {
                $name {
                    $id_column: Default::default(),
                    $($column: Default::default()),*
                }
            }
        }

        // 实现 WithoutIdTableInfo trait
        impl WithoutIdTableInfo for $name {
            fn non_id_field_names() -> Vec<&'static str> {
                vec![
                    $(stringify!($column)),*
                ]
            }
        }

        impl $name {
            pub fn create_table_sql() -> String {
                // 检查 ID 类型是否是整数类型
                if !<$id_type>::is_integer_type() {
                    panic!("Autoincrement column must be an integer type (i32 or i64)");
                }

                let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (\n", stringify!($name).to_lowercase());

                // ID 列定义为主键自增
                sql.push_str(&format!("    {} {} PRIMARY KEY AUTOINCREMENT,\n",
                                    stringify!($id_column).to_lowercase(),
                                    <$id_type>::sql_type_name()));

                // 其他列定义，包括默认值
                $(
                    // 基本列定义
                    sql.push_str(&format!("    {} {}",
                                        stringify!($column).to_lowercase(),
                                        <$type>::sql_type_name()));

                    // 可选的默认值
                    $(
                        sql.push_str(&$crate::process_default_value!($default_value, std::any::type_name::<$type>()));
                    )?

                    sql.push_str(",\n");
                )*

                sql.pop(); // Remove last comma
                sql.pop(); // Remove last newline
                sql.push_str("\n);");
                sql
            }

            /// Generate SQL for inserting a record with explicit ID
            pub fn insert() -> String {
                let cols = vec![stringify!($id_column).to_lowercase(), $(stringify!($column).to_lowercase()),*].join(", ");
                let placeholders_count = 1 + $crate::count_columns!($($column),*);
                let placeholders = vec!["?"; placeholders_count].join(", ");

                format!("INSERT INTO {} ({}) VALUES ({})",
                        stringify!($name).to_lowercase(),
                        cols,
                        placeholders)
            }

            /// 生成不带 ID 的插入 SQL（利用自增特性）
            pub fn insert_without_id() -> String {
                let cols = vec![$(stringify!($column).to_lowercase()),*].join(", ");
                let placeholders = vec!["?"; $crate::count_columns!($($column),*)].join(", ");

                format!("INSERT INTO {} ({}) VALUES ({})",
                        stringify!($name).to_lowercase(),
                        cols,
                        placeholders)
            }

            /// Generate SQL for inserting with specific columns
            pub fn insert_with(cols: &[&str]) -> String {
                let table_name = stringify!($name).to_lowercase();

                // Filter only the fields that exist
                let valid_fields: Vec<&str> = cols.iter()
                    .filter(|&f| Self::has_field(f))
                    .map(|&f| f)
                    .collect();

                // Join the valid fields
                let cols = valid_fields.join(", ");
                let placeholders = vec!["?"; valid_fields.len()].join(", ");

                format!("INSERT INTO {} ({}) VALUES ({})",
                        table_name, cols, placeholders)
            }

            /// Generate SQL for updating a record by its ID
            pub fn update() -> String {
                let table_name = stringify!($name).to_lowercase();
                let id_field = stringify!($id_column).to_lowercase();

                let set_clause = vec![$(stringify!($column).to_lowercase()),*].iter()
                    .map(|col| format!("{} = ?", col))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("UPDATE {} SET {} WHERE {} = ?", table_name, set_clause, id_field)
            }

            /// Generate SQL for deleting a record by its ID
            pub fn delete() -> String {
                let table_name = stringify!($name).to_lowercase();
                let id_field = stringify!($id_column).to_lowercase();

                format!("DELETE FROM {} WHERE {} = ?", table_name, id_field)
            }

            /// Generate SQL for querying all records
            pub fn query() -> String {
                let cols = vec![stringify!($id_column).to_lowercase(), $(stringify!($column).to_lowercase()),*].join(", ");
                format!("SELECT {} FROM {}", cols, stringify!($name).to_lowercase())
            }

            /// Generate SQL for querying a record by its ID
            pub fn query_by_id() -> String {
                let cols = vec![stringify!($id_column).to_lowercase(), $(stringify!($column).to_lowercase()),*].join(", ");
                let table_name = stringify!($name).to_lowercase();
                let id_field = stringify!($id_column).to_lowercase();

                format!(
                    "SELECT {} FROM {} WHERE {} = ?",
                    cols,
                    table_name,
                    id_field
                )
            }

            pub fn to_params(&self) -> Vec<&dyn ToSql> {
                vec![
                    &self.$id_column as &dyn ToSql,
                    $(&self.$column as &dyn ToSql),*
                ]
            }

            /// 从带有通用 WithoutId 转换为完整的模型
            pub fn from_without_id(id: $id_type, without_id: &WithoutId<$name>) -> Self {
                // 这里需要依照字段名获取字段
                $(
                let $column = match without_id.inner.get(stringify!($column)) {
                    Some(_value) => {
                        // 尝试将 Box<dyn ToSql> 转换为实际类型
                        // 由于类型系统的限制，这里我们简单地返回一个默认值或从临时字符串解析
                        // 在实际使用时，用户应该使用 FromRow trait 从行中获取数据
                        <$type as std::default::Default>::default()
                    },
                    None => <$type as std::default::Default>::default()
                };
                )*

                Self {
                    $id_column: id,
                    $($column),*
                }
            }

            /// Get parameters for update operation (fields followed by ID for WHERE clause)
            pub fn to_update_params(&self) -> Vec<&dyn ToSql> {
                let mut result = vec![
                    $(&self.$column as &dyn ToSql),*
                ];
                // 添加 ID 作为 WHERE 条件的参数
                result.push(&self.$id_column);
                result
            }

            /// 获取表的所有字段名（ID 字段在首位）
            pub fn field_names() -> Vec<&'static str> {
                vec![
                    stringify!($id_column),
                    $(stringify!($column)),*
                ]
            }

            /// 获取表的所有字段名（不包含 ID 字段）
            pub fn non_id_field_names() -> Vec<&'static str> {
                vec![
                    $(stringify!($column)),*
                ]
            }

            /// 检查字段名是否存在于模型中
            pub const fn has_field(field_name: &str) -> bool {
                // 首先检查 ID 字段
                if $crate::macros::manual_str_eq_ignore_case(stringify!($id_column), field_name) {
                    return true;
                }

                // 然后检查其他字段
                let field_names = [$(stringify!($column)),*];

                let mut i = 0;
                while i < field_names.len() {
                    if $crate::macros::manual_str_eq_ignore_case(field_names[i], field_name) {
                        return true;
                    }
                    i += 1;
                }

                false
            }
        }
    };
}

/// 创建新的不带 ID 的记录
#[macro_export]
macro_rules! create_without_id {
    ($model:ty { $($field:ident: $value:expr),* $(,)? }) => {
        {
            let mut without_id = $WithoutId::<$model>::new();
            $(
                without_id.set(stringify!($field), $value);
            )*
            without_id
        }
    };
}

// Define a trait for getting SQLite type names
pub trait SqliteTypeName {
    fn sql_type_name() -> &'static str;

    // 新增：检查类型是否为整数类型（可作为自增主键）
    fn is_integer_type() -> bool {
        false
    }
}

// Implement SqliteTypeName for common types
impl SqliteTypeName for i32 {
    fn sql_type_name() -> &'static str {
        "INTEGER"
    }

    fn is_integer_type() -> bool {
        true
    }
}

impl SqliteTypeName for i64 {
    fn sql_type_name() -> &'static str {
        "INTEGER"
    }

    fn is_integer_type() -> bool {
        true
    }
}

impl SqliteTypeName for f32 {
    fn sql_type_name() -> &'static str {
        "REAL"
    }
}

impl SqliteTypeName for f64 {
    fn sql_type_name() -> &'static str {
        "REAL"
    }
}

impl SqliteTypeName for String {
    fn sql_type_name() -> &'static str {
        "TEXT"
    }
}

impl<T> SqliteTypeName for Option<T>
where
    T: SqliteTypeName,
{
    fn sql_type_name() -> &'static str {
        T::sql_type_name()
    }

    fn is_integer_type() -> bool {
        T::is_integer_type()
    }
}

impl SqliteTypeName for Vec<u8> {
    fn sql_type_name() -> &'static str {
        "BLOB"
    }
}

impl SqliteTypeName for bool {
    fn sql_type_name() -> &'static str {
        "INTEGER"
    }
}

#[macro_export]
macro_rules! sqld {
    // 枚举类型的文本序列化
    (
        enum $enum_type:ident {
            $($variant:ident => $value:expr),+ $(,)?
        }
    ) => {
        impl $crate::SqliteBindableValue for $enum_type {
            fn to_sql_value(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                let value = match self {
                    $(
                        $enum_type::$variant => $value,
                    )+
                };
                Ok(rusqlite::types::ToSqlOutput::from(value))
            }
            
            fn from_sql_value(value: rusqlite::types::ValueRef<'_>) -> Result<Self, rusqlite::types::FromSqlError> {
                use rusqlite::types::FromSql;
                String::column_result(value).and_then(|s| {
                    match s.as_str() {
                        $(
                            $value => Ok($enum_type::$variant),
                        )+
                        _ => Err(rusqlite::types::FromSqlError::InvalidType),
                    }
                })
            }
        }

        // 直接实现 ToSql 特征
        impl rusqlite::ToSql for $enum_type {
            fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                self.to_sql_value()
            }
        }
        
        // 直接实现 FromSql 特征
        impl rusqlite::types::FromSql for $enum_type {
            fn column_result(value: rusqlite::types::ValueRef<'_>) -> Result<Self, rusqlite::types::FromSqlError> {
                Self::from_sql_value(value)
            }
        }
        
        // 实现 SqliteTypeName
        impl $crate::macros::SqliteTypeName for $enum_type {
            fn sql_type_name() -> &'static str {
                "TEXT"
            }
        }
    };

    // 二进制序列化 (使用 bincode)
    (
        binary $type:ty
    ) => {
        impl $crate::SqliteBindableValue for $type {
            fn to_sql_value(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                match bincode::serialize(&self) {
                    Ok(bytes) => Ok(rusqlite::types::ToSqlOutput::from(bytes)),
                    Err(err) => Err(rusqlite::Error::ToSqlConversionFailure(
                        Box::new(err)
                    ))
                }
            }
            
            fn from_sql_value(value: rusqlite::types::ValueRef<'_>) -> Result<Self, rusqlite::types::FromSqlError> {
                use rusqlite::types::FromSql;
                Vec::<u8>::column_result(value).and_then(|bytes| {
                    match bincode::deserialize::<$type>(&bytes) {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err(rusqlite::types::FromSqlError::Other(
                            Box::new(err)
                        ))
                    }
                })
            }

            fn sqlite_type_name() -> &'static str {
                "BLOB"
            }
        }
        
        // 直接实现 ToSql 特征
        impl rusqlite::ToSql for $type {
            fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                self.to_sql_value()
            }
        }
        
        // 直接实现 FromSql 特征
        impl rusqlite::types::FromSql for $type {
            fn column_result(value: rusqlite::types::ValueRef<'_>) -> Result<Self, rusqlite::types::FromSqlError> {
                Self::from_sql_value(value)
            }
        }
        
        // 实现 SqliteTypeName
        impl $crate::macros::SqliteTypeName for $type {
            fn sql_type_name() -> &'static str {
                "BLOB" // 使用 BLOB 类型存储二进制数据
            }
        }
    };
    
    // JSON 序列化版本
    (
        json $type:ty
    ) => {
        impl $crate::SqliteBindableValue for $type {
            fn to_sql_value(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                // 使用 serde_json 序列化为字符串
                match serde_json::to_string(&self) {
                    Ok(json) => Ok(rusqlite::types::ToSqlOutput::from(json)),
                    Err(err) => Err(rusqlite::Error::ToSqlConversionFailure(
                        Box::new(err)
                    ))
                }
            }
            
            fn from_sql_value(value: rusqlite::types::ValueRef<'_>) -> Result<Self, rusqlite::types::FromSqlError> {
                use rusqlite::types::FromSql;
                String::column_result(value).and_then(|json_str| {
                    match serde_json::from_str::<$type>(&json_str) {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err(rusqlite::types::FromSqlError::Other(
                            Box::new(err)
                        ))
                    }
                })
            }
        }
        
        // 直接实现 ToSql 特征
        impl rusqlite::ToSql for $type {
            fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                self.to_sql_value()
            }
        }
        
        // 直接实现 FromSql 特征
        impl rusqlite::types::FromSql for $type {
            fn column_result(value: rusqlite::types::ValueRef<'_>) -> Result<Self, rusqlite::types::FromSqlError> {
                Self::from_sql_value(value)
            }
        }
        
        // 实现 SqliteTypeName
        impl $crate::macros::SqliteTypeName for $type {
            fn sql_type_name() -> &'static str {
                "TEXT"
            }
        }
    };
    
    // 为了向后兼容，保留原 derive_enum_serialized 语法
    (
        $wrapper:ident($enum_type:ident) {
            $($variant:ident => $value:expr),+ $(,)?
        }
    ) => {
        $crate::derive_bindable_value!(
            enum $wrapper($enum_type) {
                $($variant => $value),+
            }
        );
    };
}

/// 表示可以存储到 SQLite 中的自定义类型
pub trait SqliteBindableValue {
    /// 将自定义类型转换为 SQLite 值
    fn to_sql_value(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>>;
    
    /// 从 SQLite 值转换为此类型
    fn from_sql_value(value: rusqlite::types::ValueRef<'_>) -> Result<Self, rusqlite::types::FromSqlError> where Self: Sized;
    
    /// 返回此类型在 SQLite 中的类型名称
    fn sqlite_type_name() -> &'static str {
        "TEXT" // 默认使用 TEXT 类型存储
    }
}

/// 包装器类型，用于实现 FromSql 和 ToSql
#[derive(Debug, Clone)]
pub struct SqliteCustomType<T: SqliteBindableValue + Default + Clone + std::fmt::Debug>(pub T);

impl<T: SqliteBindableValue + Default + Clone + std::fmt::Debug> Default for SqliteCustomType<T> {
    fn default() -> Self {
        Self(T::default())
    }
}

// 为包装器类型实现 FromSql
impl<T: SqliteBindableValue + Default + Clone + std::fmt::Debug> rusqlite::types::FromSql for SqliteCustomType<T> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> Result<Self, rusqlite::types::FromSqlError> {
        T::from_sql_value(value).map(SqliteCustomType)
    }
}

// 为包装器类型实现 ToSql
impl<T: SqliteBindableValue + Default + Clone + std::fmt::Debug> rusqlite::ToSql for SqliteCustomType<T> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.0.to_sql_value()
    }
}

// 为包装器类型实现 SqliteTypeName
impl<T: SqliteBindableValue + Default + Clone + std::fmt::Debug> crate::macros::SqliteTypeName for SqliteCustomType<T> {
    fn sql_type_name() -> &'static str {
        T::sqlite_type_name()
    }
}

/// 常量上下文中不区分大小写的字符串比较函数
///
/// 这个函数用于在编译时比较两个字符串是否相等（忽略大小写）
#[inline]
pub const fn str_eq_ignore_case(s1: &str, s2: &str) -> bool {
    // 首先检查长度是否相同
    if s1.len() != s2.len() {
        return false;
    }

    // 逐字符比较（在常量上下文中没有迭代器和其他高级功能）
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    let mut i = 0;
    while i < s1_bytes.len() {
        let c1 = s1_bytes[i];
        let c2 = s2_bytes[i];

        // 如果字符相同，继续比较
        if c1 == c2 {
            i += 1;
            continue;
        }

        // 如果是大小写字母，尝试转换后比较
        if is_ascii_letter(c1) && is_ascii_letter(c2) {
            // 将两个字符都转为小写形式进行比较
            let c1_lower = to_ascii_lowercase(c1);
            let c2_lower = to_ascii_lowercase(c2);

            if c1_lower != c2_lower {
                return false;
            }
        } else {
            // 非字母字符必须完全相同
            return false;
        }

        i += 1;
    }

    true
}

/// 检查一个字节是否是 ASCII 字母
#[inline]
pub const fn is_ascii_letter(c: u8) -> bool {
    (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z')
}

/// 将 ASCII 字母转为小写
#[inline]
pub const fn to_ascii_lowercase(c: u8) -> u8 {
    if c >= b'A' && c <= b'Z' {
        c + 32 // 将大写字母转换为小写字母
    } else {
        c
    }
}

/// 用于宏模板中的不区分大小写字符串比较函数（公开供 create_table! 宏使用）
#[inline]
pub const fn manual_str_eq_ignore_case(s1: &str, s2: &str) -> bool {
    // 首先检查长度是否相同
    if s1.len() != s2.len() {
        return false;
    }

    // 逐字符比较（在常量上下文中没有迭代器和其他高级功能）
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    let mut i = 0;
    while i < s1_bytes.len() {
        let c1 = s1_bytes[i];
        let c2 = s2_bytes[i];

        // 如果字符相同，继续比较
        if c1 == c2 {
            i += 1;
            continue;
        }

        // 如果是大小写字母，尝试转换后比较
        if is_ascii_letter(c1) && is_ascii_letter(c2) {
            // 将两个字符都转为小写形式进行比较
            let c1_lower = to_ascii_lowercase(c1);
            let c2_lower = to_ascii_lowercase(c2);

            if c1_lower != c2_lower {
                return false;
            }
        } else {
            // 非字母字符必须完全相同
            return false;
        }

        i += 1;
    }

    true
}

/// 通用的 WithoutId 结构体，用于自增 ID 表的插入操作
pub struct WithoutId<T> {
    pub inner: std::collections::HashMap<String, Box<dyn rusqlite::ToSql>>,
    _marker: std::marker::PhantomData<T>,
}

// 自定义 Debug 实现，只显示字段名而非值
impl<T> std::fmt::Debug for WithoutId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithoutId")
            .field("fields", &self.field_names())
            .field("field_count", &self.inner.len())
            .finish()
    }
}

// 自定义 Clone 实现（注意，这只能在值可克隆时工作，否则将会导致运行时错误）
impl<T: 'static> Clone for WithoutId<T> {
    fn clone(&self) -> Self {
        // 创建一个新的 WithoutId 实例
        let mut result = Self::new();

        // 我们无法直接克隆 Box<dyn ToSql>，所以这里我们使用 NULL 值
        for key in self.inner.keys() {
            // Option<String> 实现了 ToSql，并且 None 会被视为 NULL
            result.inner.insert(
                key.clone(),
                Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>,
            );
        }

        result
    }
}

impl<T> WithoutId<T> {
    /// 创建一个空的 WithoutId 结构体
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// 设置字段值
    pub fn set<V: rusqlite::ToSql + 'static>(&mut self, field: &str, value: V) -> &mut Self {
        self.inner.insert(field.to_lowercase(), Box::new(value));
        self
    }

    /// 获取此结构体中的参数列表，按照给定的字段名顺序
    pub fn to_params_ordered(&self, field_names: &[String]) -> Vec<&dyn rusqlite::ToSql> {
        field_names
            .iter()
            .filter_map(|name| {
                self.inner
                    .get(name)
                    .map(|v| v.as_ref() as &dyn rusqlite::ToSql)
            })
            .collect()
    }

    /// 获取此结构体中的参数列表（无序）
    pub fn to_params(&self) -> Vec<&dyn rusqlite::ToSql> {
        self.inner
            .values()
            .map(|v| v.as_ref() as &dyn rusqlite::ToSql)
            .collect()
    }

    /// 获取此结构体包含的字段名
    pub fn field_names(&self) -> Vec<String> {
        self.inner.keys().cloned().collect()
    }

    /// 获取字段值（如果存在）
    pub fn get_field(&self, field_name: &str) -> Option<&dyn rusqlite::ToSql> {
        self.inner
            .get(&field_name.to_lowercase())
            .map(|v| v.as_ref() as &dyn rusqlite::ToSql)
    }

    /// 获取用于插入的参数（自动处理 NULL 值）
    pub fn params_for_insert<M>(&self) -> Vec<&dyn rusqlite::ToSql>
    where
        M: WithoutIdTableInfo,
    {
        let field_names = M::non_id_field_names();
        let mut params = Vec::with_capacity(field_names.len());

        for field_name in field_names {
            if let Some(value) = self.inner.get(&field_name.to_lowercase()) {
                params.push(value.as_ref() as &dyn rusqlite::ToSql);
            } else {
                // 对于缺失的字段，使用 NULL
                params.push(&None::<String> as &dyn rusqlite::ToSql);
            }
        }

        params
    }

    /// 创建一个静态参数持有者，解决借用问题
    /// 这个方法返回一个可以安全传递的参数集合
    pub fn create_static_params_for_insert<M>(&self) -> StaticParamsHolder
    where
        M: WithoutIdTableInfo,
    {
        let field_names = M::non_id_field_names();
        let mut boxed_params = Vec::with_capacity(field_names.len());

        for field_name in field_names {
            if let Some(value) = self.inner.get(&field_name.to_lowercase()) {
                // 这里创建新的 Box<dyn ToSql> 并复制值
                // 对于基本类型，我们可以通过尝试将值转换为各种可能的类型
                let sql_output = value.to_sql().unwrap_or_else(|_| {
                    // 使用 &Value::Null 而不是 Value::Null
                    rusqlite::types::ToSqlOutput::from(&rusqlite::types::Value::Null)
                });

                match sql_output {
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Integer(
                        i,
                    )) => boxed_params.push(Box::new(i) as Box<dyn rusqlite::ToSql>),
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Real(r)) => {
                        boxed_params.push(Box::new(r) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Text(t)) => {
                        boxed_params.push(Box::new(String::from_utf8_lossy(t).into_owned())
                            as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Blob(b)) => {
                        boxed_params.push(Box::new(b.to_vec()) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Null) => {
                        boxed_params
                            .push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(i)) => {
                        boxed_params.push(Box::new(i) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Real(r)) => {
                        boxed_params.push(Box::new(r) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Text(t)) => {
                        boxed_params.push(Box::new(t) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Blob(b)) => {
                        boxed_params.push(Box::new(b) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Null) => {
                        boxed_params
                            .push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>)
                    }
                    // 添加通配符模式以处理未来可能添加的变体
                    _ => boxed_params
                        .push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>),
                }
            } else {
                // 对于缺失的字段，使用 NULL
                boxed_params.push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>);
            }
        }

        // 创建静态参数持有者
        StaticParamsHolder::new(boxed_params)
    }

    /// Creates a static parameter holder only including explicitly provided fields
    pub fn create_static_params_for_fields(
        &self,
        provided_fields: &[String],
    ) -> StaticParamsHolder {
        let mut boxed_params = Vec::with_capacity(provided_fields.len());

        for field_name in provided_fields {
            if let Some(value) = self.inner.get(field_name) {
                // Convert the value to SQL representation as in the original method
                let sql_output = value.to_sql().unwrap_or_else(|_| {
                    rusqlite::types::ToSqlOutput::from(&rusqlite::types::Value::Null)
                });

                match sql_output {
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Integer(
                        i,
                    )) => boxed_params.push(Box::new(i) as Box<dyn rusqlite::ToSql>),
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Real(r)) => {
                        boxed_params.push(Box::new(r) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Text(t)) => {
                        boxed_params.push(Box::new(String::from_utf8_lossy(t).into_owned())
                            as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Blob(b)) => {
                        boxed_params.push(Box::new(b.to_vec()) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Null) => {
                        boxed_params
                            .push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(i)) => {
                        boxed_params.push(Box::new(i) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Real(r)) => {
                        boxed_params.push(Box::new(r) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Text(t)) => {
                        boxed_params.push(Box::new(t) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Blob(b)) => {
                        boxed_params.push(Box::new(b) as Box<dyn rusqlite::ToSql>)
                    }
                    rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Null) => {
                        boxed_params
                            .push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>)
                    }
                    // Add wildcard pattern to handle any future variants
                    _ => boxed_params
                        .push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>),
                }
            }
        }

        // Create static parameter holder
        StaticParamsHolder::new(boxed_params)
    }
}

/// Trait for providing table information to WithoutId
pub trait WithoutIdTableInfo {
    /// Returns a list of field names that are not the ID field
    fn non_id_field_names() -> Vec<&'static str>;

    /// Returns a list of all field names including the ID field
    fn all_field_names() -> Vec<&'static str> {
        let mut fields = vec!["id"];
        fields.extend(Self::non_id_field_names());
        fields
    }

    /// 获取字段的元数据信息（用于增强 IDE 支持）
    fn field_metadata() -> Vec<(&'static str, &'static str)> {
        Self::non_id_field_names()
            .iter()
            .map(|&name| (name, "unknown"))
            .collect()
    }
}

/// A holder for static SQL parameters that safely manages their lifetimes
/// This structure is used to hold SQL parameters and provide them as a slice
/// of references with static lifetimes, solving borrowing issues
pub struct StaticParamsHolder {
    // We store Box<dyn ToSql> to allow for heterogeneous parameter types
    params: Vec<Box<dyn rusqlite::ToSql>>,
    // A cache of static references to the boxed parameters
    static_refs: Vec<&'static dyn rusqlite::ToSql>,
}

impl StaticParamsHolder {
    /// Creates a new instance with the given parameters
    pub fn new(params: Vec<Box<dyn rusqlite::ToSql>>) -> Self {
        let mut holder = Self {
            params,
            static_refs: Vec::new(),
        };

        // Convert the boxed parameters to static references
        holder.create_static_refs();
        holder
    }

    /// Creates static references to the boxed parameters
    fn create_static_refs(&mut self) {
        // Clear any existing references
        self.static_refs.clear();

        // For each boxed parameter, create a static reference using unsafe code
        for param in &self.params {
            // Safety: We're extending the lifetime of the reference to 'static,
            // which is safe as long as StaticParamsHolder lives as long as the references
            let static_param = unsafe {
                std::mem::transmute::<&dyn rusqlite::ToSql, &'static dyn rusqlite::ToSql>(
                    param.as_ref(),
                )
            };
            self.static_refs.push(static_param);
        }
    }

    /// Returns a slice of static references to the SQL parameters
    pub fn as_slice(&self) -> &[&'static dyn rusqlite::ToSql] {
        &self.static_refs
    }

    /// Returns a reference to the boxed parameters
    pub fn params(&self) -> &Vec<Box<dyn rusqlite::ToSql>> {
        &self.params
    }
}

// 修正 AsRef trait 的实现，保持正确的生命周期关系
impl<'a> AsRef<[&'a dyn rusqlite::ToSql]> for StaticParamsHolder {
    fn as_ref(&self) -> &[&'a dyn rusqlite::ToSql] {
        // 我们需要转换生命周期，因为参数持有 'static 生命周期的引用，
        // 但我们需要返回具有调用者请求的 'a 生命周期的引用
        unsafe {
            std::mem::transmute::<&[&'static dyn rusqlite::ToSql], &[&'a dyn rusqlite::ToSql]>(
                &self.static_refs,
            )
        }
    }
}

// 实现 Deref trait，使得 StaticParamsHolder 可以直接解引用为 [&dyn ToSql]
// 这使得它可以直接被用在期望 &[&dyn ToSql] 的上下文中
impl std::ops::Deref for StaticParamsHolder {
    type Target = [&'static dyn rusqlite::ToSql];

    fn deref(&self) -> &Self::Target {
        &self.static_refs
    }
}

// Add these trait implementations

/// Extension trait for StaticParamsHolder to extract params as Vec
pub trait StaticParamsExt {
    fn to_boxed_vec(&self) -> Vec<Box<dyn rusqlite::ToSql>>;
}

impl StaticParamsExt for StaticParamsHolder {
    fn to_boxed_vec(&self) -> Vec<Box<dyn rusqlite::ToSql>> {
        // Convert each parameter based on its SQL representation
        let mut result = Vec::with_capacity(self.params.len());

        for param in &self.params {
            // Get the SQL representation and create a new boxed value
            match param.to_sql().unwrap_or_else(|_| {
                rusqlite::types::ToSqlOutput::from(&rusqlite::types::Value::Null)
            }) {
                rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Integer(i)) => {
                    result.push(Box::new(i) as Box<dyn rusqlite::ToSql>)
                }
                rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Real(r)) => {
                    result.push(Box::new(r) as Box<dyn rusqlite::ToSql>)
                }
                rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Text(t)) => {
                    result.push(Box::new(String::from_utf8_lossy(t).into_owned())
                        as Box<dyn rusqlite::ToSql>)
                }
                rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Blob(b)) => {
                    result.push(Box::new(b.to_vec()) as Box<dyn rusqlite::ToSql>)
                }
                rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Null) => {
                    result.push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>)
                }
                rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(i)) => {
                    result.push(Box::new(i) as Box<dyn rusqlite::ToSql>)
                }
                rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Real(r)) => {
                    result.push(Box::new(r) as Box<dyn rusqlite::ToSql>)
                }
                rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Text(t)) => {
                    result.push(Box::new(t) as Box<dyn rusqlite::ToSql>)
                }
                rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Blob(b)) => {
                    result.push(Box::new(b) as Box<dyn rusqlite::ToSql>)
                }
                rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Null) => {
                    result.push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>)
                }
                // Handle any other case
                _ => result.push(Box::new(Option::<String>::None) as Box<dyn rusqlite::ToSql>),
            }
        }

        result
    }
}

/// Extension trait for StaticParamsHolder params
pub trait ToSqlClone: rusqlite::ToSql {
    fn clone_box(&self) -> Box<dyn rusqlite::ToSql>;
}

// Implement for common types
impl<T: rusqlite::ToSql + Clone + 'static> ToSqlClone for T {
    fn clone_box(&self) -> Box<dyn rusqlite::ToSql> {
        Box::new(self.clone())
    }
}
