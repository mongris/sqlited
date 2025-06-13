use crate::connection::SqliteConnection;
use crate::error::{Result, SqlitedError};
use crate::{FromSql, ToSql};
use std::sync::{LazyLock, Mutex, Arc};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::pool::ConnectionPool;

/// Structure that holds a SQL query and its parameters
pub struct SqlQuery {
    pub query: String,
    pub params: Vec<Box<dyn crate::rq::ToSql>>,
}

// Custom Debug implementation since dyn $crate::rq::ToSql doesn't implement Debug
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
    pub fn execute(&self, conn: &SqliteConnection) -> Result<usize> {
        let param_refs: Vec<&dyn crate::rq::ToSql> = self
            .params
            .iter()
            .map(|p| p.as_ref() as &dyn crate::rq::ToSql)
            .collect();
        conn.execute(&self.query, param_refs.as_slice())
    }

    /// Query multiple rows and map each to a value using the provided function
    pub fn query_map<T, F>(&self, conn: &SqliteConnection, f: F) -> Result<Vec<T>>
    where
        F: FnMut(&crate::Row<'_>) -> crate::rq::Result<T>,
    {
        let param_refs: Vec<&dyn crate::rq::ToSql> = self
            .params
            .iter()
            .map(|p| p.as_ref() as &dyn crate::rq::ToSql)
            .collect();
        conn.query(&self.query, param_refs.as_slice(), f)
            .map_err(SqlitedError::from)
    }

    /// Query a single row and map it to a value using the provided function
    pub fn query_row<T, F>(&self, conn: &SqliteConnection, f: F) -> Result<T>
    where
        F: FnOnce(&crate::Row<'_>) -> crate::rq::Result<T>,
    {
        let param_refs: Vec<&dyn crate::rq::ToSql> = self
            .params
            .iter()
            .map(|p| p.as_ref() as &dyn crate::rq::ToSql)
            .collect();
        conn.query_row(&self.query, param_refs.as_slice(), f)
    }
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

impl<T> SqliteTypeName for Vec<T>
where
    T: FromSql + ToSql
{
    fn sql_type_name() -> &'static str {
        "BLOB"
    }
}

// 为 u64 实现 SqliteTypeName
impl SqliteTypeName for u64 {
    fn sql_type_name() -> &'static str {
        "INTEGER"
    }
    
    fn is_integer_type() -> bool {
        true
    }
}

// 为其他常见无符号整数类型实现
impl SqliteTypeName for u8 {
    fn sql_type_name() -> &'static str {
        "INTEGER"
    }
    
    fn is_integer_type() -> bool {
        true
    }
}

impl SqliteTypeName for u16 {
    fn sql_type_name() -> &'static str {
        "INTEGER"
    }
    
    fn is_integer_type() -> bool {
        true
    }
}

impl SqliteTypeName for u32 {
    fn sql_type_name() -> &'static str {
        "INTEGER"
    }
    
    fn is_integer_type() -> bool {
        true
    }
}

// 为其他常见有符号整数类型实现
impl SqliteTypeName for i8 {
    fn sql_type_name() -> &'static str {
        "INTEGER"
    }
    
    fn is_integer_type() -> bool {
        true
    }
}

impl SqliteTypeName for i16 {
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

impl SqliteTypeName for solana_pubkey::Pubkey {
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
            fn to_sql_value(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                let value = match self {
                    $(
                        $enum_type::$variant => $value,
                    )+
                };
                Ok($crate::rq::types::ToSqlOutput::from(value))
            }
            
            fn from_sql_value(value: $crate::rq::types::ValueRef<'_>) -> Result<Self, $crate::rq::types::FromSqlError> {
                use $crate::rq::types::FromSql;
                String::column_result(value).and_then(|s| {
                    match s.as_str() {
                        $(
                            $value => Ok($enum_type::$variant),
                        )+
                        _ => Err($crate::rq::types::FromSqlError::InvalidType),
                    }
                })
            }

            fn sqlite_type_name() -> &'static str {
                "TEXT"
            }
        }

        // 实现自定义的 sqlited::ToSql 特征
        impl $crate::ToSql for $enum_type {
            fn to_sql(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                self.to_sql_value()
            }

            fn sql_type(&self) -> $crate::rq::types::Type {
                match <Self as $crate::SqliteBindableValue>::sqlite_type_name() {
                    "TEXT" => $crate::rq::types::Type::Text,
                    "INTEGER" => $crate::rq::types::Type::Integer,
                    "REAL" => $crate::rq::types::Type::Real,
                    "BLOB" => $crate::rq::types::Type::Blob,
                    unknown_type => panic!(
                        "从 SqliteBindableValue 获取到不支持的 SQLite 类型名称 '{}' (用于类型 {})",
                        unknown_type,
                        stringify!($enum_type)
                    ),
                }
            }
        }
        
        // 实现自定义的 sqlited::FromSql 特征
        impl $crate::FromSql for $enum_type {
            fn from_sql(value: $crate::rq::types::ValueRef<'_>) -> std::result::Result<Self, $crate::FromSqlError> {
                Self::from_sql_value(value).map_err(Into::into)
            }
        }
        
        // 实现 SqliteTypeName
        impl $crate::macros::SqliteTypeName for $enum_type {
            fn sql_type_name() -> &'static str {
                "TEXT"
            }
        }
    };

    // enum_int $enum_type:ident { ... }
    (
        enum_int $enum_type:ident {
            $($variant:ident => $value:expr),+ $(,)?
        }
    ) => {
        impl $crate::SqliteBindableValue for $enum_type {
            fn to_sql_value(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                match self {
                    $(
                        $enum_type::$variant => Ok($crate::rq::types::ToSqlOutput::from($value as i64)),
                    )+
                }
            }
            
            fn from_sql_value(value: $crate::rq::types::ValueRef<'_>) -> Result<Self, $crate::rq::types::FromSqlError> {
                let val_int = value.as_i64()?;
                match val_int {
                    $(
                        v if v == ($value as i64) => Ok($enum_type::$variant),
                    )+
                    _ => Err($crate::rq::types::FromSqlError::Other(Box::new($crate::error::SqlitedError::Error(anyhow::anyhow!(format!("Invalid integer value {} for enum {}", val_int, stringify!($enum_type))))))),
                }
            }
            
            fn sqlite_type_name() -> &'static str {
                "INTEGER"
            }
        }

        // 实现自定义的 sqlited::ToSql 特征
        impl $crate::ToSql for $enum_type {
            fn to_sql(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                self.to_sql_value()
            }

            fn sql_type(&self) -> $crate::rq::types::Type {
                match <Self as $crate::SqliteBindableValue>::sqlite_type_name() {
                    "TEXT" => $crate::rq::types::Type::Text,
                    "INTEGER" => $crate::rq::types::Type::Integer,
                    "REAL" => $crate::rq::types::Type::Real,
                    "BLOB" => $crate::rq::types::Type::Blob,
                    unknown_type => panic!(
                        "从 SqliteBindableValue 获取到不支持的 SQLite 类型名称 '{}' (用于类型 {})",
                        unknown_type,
                        stringify!($enum_type)
                    ),
                }
            }
        }
        
        // 实现自定义的 sqlited::FromSql 特征
        impl $crate::FromSql for $enum_type {
            fn from_sql(value: $crate::rq::types::ValueRef<'_>) -> std::result::Result<Self, $crate::FromSqlError> {
                Self::from_sql_value(value).map_err(Into::into)
            }
        }
        
        // 实现 SqliteTypeName
        impl $crate::macros::SqliteTypeName for $enum_type {
            fn sql_type_name() -> &'static str {
                "INTEGER"
            }
            fn is_integer_type() -> bool {
                false 
            }
        }
    };

    // 二进制序列化 (使用 bincode)
    (
        binary $type:ty
    ) => {
        impl $crate::SqliteBindableValue for $type {
            fn to_sql_value(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                match $crate::bincode::serialize(&self) {
                    Ok(bytes) => Ok($crate::rq::types::ToSqlOutput::from(bytes)),
                    Err(err) => Err($crate::rq::Error::ToSqlConversionFailure(
                        Box::new(err)
                    ))
                }
            }
            
            fn from_sql_value(value: $crate::rq::types::ValueRef<'_>) -> Result<Self, $crate::rq::types::FromSqlError> {
                use $crate::rq::types::FromSql;
                Vec::<u8>::column_result(value).and_then(|bytes| {
                    match $crate::bincode::deserialize::<$type>(&bytes) {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err($crate::rq::types::FromSqlError::Other(
                            Box::new(err)
                        ))
                    }
                })
            }

            fn sqlite_type_name() -> &'static str {
                "BLOB"
            }
        }
        
        // 实现自定义的 sqlited::ToSql 特征
        impl $crate::ToSql for $type {
            fn to_sql(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                self.to_sql_value()
            }

            fn sql_type(&self) -> $crate::rq::types::Type {
                match <Self as $crate::SqliteBindableValue>::sqlite_type_name() {
                    "TEXT" => $crate::rq::types::Type::Text,
                    "INTEGER" => $crate::rq::types::Type::Integer,
                    "REAL" => $crate::rq::types::Type::Real,
                    "BLOB" => $crate::rq::types::Type::Blob,
                    unknown_type => panic!(
                        "从 SqliteBindableValue 获取到不支持的 SQLite 类型名称 '{}' (用于类型 {})",
                        unknown_type,
                        stringify!($type)
                    ),
                }
            }
        }
        
        // 实现自定义的 sqlited::FromSql 特征
        impl $crate::FromSql for $type {
            fn from_sql(value: $crate::rq::types::ValueRef<'_>) -> std::result::Result<Self, $crate::FromSqlError> {
                Self::from_sql_value(value).map_err(Into::into)
            }
        }
        
        // 实现 SqliteTypeName
        impl $crate::macros::SqliteTypeName for $type {
            fn sql_type_name() -> &'static str {
                "BLOB"
            }
        }
    };

    // 二进制序列化 (使用 borsh)
    (
        borsh $type:ty
    ) => {
        impl $crate::SqliteBindableValue for $type {
            fn to_sql_value(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                match $crate::borsh::to_vec(&self) {
                    Ok(bytes) => Ok($crate::rq::types::ToSqlOutput::from(bytes)),
                    Err(err) => Err($crate::rq::Error::ToSqlConversionFailure(
                        Box::new(err)
                    ))
                }
            }
            
            fn from_sql_value(value: $crate::rq::types::ValueRef<'_>) -> Result<Self, $crate::rq::types::FromSqlError> {
                use $crate::rq::types::FromSql;
                Vec::<u8>::column_result(value).and_then(|bytes| {
                    match $crate::borsh::from_slice::<$type>(&bytes) {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err($crate::rq::types::FromSqlError::Other(
                            Box::new(err)
                        ))
                    }
                })
            }

            fn sqlite_type_name() -> &'static str {
                "BLOB"
            }
        }
        
        // 实现自定义的 sqlited::ToSql 特征
        impl $crate::ToSql for $type {
            fn to_sql(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                self.to_sql_value()
            }

            fn sql_type(&self) -> $crate::rq::types::Type {
                $crate::rq::types::Type::Blob
            }
        }
        
        // 实现自定义的 sqlited::FromSql 特征
        impl $crate::FromSql for $type {
            fn from_sql(value: $crate::rq::types::ValueRef<'_>) -> std::result::Result<Self, $crate::FromSqlError> {
                Self::from_sql_value(value).map_err(Into::into)
            }
        }
        
        // 实现 SqliteTypeName
        impl $crate::macros::SqliteTypeName for $type {
            fn sql_type_name() -> &'static str {
                "BLOB"
            }
        }
    };
    
    // JSON 序列化版本
    (
        json $type:ty
    ) => {
        impl $crate::SqliteBindableValue for $type {
            fn to_sql_value(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                match serde_json::to_string(&self) {
                    Ok(json) => Ok($crate::rq::types::ToSqlOutput::from(json)),
                    Err(err) => Err($crate::rq::Error::ToSqlConversionFailure(
                        Box::new(err)
                    ))
                }
            }
            
            fn from_sql_value(value: $crate::rq::types::ValueRef<'_>) -> Result<Self, $crate::rq::types::FromSqlError> {
                use $crate::rq::types::FromSql;
                String::column_result(value).and_then(|json_str| {
                    match serde_json::from_str::<$type>(&json_str) {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err($crate::rq::types::FromSqlError::Other(
                            Box::new(err)
                        ))
                    }
                })
            }

            fn sqlite_type_name() -> &'static str {
                "TEXT"
            }
        }
        
        // 实现自定义的 sqlited::ToSql 特征
        impl $crate::ToSql for $type {
            fn to_sql(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                self.to_sql_value()
            }

            fn sql_type(&self) -> $crate::rq::types::Type {
                match <Self as $crate::SqliteBindableValue>::sqlite_type_name() {
                    "TEXT" => $crate::rq::types::Type::Text,
                    "INTEGER" => $crate::rq::types::Type::Integer,
                    "REAL" => $crate::rq::types::Type::Real,
                    "BLOB" => $crate::rq::types::Type::Blob,
                    unknown_type => panic!(
                        "从 SqliteBindableValue 获取到不支持的 SQLite 类型名称 '{}' (用于类型 {})",
                        unknown_type,
                        stringify!($type)
                    ),
                }
            }
        }
        
        // 实现自定义的 sqlited::FromSql 特征
        impl $crate::FromSql for $type {
            fn from_sql(value: $crate::rq::types::ValueRef<'_>) -> std::result::Result<Self, $crate::FromSqlError> {
                Self::from_sql_value(value).map_err(Into::into)
            }
        }
        
        // 实现 SqliteTypeName
        impl $crate::macros::SqliteTypeName for $type {
            fn sql_type_name() -> &'static str {
                "TEXT"
            }
        }
    };

    // jsonb $type:ty
    (
        jsonb $type:ty
    ) => {
        impl $crate::SqliteBindableValue for $type {
            fn to_sql_value(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                match $crate::jsonb::to_vec(&self) {
                    Ok(bytes) => Ok($crate::rq::types::ToSqlOutput::from(bytes)),
                    Err(err) => Err($crate::rq::Error::ToSqlConversionFailure(
                        Box::new(err)
                    ))
                }
            }
            
            fn from_sql_value(value: $crate::rq::types::ValueRef<'_>) -> Result<Self, $crate::rq::types::FromSqlError> {
                use $crate::rq::types::FromSql;
                Vec::<u8>::column_result(value).and_then(|bytes| {
                    match $crate::jsonb::from_slice::<$type>(&bytes) {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err($crate::rq::types::FromSqlError::Other(
                            Box::new(err)
                        ))
                    }
                })
            }

            fn sqlite_type_name() -> &'static str {
                "BLOB"
            }
        }
        
        // 实现自定义的 sqlited::ToSql 特征
        impl $crate::ToSql for $type {
            fn to_sql(&self) -> $crate::rq::Result<$crate::rq::types::ToSqlOutput<'_>> {
                self.to_sql_value()
            }

            fn sql_type(&self) -> $crate::rq::types::Type {
                match <Self as $crate::SqliteBindableValue>::sqlite_type_name() {
                    "TEXT" => $crate::rq::types::Type::Text,
                    "INTEGER" => $crate::rq::types::Type::Integer,
                    "REAL" => $crate::rq::types::Type::Real,
                    "BLOB" => $crate::rq::types::Type::Blob,
                    unknown_type => panic!(
                        "从 SqliteBindableValue 获取到不支持的 SQLite 类型名称 '{}' (用于类型 {})",
                        unknown_type,
                        stringify!($type)
                    ),
                }
            }
        }
        
        // 实现自定义的 sqlited::FromSql 特征
        impl $crate::FromSql for $type {
            fn from_sql(value: $crate::rq::types::ValueRef<'_>) -> std::result::Result<Self, $crate::FromSqlError> {
                Self::from_sql_value(value).map_err(Into::into)
            }
        }
        
        // 实现 SqliteTypeName (保持不变)
        impl $crate::macros::SqliteTypeName for $type {
            fn sql_type_name() -> &'static str {
                "BLOB"
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
    fn to_sql_value(&self) -> crate::rq::Result<crate::rq::types::ToSqlOutput<'_>>;
    
    /// 从 SQLite 值转换为此类型
    fn from_sql_value(value: crate::rq::types::ValueRef<'_>) -> anyhow::Result<Self, crate::rq::types::FromSqlError> where Self: Sized;
    
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

// 为包装器类型实现自定义的 sqlited::FromSql 特征
impl<T: SqliteBindableValue + Default + Clone + std::fmt::Debug> crate::FromSql for SqliteCustomType<T> {
    fn from_sql(value: crate::rq::types::ValueRef<'_>) -> std::result::Result<Self, crate::FromSqlError> {
        T::from_sql_value(value)
            .map(SqliteCustomType) // 包装 T 为 SqliteCustomType<T>
            .map_err(Into::into)
    }
}

// 为包装器类型实现自定义的 sqlited::ToSql 特征
impl<T: SqliteBindableValue + Default + Clone + std::fmt::Debug> crate::ToSql for SqliteCustomType<T> {
    fn to_sql(&self) -> crate::rq::Result<crate::rq::types::ToSqlOutput<'_>> {
        // T::to_sql_value() 已经返回 crate::rq::Result<crate::rq::types::ToSqlOutput<'_>>
        self.0.to_sql_value()
    }

    fn sql_type(&self) -> crate::rq::types::Type {
        // 使用 T 实现的 SqliteBindableValue::sqlite_type_name() 进行映射
        match T::sqlite_type_name() {
            "TEXT" => crate::rq::types::Type::Text,
            "INTEGER" => crate::rq::types::Type::Integer,
            "REAL" => crate::rq::types::Type::Real,
            "BLOB" => crate::rq::types::Type::Blob,
            unknown_type => panic!(
                "从 SqliteBindableValue (通过 SqliteCustomType<{}>) 获取到不支持的 SQLite 类型名称 '{}'",
                std::any::type_name::<T>(),
                unknown_type
            ),
        }
    }
}

// 为包装器类型实现 SqliteTypeName
impl<T: SqliteBindableValue + Default + Clone + std::fmt::Debug> crate::macros::SqliteTypeName for SqliteCustomType<T> {
    fn sql_type_name() -> &'static str {
        T::sqlite_type_name()
    }

    fn is_integer_type() -> bool {
        // 假设 SqliteBindableValue 的实现者也会考虑其包装类型的整数性质
        // 如果 T 是整数类型，那么 SqliteCustomType<T> 也应被视为整数类型
        // 为了准确性，这里可以考虑为 SqliteBindableValue 添加 is_integer_type 方法
        // 但如果 SqliteCustomType 主要用于复杂类型，默认为 false 可能更安全
        // 或者，如果 T 实现了 SqliteTypeName，则可以委托给 T::is_integer_type()
        // 鉴于 SqliteBindableValue 目前没有 is_integer_type, 我们需要做一个决定。
        // 选项1: 默认为 false
        // false
        // 选项2: 尝试要求 T 实现 SqliteTypeName (这会增加约束)
        // T::is_integer_type() // 如果 T: SqliteTypeName
        // 选项3: 为 SqliteBindableValue 添加 is_integer_type()
        // T::is_integer_type() // 如果 SqliteBindableValue 有 is_integer_type
        // 当前 SqliteTypeName 已有 is_integer_type, 但 SqliteBindableValue 没有。
        // 如果 T 总是也实现了 SqliteTypeName (例如通过 sqld! 宏)，则可以这样做：
        // (需要 T: SqliteBindableValue + SqliteTypeName + ...)
        // 为了简单起见，并与 SqliteTypeName for Option<T> 行为一致，
        // 如果 T 自身能提供这个信息是最好的。
        // 假设通过 sqld! 宏生成的类型会同时实现 SqliteBindableValue 和 SqliteTypeName。
        // 对于手动实现 SqliteBindableValue 的类型，用户也应考虑实现 SqliteTypeName。
        // 因此，这里可以尝试调用 T::is_integer_type()，但这需要 T: SqliteTypeName 约束。
        // 鉴于 SqliteCustomType<T> 的 T 约束目前只有 SqliteBindableValue，
        // 我们不能直接调用 T::is_integer_type()。
        // 一个折中的办法是，如果 T::sqlite_type_name() 是 "INTEGER"，则返回 true。
        match T::sqlite_type_name() {
            "INTEGER" => true,
            _ => false,
        }
    }
}

/// 通用的 WithoutId 结构体，用于自增 ID 表的插入操作
pub struct WithoutId<T> {
    pub inner: std::collections::HashMap<String, Box<dyn crate::ToSql>>,
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
                Box::new(Option::<String>::None) as Box<dyn crate::ToSql>,
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
    pub fn set<V: crate::ToSql + 'static>(&mut self, field: &str, value: V) -> &mut Self {
        self.inner.insert(field.to_lowercase(), Box::new(value));
        self
    }

    /// 获取此结构体中的参数列表，按照给定的字段名顺序
    pub fn to_params_ordered(&self, field_names: &[String]) -> Vec<&dyn crate::ToSql> {
        field_names
            .iter()
            .filter_map(|name| {
                self.inner
                    .get(name)
                    .map(|v| v.as_ref() as &dyn crate::ToSql)
            })
            .collect()
    }

    /// 获取此结构体中的参数列表（无序）
    pub fn to_params(&self) -> Vec<&dyn crate::ToSql> {
        self.inner
            .values()
            .map(|v| v.as_ref() as &dyn crate::ToSql)
            .collect()
    }

    /// 获取此结构体包含的字段名
    pub fn field_names(&self) -> Vec<String> {
        self.inner.keys().cloned().collect()
    }

    /// 获取字段值（如果存在）
    pub fn get_field(&self, field_name: &str) -> Option<&dyn crate::ToSql> {
        self.inner
            .get(&field_name.to_lowercase())
            .map(|v| v.as_ref() as &dyn crate::ToSql)
    }

    /// 获取用于插入的参数（自动处理 NULL 值）
    pub fn params_for_insert<M>(&self) -> Vec<&dyn crate::ToSql>
    where
        M: WithoutIdTableInfo,
    {
        let field_names = M::non_id_field_names();
        let mut params = Vec::with_capacity(field_names.len());

        for field_name in field_names {
            if let Some(value) = self.inner.get(&field_name.to_lowercase()) {
                params.push(value.as_ref() as &dyn crate::ToSql);
            } else {
                // 对于缺失的字段，使用 NULL
                params.push(&None::<String> as &dyn crate::ToSql);
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
                    crate::rq::types::ToSqlOutput::from(&crate::rq::types::Value::Null)
                });

                match sql_output {
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Integer(
                        i,
                    )) => boxed_params.push(Box::new(i) as Box<dyn crate::rq::ToSql>),
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Real(r)) => {
                        boxed_params.push(Box::new(r) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Text(t)) => {
                        boxed_params.push(Box::new(String::from_utf8_lossy(t).into_owned())
                            as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Blob(b)) => {
                        boxed_params.push(Box::new(b.to_vec()) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Null) => {
                        boxed_params
                            .push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Integer(i)) => {
                        boxed_params.push(Box::new(i) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Real(r)) => {
                        boxed_params.push(Box::new(r) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Text(t)) => {
                        boxed_params.push(Box::new(t) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Blob(b)) => {
                        boxed_params.push(Box::new(b) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Null) => {
                        boxed_params
                            .push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>)
                    }
                    // 添加通配符模式以处理未来可能添加的变体
                    _ => boxed_params
                        .push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>),
                }
            } else {
                // 对于缺失的字段，使用 NULL
                boxed_params.push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>);
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
                    crate::rq::types::ToSqlOutput::from(&crate::rq::types::Value::Null)
                });

                match sql_output {
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Integer(
                        i,
                    )) => boxed_params.push(Box::new(i) as Box<dyn crate::rq::ToSql>),
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Real(r)) => {
                        boxed_params.push(Box::new(r) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Text(t)) => {
                        boxed_params.push(Box::new(String::from_utf8_lossy(t).into_owned())
                            as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Blob(b)) => {
                        boxed_params.push(Box::new(b.to_vec()) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Null) => {
                        boxed_params
                            .push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Integer(i)) => {
                        boxed_params.push(Box::new(i) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Real(r)) => {
                        boxed_params.push(Box::new(r) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Text(t)) => {
                        boxed_params.push(Box::new(t) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Blob(b)) => {
                        boxed_params.push(Box::new(b) as Box<dyn crate::rq::ToSql>)
                    }
                    crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Null) => {
                        boxed_params
                            .push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>)
                    }
                    // Add wildcard pattern to handle any future variants
                    _ => boxed_params
                        .push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>),
                }
            }
        }

        // Create static parameter holder
        StaticParamsHolder::new(boxed_params)
    }
}

/// Trait for providing table information to WithoutId
pub trait WithoutIdTableInfo {
    /// 返回表名
    fn table_name() -> &'static str;

    /// 返回表的所有字段名列表
    fn field_names() -> Vec<&'static str>;

    /// 返回表的字段类型列表
    fn field_types() -> Vec<(&'static str, &'static str)>;

    /// 生成创建表的 SQL 语句，包含全部约束和索引
    fn create_table_sql() -> String;

    /// 返回除 id 字段外的所有字段名
    fn non_id_field_names() -> Vec<&'static str> {
        Self::field_names().into_iter()
            .filter(|&name| name != "id")
            .collect()
    }

    /// 返回所有字段名（包括 id）
    fn all_field_names() -> Vec<&'static str> {
        Self::field_names()
    }

    /// 生成不带 id 字段的插入 SQL 语句
    fn insert_without_id() -> String {
        let table_name = Self::table_name();
        let field_names: Vec<&str> = Self::field_names().into_iter()
            .filter(|&f| f != "id")
            .collect();
        
        if field_names.is_empty() {
            return format!("INSERT INTO {} DEFAULT VALUES", table_name);
        }
        
        let placeholders: Vec<&str> = field_names.iter()
            .map(|_| "?")
            .collect();
        
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            field_names.join(", "),
            placeholders.join(", ")
        )
    }

    /// 生成指定字段的插入 SQL 语句
    fn insert_with(fields: &[&str]) -> String {
        let table_name = Self::table_name();
        
        if fields.is_empty() {
            return format!("INSERT INTO {} DEFAULT VALUES", table_name);
        }
        
        let placeholders: Vec<&str> = fields.iter()
            .map(|_| "?")
            .collect();
        
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            fields.join(", "),
            placeholders.join(", ")
        )
    }

    /// 获取字段的元数据信息（用于增强 IDE 支持）
    fn field_metadata() -> Vec<(&'static str, &'static str)> {
        Self::field_types()
    }
    
    /// 检查表是否包含指定字段
    fn has_field(field_name: &str) -> bool {
        Self::field_names()
            .iter()
            .any(|&f| f.eq_ignore_ascii_case(field_name))
    }
    
    /// 获取指定字段的 SQLite 类型
    fn field_type(field_name: &str) -> Option<&'static str> {
        Self::field_types()
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(field_name))
            .map(|(_, type_name)| *type_name)
    }
    
    /// 判断指定字段是否为 ID 字段
    fn is_id_field(field_name: &str) -> bool {
        field_name.eq_ignore_ascii_case("id")
    }
    
    /// 获取表中所有索引的定义 SQL
    fn index_definitions() -> Vec<String> {
        // 默认实现为空 - 在启用索引支持的 table! 实现中会被覆盖
        Vec::new()
    }
    
    /// 获取表中所有约束的定义 SQL
    fn constraint_definitions() -> Vec<String> {
        // 默认实现为空 - 在启用约束支持的 table! 实现中会被覆盖
        Vec::new()
    }
}

/// A holder for static SQL parameters that safely manages their lifetimes
/// This structure is used to hold SQL parameters and provide them as a slice
/// of references with static lifetimes, solving borrowing issues
pub struct StaticParamsHolder {
    // We store Box<dyn ToSql> to allow for heterogeneous parameter types
    params: Vec<Box<dyn crate::rq::ToSql>>,
    // A cache of static references to the boxed parameters
    static_refs: Vec<&'static dyn crate::rq::ToSql>,
}

impl StaticParamsHolder {
    /// Creates a new instance with the given parameters
    pub fn new(params: Vec<Box<dyn crate::rq::ToSql>>) -> Self {
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
                std::mem::transmute::<&dyn crate::rq::ToSql, &'static dyn crate::rq::ToSql>(
                    param.as_ref(),
                )
            };
            self.static_refs.push(static_param);
        }
    }

    /// Returns a slice of static references to the SQL parameters
    pub fn as_slice(&self) -> &[&'static dyn crate::rq::ToSql] {
        &self.static_refs
    }

    /// Returns a reference to the boxed parameters
    pub fn params(&self) -> &Vec<Box<dyn crate::rq::ToSql>> {
        &self.params
    }
}

// 修正 AsRef trait 的实现，保持正确的生命周期关系
impl<'a> AsRef<[&'a dyn crate::rq::ToSql]> for StaticParamsHolder {
    fn as_ref(&self) -> &[&'a dyn crate::rq::ToSql] {
        // 我们需要转换生命周期，因为参数持有 'static 生命周期的引用，
        // 但我们需要返回具有调用者请求的 'a 生命周期的引用
        unsafe {
            std::mem::transmute::<&[&'static dyn crate::rq::ToSql], &[&'a dyn crate::rq::ToSql]>(
                &self.static_refs,
            )
        }
    }
}

// 实现 Deref trait，使得 StaticParamsHolder 可以直接解引用为 [&dyn ToSql]
// 这使得它可以直接被用在期望 &[&dyn ToSql] 的上下文中
impl std::ops::Deref for StaticParamsHolder {
    type Target = [&'static dyn crate::rq::ToSql];

    fn deref(&self) -> &Self::Target {
        &self.static_refs
    }
}

// Add these trait implementations

/// Extension trait for StaticParamsHolder to extract params as Vec
pub trait StaticParamsExt {
    fn to_boxed_vec(&self) -> Vec<Box<dyn crate::rq::ToSql>>;
}

impl StaticParamsExt for StaticParamsHolder {
    fn to_boxed_vec(&self) -> Vec<Box<dyn crate::rq::ToSql>> {
        // Convert each parameter based on its SQL representation
        let mut result = Vec::with_capacity(self.params.len());

        for param in &self.params {
            // Get the SQL representation and create a new boxed value
            match param.to_sql().unwrap_or_else(|_| {
                crate::rq::types::ToSqlOutput::from(&crate::rq::types::Value::Null)
            }) {
                crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Integer(i)) => {
                    result.push(Box::new(i) as Box<dyn crate::rq::ToSql>)
                }
                crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Real(r)) => {
                    result.push(Box::new(r) as Box<dyn crate::rq::ToSql>)
                }
                crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Text(t)) => {
                    result.push(Box::new(String::from_utf8_lossy(t).into_owned())
                        as Box<dyn crate::rq::ToSql>)
                }
                crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Blob(b)) => {
                    result.push(Box::new(b.to_vec()) as Box<dyn crate::rq::ToSql>)
                }
                crate::rq::types::ToSqlOutput::Borrowed(crate::rq::types::ValueRef::Null) => {
                    result.push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>)
                }
                crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Integer(i)) => {
                    result.push(Box::new(i) as Box<dyn crate::rq::ToSql>)
                }
                crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Real(r)) => {
                    result.push(Box::new(r) as Box<dyn crate::rq::ToSql>)
                }
                crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Text(t)) => {
                    result.push(Box::new(t) as Box<dyn crate::rq::ToSql>)
                }
                crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Blob(b)) => {
                    result.push(Box::new(b) as Box<dyn crate::rq::ToSql>)
                }
                crate::rq::types::ToSqlOutput::Owned(crate::rq::types::Value::Null) => {
                    result.push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>)
                }
                // Handle any other case
                _ => result.push(Box::new(Option::<String>::None) as Box<dyn crate::rq::ToSql>),
            }
        }

        result
    }
}

/// Extension trait for StaticParamsHolder params
pub trait ToSqlClone: crate::rq::ToSql {
    fn clone_box(&self) -> Box<dyn crate::rq::ToSql>;
}

// Implement for common types
impl<T: crate::rq::ToSql + Clone + 'static> ToSqlClone for T {
    fn clone_box(&self) -> Box<dyn crate::rq::ToSql> {
        Box::new(self.clone())
    }
}

/// 全局连接池管理，用于在多个打开同一数据库文件的请求间共享连接
pub static CONNECTION_POOLS: LazyLock<Mutex<HashMap<PathBuf, Arc<ConnectionPool>>>> = 
    LazyLock::new(|| Mutex::new(HashMap::new()));

// 在宏中添加这个函数来替代原来的MD5计算
pub fn get_statement_key(statement: &str) -> String {
    let statement = statement.trim().to_lowercase();
    
    // 检测是否是CREATE TABLE语句
    if statement.starts_with("create table") {
        // 提取表名
        let parts: Vec<&str> = statement.split_whitespace().collect();
        if parts.len() >= 3 {
            let mut table_name = parts[2];
            // 处理"IF NOT EXISTS"
            if table_name == "if" && parts.len() >= 6 {
                table_name = parts[5];
            }
            
            // 移除引号
            table_name = table_name.trim_matches(|c| c == '"' || c == '`' || c == '\'');
            
            // 使用表名作为哈希键的一部分
            return format!("create_table:{}", table_name);
        }
    }
    
    // 对其他SQL语句使用完整哈希
    format!("sql:{:x}", md5::compute(statement))
}

/// 定义数据库结构、表和迁移
/// 此宏允许定义带有自定义类型的数据库，使你可以为数据库结构实现自定义方法。
///
/// # 示例
///
/// ```rust
/// #[table]
/// struct User {
///   #[autoincrement]
///   id: i64,
///   name: String,
///   email: String
/// }
/// define_db!(
///   pub static ref USER_DB: UserDb<()> = [
///     User,
///     "CREATE INDEX IF NOT EXISTS users_email_idx ON users(email)"
///   ]
/// );
/// let db = USER_DB::open(path_to_db);
/// let memory_db = USER_DB::memory();
/// 
/// // 添加自定义方法
/// impl UserDb {
///     pub fn get_user_by_name(&self, name: &str) -> Result<User> {
///         self.query_row("SELECT * FROM user WHERE name = ?", [name], User::from_row)
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_db {
    // Rule 1: No generics, WITH init query
    // Delegates to Rule 2 (main implementation with generics)
    (
        pub static ref $id:ident: $t:ident = [
            $( $element:tt ),* $(,)?
        ],
        $init_query:expr
    ) => {
        $crate::define_db!(
            pub static ref $id: $t<()> = [
                $( $element ),*
            ],
            $init_query
        );
    };
    // Rule 2: With generics, WITH init query (MAIN IMPLEMENTATION)
    (
        pub static ref $id:ident: $t:ident<$($d:ty),*> = [
            $( $element:tt ),* $(,)?
        ],
        $init_query:expr // The new optional initial query parameter
    ) => {
        // 定义自定义结构体，包装 Database
        #[derive(Clone)]
        pub struct $t {
            db: Database
        }

        // 实现 Deref 以提供对底层数据库的访问
        impl std::ops::Deref for $t {
            type Target = Database;

            fn deref(&self) -> &Self::Target {
                &self.db
            }
        }

        // 基本 Database 结构体定义
        #[derive(Clone)]
        pub struct Database {
            pool: std::sync::Arc<$crate::pool::ConnectionPool>,
        }
        
        impl Database {
            fn new(pool: std::sync::Arc<$crate::pool::ConnectionPool>) -> Self {
                Self {
                    pool,
                }
            }

            // Helper method to get the initial query string provided to the macro
            fn get_initial_query() -> String {
                String::from($init_query)
            }

            // Helper to get a connection (internal use)
            fn get_conn(&self) -> $crate::error::Result<$crate::connection::SqliteConnection> {
                $crate::connection::get_connection(&self.pool)
            }

            /// Execute a raw SQL query and return the number of rows affected
            pub fn execute<P: $crate::rq::Params>(&self, query: &str, params: P) -> $crate::error::Result<usize> {
                let conn = self.get_conn()?;
                conn.execute(query, params)
            }

            pub fn execute2(&self, query: &str, params: StaticParamsHolder) -> $crate::error::Result<usize> {
                let conn = self.get_conn()?;
                conn.execute2(query, params)
            }

            /// Execute an INSERT query and return the last inserted row ID.
            /// Ensures the row ID is retrieved from the same connection used for the insert.
            pub fn execute_insert<P: $crate::rq::Params>(&self, query: &str, params: P) -> $crate::error::Result<i64> {
                let conn = self.get_conn()?; // Get a connection
                conn.execute(query, params)?; // Execute the insert on this connection
                Ok(conn.last_insert_rowid()) // Get rowid from the *same* connection
            }

            pub fn execute_insert2(&self, query: &str, params: StaticParamsHolder) -> $crate::error::Result<i64> {
                let conn = self.get_conn()?; // Get a connection
                conn.execute2(query, params)?; // Execute the insert on this connection
                Ok(conn.last_insert_rowid()) // Get rowid from the *same* connection
            }

            /// Execute a raw SQL query and return the rows as a statement
            pub fn query<F, T, P: $crate::rq::Params>(&self, query: &str, params: P, map_fn: F) -> $crate::error::Result<Vec<T>>
            where
                F: FnMut(&$crate::Row) -> $crate::rq::Result<T>,
            {
                let conn = self.get_conn()?;
                conn.query(query, params, map_fn)
            }

            pub fn query2<F, T>(&self, query: &str, params: StaticParamsHolder, map_fn: F) -> $crate::error::Result<Vec<T>>
            where
                F: FnMut(&$crate::Row) -> $crate::rq::Result<T>,
            {
                let conn = self.get_conn()?;
                conn.query2(query, params, map_fn)
            }

            /// Query a single row
            pub fn query_row<P, F, T>(&self, sql: &str, params: P, f: F) -> $crate::error::Result<T>
            where
                P: $crate::rq::Params,
                F: FnOnce(&$crate::Row<'_>) -> $crate::rq::Result<T>,
            {
                let conn = self.get_conn()?;
                conn.query_row(sql, params, f)
            }

            pub fn query_row2<F, T>(&self, sql: &str, params: StaticParamsHolder, f: F) -> $crate::error::Result<T>
            where
                F: FnOnce(&$crate::Row<'_>) -> $crate::rq::Result<T>,
            {
                let conn = self.get_conn()?;
                conn.query_row2(sql, params, f)
            }

            /// Get the last inserted row ID.
            pub fn last_insert_rowid(&self) -> $crate::error::Result<i64> {
                let conn = self.get_conn()?;
                Ok(conn.last_insert_rowid()) // Wrap in Ok as get_conn can fail
            }

            // ... other direct connection methods if needed ...

            pub fn raw_pool(&self) -> &std::sync::Arc<$crate::pool::ConnectionPool> {
                &self.pool
            }

            // 获取表的所有迁移
            fn get_all_table_migrations() -> Vec<(String, String, Option<String>)> {
                let mut migrations = Vec::new();
                
                // 收集所有表的迁移
                $(
                    if let Some(table_migrations) = $crate::_collect_table_migrations!($element) {
                        migrations.extend(table_migrations);
                    }
                )*
                
                migrations
            }
            
            // 返回迁移列表
            fn get_migrations() -> Vec<String> {
                vec![
                    $(
                        $crate::_resolve_migration_element!($element),
                    )*
                ]
            }
            
            // 应用迁移到此数据库
            pub fn apply_migrations(&self) -> $crate::error::Result<()> {
                // Get a connection specifically for applying migrations
                let mut conn = self.get_conn()?;

                // 创建迁移表（如果不存在）
                conn.execute(
                    "CREATE TABLE IF NOT EXISTS _sqlited_migrations (
                        id INTEGER PRIMARY KEY,
                        name TEXT NOT NULL,
                        applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
                    )",
                    [],
                )?;

                // 获取所有表定义的迁移
                let table_migrations = Self::get_all_table_migrations();

                let mut tx = conn.raw_connection_mut().transaction()?; 
                
                let mut success = true;

                // 首先应用表迁移
                for (name, up_sql, _) in table_migrations {
                    if name.starts_with("error") {
                        println!("Skipping invalid migration: {}", up_sql);
                        continue;
                    }
                    
                    let already_applied = tx.query_row(
                        "SELECT COUNT(*) FROM _sqlited_migrations WHERE name = ?",
                        [&name],
                        |row| row.get::<_, i32>(0),
                    ).unwrap_or(0) > 0;
                    
                    if !already_applied {
                        // 按分号拆分多个 SQL 语句
                        let statements = up_sql.split(';')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<_>>();
                        
                        for statement in &statements {
                            match tx.execute(statement, []) {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("Failed to apply migration {}: {}", name, e);
                                    success = false;
                                    break;
                                }
                            }
                        }
                        
                        // 记录已应用的迁移
                        if success {
                            if let Err(e) = tx.execute(
                                "INSERT INTO _sqlited_migrations (name) VALUES (?)",
                                [&name],
                            ) {
                                eprintln!("Failed to record migration {}: {}", name, e);
                                success = false;
                            }
                        }
                    }
                    
                    if !success {
                        break;
                    }
                }

                // 按顺序应用其他SQL迁移
                if success {
                    for migration in Self::get_migrations() {
                        // 按分号拆分多个 SQL 语句
                        let statements = migration.split(';')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<_>>();
                        
                        for statement in &statements {
                            if statement.is_empty() {
                                continue;
                            }
                            
                            // 对每条语句单独应用迁移逻辑
                            let statement_hash = $crate::macros::get_statement_key(statement);

                            // query_row now returns crate::error::Result, use ?
                            let count = tx.query_row(
                                "SELECT COUNT(*) FROM _sqlited_migrations WHERE name = ?",
                                [&statement_hash],
                                |row| row.get::<_, i32>(0),
                            ).unwrap_or(0); // Keep unwrap_or(0) as fallback if query fails finding row
                            
                            if count == 0 {
                                match tx.execute(statement, []) {
                                    Ok(_) => {
                                        // 记录已应用的迁移
                                        if let Err(e) = tx.execute(
                                            "INSERT INTO _sqlited_migrations (name) VALUES (?)",
                                            [&statement_hash],
                                        ) {
                                            eprintln!("Failed to record migration: {}", e);
                                            success = false;
                                            break;
                                        }
                                    },
                                    Err(e) => {
                                        eprintln!("Failed to apply migration: {}", e);
                                        success = false;
                                        break;
                                    }
                                }
                            }
                        }

                        if !success { break; } // Exit outer loop on failure
                    }
                }
                
                // 根据迁移结果提交或回滚
                if success {
                    tx.commit()?;
                } else {
                    // Rollback happens automatically on drop if not committed,
                    // but explicit rollback is fine too. We still need to return an error.
                    // tx.rollback()?; // Optional explicit rollback
                    return Err($crate::error::SqlitedError::Rusqlite($crate::rq::Error::SqliteFailure(
                        $crate::rq::ffi::Error {
                            code: $crate::rq::ffi::ErrorCode::InternalMalfunction,
                            extended_code: 1
                        },
                        Some("Failed to apply migrations".to_string())
                    )));
                }
                
                Ok(())
            }
            
            /// 返回一个新的连接到同一数据库
            pub fn new_connection(&self) -> $crate::error::Result<Self> {
                // get_connection already returns the correct Result type
                Ok(Self::new(self.pool.clone()))
            }
            
            /// 在事务中执行闭包，自动处理提交和回滚
            pub fn transaction<T, F>(&self, f: F) -> $crate::error::Result<T>
            where
                F: FnOnce(&mut $crate::rq::Transaction) -> $crate::error::Result<T>,
            {
                let mut conn = self.get_conn()?; // Get a connection for the transaction

                // Use the underlying rusqlite connection to start the transaction
                let mut tx = conn.raw_connection_mut().transaction()?;

                match f(&mut tx) { // Pass the rusqlite transaction to the closure
                    Ok(result) => {
                        tx.commit().map_err($crate::error::SqlitedError::from)?;
                        Ok(result)
                    },
                    Err(e) => {
                        // Rollback is automatic on drop if commit fails or isn't called.
                        // Explicit rollback is optional: tx.rollback().ok();
                        Err(e) // Propagate the original error
                    }
                }
            }
        }
        
        // 为自定义类型提供方法
        #[allow(non_camel_case_types)]
        impl $t {
            /// 打开给定路径的数据库（如果为None则使用内存模式）
            fn _open(path: Option<impl AsRef<std::path::Path>>) -> $crate::error::Result<Self> {
                let pool_result = match path {
                    Some(p) => {
                        let path_buf = p.as_ref().to_path_buf();
                        let canonical_path = if path_buf.exists() {
                            std::fs::canonicalize(&path_buf).unwrap_or(path_buf)
                        } else {
                            // Ensure directory creation maps error correctly
                            if let Some(parent) = path_buf.parent() {
                                std::fs::create_dir_all(parent)
                                    .map_err(|e| $crate::error::SqlitedError::Rusqlite($crate::rq::Error::SqliteFailure(
                                        $crate::rq::ffi::Error {
                                            code: $crate::rq::ffi::ErrorCode::CannotOpen,
                                            extended_code: 1
                                        },
                                        Some(format!("Failed to create database directory: {}", e))
                                    )))?;
                            }
                            path_buf
                        };
                        
                        // 尝试从连接池缓存获取
                        let pool = {
                            let mut pools = $crate::CONNECTION_POOLS.lock().unwrap();
                            if let Some(existing_pool) = pools.get(&canonical_path) {
                                existing_pool.clone()
                            } else {
                                let new_pool = $crate::pool::ConnectionPool::new(&canonical_path, Database::get_initial_query())
                                    .map(std::sync::Arc::new)
                                    .map_err($crate::error::SqlitedError::from)?; // Use From trait
                                pools.insert(canonical_path, new_pool.clone());
                                new_pool
                            }
                        };
                        Ok(pool)
                    },
                    None => {
                        // new_memory_pool returns Result<_, PoolError>, map it
                        $crate::connection::new_memory_pool().map(std::sync::Arc::new)
                    }
                };

                let pool = pool_result?;
                let db = Database::new(pool);
                db.apply_migrations()?; // Apply migrations using the pool
                Ok(Self { db }) // Create the custom wrapper struct
            }

            /// 打开指定路径的数据库
            pub fn open(path: impl AsRef<std::path::Path>) -> $crate::Result<Self> {
                Self::_open(Some(path))
            }
            
            /// 打开内存数据库
            pub fn memory() -> $crate::Result<Self> {
                Self::_open(None::<&std::path::Path>)
            }
            
            /// 在临时位置创建数据库
            pub fn temp() -> $crate::Result<Self> {
                let temp_dir = std::env::temp_dir();
                let db_file = temp_dir.join(format!("sqlited_{}.db", uuid::Uuid::new_v4()));
                Self::_open(Some(db_file))
            }
            
            /// 打开共享内存数据库（使用命名内存数据库）
            pub fn shared_memory(name: &str) -> $crate::Result<Self> {
                // 正确的共享内存语法，注意必须以 "file:" 开头
                let memory_path = format!("file:{}?mode=memory&cache=shared", name);
                
                // 使用标准 open 方法
                Self::_open(Some(memory_path))
            }
            
            /// Get a new instance sharing the same database pool
            pub fn new_connection(&self) -> $crate::error::Result<Self> {
                // Ask the inner Database to create a new instance with the same pool
                let new_db = self.db.new_connection()?;
                Ok(Self { db: new_db })
            }
            
            /// Perform operations within a transaction.
            /// The closure receives a reference to the custom DB type (`&Self`),
            /// but operations inside should use the provided `rusqlite::Transaction`.
            /// NOTE: This signature might be less intuitive now. Consider changing
            /// the closure to `FnOnce(&mut $crate::rq::Transaction) -> $crate::error::Result<T>`
            /// for clarity, matching the underlying `Database::transaction`.
            pub fn transaction<T, F>(&self, f: F) -> $crate::error::Result<T>
            where
                // Option 1: Keep original signature (closure needs to call self.db.transaction internally) - Less direct
                // F: FnOnce(&Self) -> $crate::error::Result<T>,
                // Option 2: Change signature for clarity (Recommended)
                F: FnOnce(&mut $crate::rq::Transaction) -> $crate::error::Result<T>,
            {
                // Delegate directly to the inner Database's transaction method
                self.db.transaction(f)

                // If using Option 1 signature:
                // self.db.transaction(|tx_self| { // tx_self is &Database
                //     // Need a way to run the user's closure `f` which expects `&Self`
                //     // This becomes awkward. Option 2 is better.
                // })
            }
        }
        
        // 为保持与现有代码兼容，定义类型别名
        #[allow(non_upper_case_globals)]
        #[allow(non_camel_case_types)]
        pub type $id = $t;
    };

    // Rule 3: No generics, NO init query
    // Delegates to Rule 2, providing an empty string for init_query
    (
        pub static ref $id:ident: $t:ident = [
            $( $element:tt ),* $(,)?
        ]
    ) => {
        $crate::define_db!(
            pub static ref $id: $t<()> = [
                $( $element ),*
            ],
            "" // Default empty string for init query
        );
    };

    // Rule 4: With generics, NO init query
    // Delegates to Rule 2 (main implementation), providing an empty string for init_query
    (
        pub static ref $id:ident: $t:ident<$($d:ty),*> = [
            $( $element:tt ),* $(,)?
        ]
    ) => {
        $crate::define_db!(
            pub static ref $id: $t<$($d),*> = [
                $( $element ),*
            ],
            "" // Default empty string for init query
        );
    };
}

/// 处理迁移元素的辅助宏
#[macro_export]
#[doc(hidden)]
macro_rules! _process_migration_element {
    // 对于table!宏调用，使用create_table_sql()
    (table!($table_name:ident { $($rest:tt)* })) => {
        $table_name::create_table_sql()
    };
    
    // 对于原始字符串，按原样使用
    ($sql:expr) => {
        $sql
    };
}

/// 提取表定义的辅助宏
#[macro_export]
#[doc(hidden)]
macro_rules! _extract_table_definition {
    // 对于table!宏调用，定义表
    (table!($table_name:ident { $($rest:tt)* })) => {
        table!($table_name { $($rest)* });
    };
    
    // 对于原始字符串，不做任何操作
    ($sql:expr) => {};
}

/// 提取表字段的辅助宏
#[macro_export]
#[doc(hidden)]
macro_rules! _extract_table_field {
    // 对于table!宏调用，创建字段
    (table!($table_name:ident { $($rest:tt)* })) => {
        pub $table_name: $table_name,
    };
    
    // 对于原始字符串，不做任何操作
    ($sql:expr) => {};
}

/// 提取表实例化的辅助宏
#[macro_export]
#[doc(hidden)]
macro_rules! _extract_table_instance {
    // 对于table!宏调用，创建实例
    (table!($table_name:ident { $($rest:tt)* }), $conn:expr) => {
        $table_name: $table_name::default(),
    };
    
    // 对于原始字符串，不做任何操作
    ($sql:expr, $conn:expr) => {};
}

/// 注册 sqlited 的属性宏
/// 这个宏必须在使用 table! 宏之前调用，以确保所有自定义属性都在作用域内
#[macro_export]
macro_rules! register_attribute_macros {
    () => {
        // 属性宏声明（这些不会生成任何代码，只是对编译器的提示）
        #[allow(unused_attributes)]
        const _: () = {
            // 这里声明所有可能用到的属性，告诉编译器这些是合法的属性名
            struct __AttrAutoincrement;
            struct __AttrPrimaryKey;
            struct __AttrUnique;
            struct __AttrCheck;
            struct __AttrNotNull;
            struct __AttrDefault;
            struct __AttrForeignKey;
            struct __AttrDbDefault;
            struct __AttrIndex;
            struct __AttrUniqueIndex;
            struct __AttrConstraint;
            
            // 用空元组作为返回值，避免未使用警告
            ()
        };
        
        // 自定义属性声明
        #[allow(non_snake_case)]
        trait AttributeDefs {
            #[allow(unused_attributes)]
            const autoincrement: () = ();
            #[allow(unused_attributes)]
            const primary_key: () = ();
            #[allow(unused_attributes)]
            const unique: () = ();
            #[allow(unused_attributes)]
            const check: () = ();
            #[allow(unused_attributes)]
            const not_null: () = ();
            #[allow(unused_attributes)]
            const default: () = ();
            #[allow(unused_attributes)]
            const foreign_key: () = ();
            #[allow(unused_attributes)]
            const db_default: () = ();
            #[allow(unused_attributes)]
            const index: () = ();
            #[allow(unused_attributes)]
            const unique_index: () = ();
            #[allow(unused_attributes)]
            const constraint: () = ();
        }
        
        #[allow(non_snake_case, dead_code)]
        struct __SqlitedAttributes;
        
        #[allow(non_snake_case, dead_code)]
        impl AttributeDefs for __SqlitedAttributes {}
    };
}

/// 将表名或SQL表达式转换为迁移字符串
#[macro_export]
#[doc(hidden)]
macro_rules! _resolve_migration_element {
    // 处理标识符 (表名)
    ($table:ident) => {
        $table::create_table_sql().to_string()
    };
    
    // 处理表达式 (已经是 SQL 或调用了 create_table_sql())
    ($expr:expr) => {
        $expr.to_string()
    };
}

/// 收集表的迁移
#[macro_export]
#[doc(hidden)]
macro_rules! _collect_table_migrations {
    // 对于表标识符，简单地调用 get_migrations 方法
    ($table:ident) => {{
        // 直接尝试调用 get_migrations 方法，错误会在编译时捕获
        match std::panic::catch_unwind(|| $table::get_migrations()) {
            Ok(migrations) => Some(migrations),
            Err(_) => None
        }
    }};
    
    // 对于其他表达式，返回None
    ($expr:expr) => {
        None::<Vec<(String, String, Option<String>)>>
    };
}