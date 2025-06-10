use rusqlite::{
    Error,
    types::{ToSqlOutput, Value, ValueRef},
};
use std::fmt;
use thiserror::Error;

// Re-export for public use
pub use r2d2;
pub use r2d2_sqlite;
pub use rusqlite;
pub use sqlited_macros::{table, sql, sql_as, sql_as_value, sql_params, sql_str, query, autoincrement, primary_key, unique, check, not_null, default, foreign_key, index, unique_index, constraint, migration};

pub extern crate rusqlite as rq;
pub extern crate bincode;
pub extern crate serde_sqlite_jsonb as jsonb;

// Export our public modules
pub mod row;
pub mod connection;
pub mod macros;
pub mod migrations;
pub mod pool;
pub mod savepoint;

pub mod types;
pub mod error;

pub use error::{Result, SqlitedError};

// #[cfg(test)]
// mod macros_test;
// #[cfg(test)]
// mod without_id_test; // 添加新的测试模块
// #[cfg(test)]
// mod sql_params_test; // 引入新的测试模块
// #[cfg(test)]
// mod custom_type_test; // 引入新的测试模块

pub mod prelude {
    pub use crate::macros::*;
    pub use crate::types::*;
    pub use crate::row::*;
    pub use crate::connection::*;
}

pub use prelude::*;

/// 用于字段验证的特性
pub trait ValidateFields {
    /// 验证字段是否存在于模型中
    fn validate_field(field_name: &str) -> bool;

    /// 获取所有有效的字段名称列表 (不包括 id)
    fn field_names() -> Vec<&'static str>;

    /// 获取全部字段名称列表
    fn all_field_names() -> Vec<&'static str>;

    /// 获取字段类型映射 - 字段名到类型的映射
    fn field_types() -> Vec<(&'static str, &'static str)>;
}

/// 为所有实现 WithoutIdTableInfo 的模型类型实现 ValidateFields 特性
impl<T: WithoutIdTableInfo> ValidateFields for T {
    fn validate_field(field_name: &str) -> bool {
        // 忽略大小写进行比较，确保字段名有效
        Self::non_id_field_names()
            .iter()
            .any(|&valid_field| valid_field.eq_ignore_ascii_case(field_name))
    }

    fn field_names() -> Vec<&'static str> {
        // 返回所有非 id 字段
        Self::non_id_field_names()
    }

    fn all_field_names() -> Vec<&'static str> {
        // 返回所有字段，包括 id
        let mut fields = vec!["id"];
        fields.extend(Self::non_id_field_names());
        fields
    }

    fn field_types() -> Vec<(&'static str, &'static str)> {
        // 这里简化实现，实际项目中可能需要从模型元数据获取
        Self::non_id_field_names()
            .into_iter()
            .map(|name| (name, "unknown"))
            .collect()
    }
}

/// A trait for types that can be converted into SQL values.
/// This is inspired by the ToSql trait in rust-postgres but adapted for SQLite.
pub trait ToSql: fmt::Debug {
    /// Converts this value into a SQLite value.
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>>;

    /// A reference to the SQL type of this value.
    fn sql_type(&self) -> rusqlite::types::Type;

    /// Determines if this value needs to be dynamically typed.
    ///
    /// If this returns true, the SQL type of the value isn't known and
    /// needs to be determined from the value itself.
    fn is_dynamic(&self) -> bool {
        false
    }
}

/// An error that can occur when converting a SQLite value to a Rust type.
#[derive(Debug, Error)]
pub enum FromSqlError {
    /// The conversion failed because the database value was NULL.
    #[error("unexpected null value")]
    UnexpectedNull,

    /// The conversion failed because the database value could not be converted into the requested type.
    #[error("invalid type conversion: {0}")]
    InvalidType(String),

    /// An error from the underlying SQLite connection occurred.
    #[error("database error: {0}")]
    SqliteError(#[from] rusqlite::Error),
}

impl From<rusqlite::types::FromSqlError> for FromSqlError {
    fn from(err: rusqlite::types::FromSqlError) -> Self {
        match err {
            rusqlite::types::FromSqlError::InvalidType =>
                FromSqlError::InvalidType("Invalid type for Timestamp conversion".to_string()),
            rusqlite::types::FromSqlError::OutOfRange(_) => 
                FromSqlError::InvalidType("Value out of range for Timestamp conversion".to_string()),
            rusqlite::types::FromSqlError::Other(err) => 
                FromSqlError::InvalidType(format!("Underlying error during Timestamp conversion: {}", err)),
            _ =>
                FromSqlError::InvalidType("Unknown error".to_string()),
        }
    }
}

/// A trait for types that can be created from SQL values.
pub trait FromSql: Sized {
    /// Attempts to convert a SQLite value into this type.
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError>;
}

// Implement ToSql for Rust primitive types

impl<'a, T> ToSql for &'a T
where
    T: ToSql + ?Sized,
{
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        (*self).to_sql()
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        (*self).sql_type()
    }

    fn is_dynamic(&self) -> bool {
        (*self).is_dynamic()
    }
}

// We implement ToSql for common Rust types below

impl ToSql for bool {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self as i64))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Integer
    }
}

impl ToSql for i8 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self as i64))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Integer
    }
}

impl ToSql for i16 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self as i64))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Integer
    }
}

impl ToSql for i32 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self as i64))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Integer
    }
}

impl ToSql for i64 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Integer
    }
}

impl ToSql for u8 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self as i64))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Integer
    }
}

impl ToSql for u16 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self as i64))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Integer
    }
}

impl ToSql for u32 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self as i64))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Integer
    }
}


impl ToSql for u64 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        if *self > i64::MAX as u64 {
            return Err(Error::ToSqlConversionFailure(Box::new(
                std::io::Error::new(std::io::ErrorKind::Other, "u32 value out of range for i64"),
            )));
        }
        Ok(ToSqlOutput::from(*self as i64))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Integer
    }
}

impl ToSql for f32 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self as f64))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Real
    }
}

impl ToSql for f64 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(*self))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Real
    }
}

impl ToSql for str {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Text
    }
}

impl ToSql for String {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Text
    }
}

impl ToSql for Vec<String> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match jsonb::to_vec(self) {
            Ok(json_string) => Ok(ToSqlOutput::from(json_string)),
            Err(e) => Err(rusqlite::Error::ToSqlConversionFailure(
                Box::new(e)
            )),
        }
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Blob
    }
}

impl ToSql for Vec<u8> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_slice()))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Blob
    }
}

impl ToSql for [u8] {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Blob
    }
}

impl<T: ToSql> ToSql for Option<T> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match *self {
            Some(ref value) => value.to_sql(),
            None => Ok(ToSqlOutput::from(&Value::Null)),
        }
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        match *self {
            Some(ref value) => value.sql_type(),
            None => rusqlite::types::Type::Null,
        }
    }

    fn is_dynamic(&self) -> bool {
        match *self {
            Some(ref value) => value.is_dynamic(),
            None => false,
        }
    }
}

// Also implement FromSql trait for these same types

impl FromSql for bool {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => Ok(i != 0),
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for i8 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => i.try_into().map_err(|_| {
                FromSqlError::InvalidType(format!("Integer value {} out of range for i8", i))
            }),
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for i16 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => i.try_into().map_err(|_| {
                FromSqlError::InvalidType(format!("Integer value {} out of range for i16", i))
            }),
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for i32 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => i.try_into().map_err(|_| {
                FromSqlError::InvalidType(format!("Integer value {} out of range for i32", i))
            }),
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for i64 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => Ok(i),
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for u8 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => {
                if i < 0 {
                    return Err(FromSqlError::InvalidType(format!(
                        "Integer value {} out of range for u8",
                        i
                    )));
                }
                i.try_into().map_err(|_| {
                    FromSqlError::InvalidType(format!("Integer value {} out of range for u8", i))
                })
            }
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for u16 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => {
                if i < 0 {
                    return Err(FromSqlError::InvalidType(format!(
                        "Integer value {} out of range for u16",
                        i
                    )));
                }
                i.try_into().map_err(|_| {
                    FromSqlError::InvalidType(format!("Integer value {} out of range for u16", i))
                })
            }
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for u32 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => {
                if i < 0 {
                    return Err(FromSqlError::InvalidType(format!(
                        "Integer value {} out of range for u32",
                        i
                    )));
                }
                i.try_into().map_err(|_| {
                    FromSqlError::InvalidType(format!("Integer value {} out of range for u32", i))
                })
            }
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for u64 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => {
                if i < 0 {
                    return Err(FromSqlError::InvalidType(format!(
                        "Integer value {} out of range for u64",
                        i
                    )));
                }
                i.try_into().map_err(|_| {
                    FromSqlError::InvalidType(format!("Integer value {} out of range for u64", i))
                })
            }
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for f32 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Real(r) => Ok(r as f32),
            ValueRef::Integer(i) => Ok(i as f32),
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected REAL, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for f64 {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Real(r) => Ok(r),
            ValueRef::Integer(i) => Ok(i as f64),
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected REAL, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for String {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Text(t) => std::str::from_utf8(t)
                .map(|s| s.to_owned())
                .map_err(|_| FromSqlError::InvalidType("Invalid UTF-8 string".to_string())),
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected TEXT, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for Vec<String> {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Blob(b) => {
                let r = jsonb::from_slice::<Self>(b)
                    .map_err(|_| FromSqlError::InvalidType("Invalid jsonb for Vec<String>".to_string()))?;
                Ok(r)
            }
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected Blob for Vec<String>, got {:?}",
                value
            ))),
        }
    }
}

impl FromSql for Vec<u8> {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Blob(b) => Ok(b.to_vec()),
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected BLOB, got {:?}",
                value
            ))),
        }
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Null => Ok(None),
            _ => T::from_sql(value).map(Some),
        }
    }
}

// 添加这个宏到 lib.rs 中以便全局使用
#[macro_export]
macro_rules! without_id {
    // 现有版本
    (<$model:ty> { $($field:ident: $value:expr),* $(,)? }) => {
        {
            let mut result = $crate::WithoutId::<$model>::new();
            $(
                result.set(stringify!($field), $value);
            )*
            result
        }
    };

    // for_insert 版本：使用 create_static_params_for_insert 方法返回静态引用
    // (<$model:ty> for_insert { $($field:ident: $value:expr),* $(,)? }) => {
    //     {
    //         let mut result = $crate::WithoutId::<$model>::new();
    //         $(
    //             result.set(stringify!($field), $value);
    //         )*
    //         result.create_static_params_for_insert::<$model>()
    //     }
    // };
}
