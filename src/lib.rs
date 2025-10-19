use rusqlite::{
    Error,
    types::{ToSqlOutput, Value, ValueRef},
};
use std::{fmt, str::FromStr};
use thiserror::Error;

// Re-export for public use
pub use r2d2;
pub use r2d2_sqlite;
pub use rusqlite;
pub use sqlited_macros::{table, sql, sql_as, sql_as_value, sql_params, sql_str, query, autoincrement, primary_key, unique, check, not_null, default, foreign_key, index, unique_index, constraint, migration};

pub extern crate rusqlite as rq;
pub extern crate bincode;
pub extern crate borsh as borsh;
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
    pub use crate::{ToSql, FromSql, FromSqlError};
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

impl ToSql for usize {
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
        if *self <= i64::MAX as u64 {
            // 如果在 i64 范围内，直接存储
            Ok(ToSqlOutput::from(*self as i64))
        } else {
            // 如果超出 i64 范围，计算偏移量并存储为负数
            // 将值映射到负数范围：[i64::MIN, -1]
            let offset = *self - (i64::MAX as u64 + 1);
            // 确保 offset 在有效范围内
            if offset <= i64::MAX as u64 {
                Ok(ToSqlOutput::from(i64::MIN + offset as i64))
            } else {
                // 这种情况不应该发生，因为 u64 的最大值减去 i64::MAX+1 应该等于 i64::MAX
                Err(rusqlite::Error::ToSqlConversionFailure(
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "u64 value too large to encode"
                    ))
                ))
            }
        }
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

// impl ToSql for Vec<&str> {
//     fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
//         match jsonb::to_vec(self) {
//             Ok(b) => Ok(ToSqlOutput::from(b)),
//             Err(e) => Err(rusqlite::Error::ToSqlConversionFailure(
//                 Box::new(e)
//             )),
//         }
//     }

//     fn sql_type(&self) -> rusqlite::types::Type {
//         rusqlite::types::Type::Blob
//     }
// }

impl<T> crate::ToSql for Vec<T>
where
    T: crate::borsh::BorshSerialize + fmt::Debug,
{
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        // self.0 is the inner Vec<T>
        match crate::borsh::to_vec(self) { // Vec<T> implements borsh::BorshSerialize
            Ok(bytes) => Ok(ToSqlOutput::from(bytes)),
            Err(e) => {
                let err_msg = format!("Failed to BorshSerialize Vec<T>: {}", e);
                Err(rusqlite::Error::ToSqlConversionFailure(
                    Box::new(
                        std::io::Error::new(std::io::ErrorKind::Other, err_msg),
                    )))
            }
        }
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Blob
    }
}

// impl ToSql for Vec<String> {
//     fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
//         match jsonb::to_vec(self) {
//             Ok(b) => Ok(ToSqlOutput::from(b)),
//             Err(e) => Err(rusqlite::Error::ToSqlConversionFailure(
//                 Box::new(e)
//             )),
//         }
//     }

//     fn sql_type(&self) -> rusqlite::types::Type {
//         rusqlite::types::Type::Blob
//     }
// }

// impl ToSql for Vec<solana_pubkey::Pubkey> {
//     fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
//         if self.is_empty() {
//             // Handle empty Vec case: store as an empty blob or NULL,
//             // depending on your preference. Empty blob is often fine.
//             return Ok(ToSqlOutput::from(Vec::<u8>::new()));
//         }

//         // Each Pubkey is 32 bytes.
//         // Pre-allocate a Vec<u8> with the total required capacity.
//         let mut bytes_vec = Vec::with_capacity(self.len() * 32);
//         for pubkey in self {
//             bytes_vec.extend_from_slice(pubkey.as_ref()); // solana_pubkey::Pubkey derefs to [u8; 32]
//                                                           // or use pubkey.to_bytes() if that's the method name
//         }
//         Ok(ToSqlOutput::from(bytes_vec))
//     }

//     fn sql_type(&self) -> rusqlite::types::Type {
//         rusqlite::types::Type::Blob
//     }
// }

// impl ToSql for Vec<u8> {
//     fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
//         Ok(ToSqlOutput::from(self.as_slice()))
//     }

//     fn sql_type(&self) -> rusqlite::types::Type {
//         rusqlite::types::Type::Blob
//     }
// }

// impl ToSql for Vec<i64> {
//     fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
//         let mut bytes_vec = Vec::with_capacity(self.len() * 8);
//         for &value in self {
//             bytes_vec.extend_from_slice(&value.to_le_bytes());
//         }
//         Ok(ToSqlOutput::from(bytes_vec))
//     }

//     fn sql_type(&self) -> rusqlite::types::Type {
//         rusqlite::types::Type::Blob
//     }
// }

impl ToSql for [u8] {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Blob
    }
}

impl ToSql for solana_pubkey::Pubkey {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Text
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

impl FromSql for usize {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => i.try_into().map_err(|_| {
                FromSqlError::InvalidType(format!("Integer value {} out of range for usize", i))
            }),
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
                if i >= 0 {
                    // 正数直接转换
                    Ok(i as u64)
                } else {
                    // 负数表示超出 i64::MAX 的 u64 值
                    // 从负数范围恢复：offset = i - i64::MIN
                    let offset = (i - i64::MIN) as u64;
                    Ok((i64::MAX as u64 + 1) + offset)
                }
            }
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected INTEGER for u64, got {:?}",
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

impl<T> crate::FromSql for Vec<T>
where
    T: crate::borsh::BorshDeserialize,
{
    fn from_sql(value: rusqlite::types::ValueRef<'_>) -> std::result::Result<Self, crate::FromSqlError> {
        match value {
            rusqlite::types::ValueRef::Blob(b) => {
                // Deserialize the whole Vec<T> from the slice
                crate::borsh::from_slice::<Vec<T>>(b)
                    .map_err(|e| {
                        let err_msg = format!("Failed to BorshDeserialize Vec<T>: {}", e);
                        crate::FromSqlError::InvalidType(err_msg) // Use your defined FromSqlError variant
                    })
            }
            _ => Err(crate::FromSqlError::InvalidType(format!(
                "Expected BLOB for Vec<T>, got {:?}",
                value.data_type()
            ))),
        }
    }
}

/// Implements ToSql for tuple (A, B) where both A and B implement BorshSerialize
impl<A, B> crate::ToSql for (A, B)
where
    A: crate::borsh::BorshSerialize + fmt::Debug,
    B: crate::borsh::BorshSerialize + fmt::Debug,
{
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match crate::borsh::to_vec(self) {
            Ok(bytes) => Ok(ToSqlOutput::from(bytes)),
            Err(e) => Err(rusqlite::Error::ToSqlConversionFailure(Box::new(e))),
        }
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Blob
    }
}

/// Implements FromSql for tuple (A, B) where both A and B implement BorshDeserialize
impl<A, B> crate::FromSql for (A, B)
where
    A: crate::borsh::BorshDeserialize,
    B: crate::borsh::BorshDeserialize,
{
    fn from_sql(value: rusqlite::types::ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        use rusqlite::types::FromSql;
        Vec::<u8>::column_result(value)
            .and_then(|bytes| {
                crate::borsh::from_slice::<(A, B)>(&bytes)
                    .map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(e)))
            })
            .map_err(Into::into)
    }
}


/// Implements ToSql for tuple (A, B, C)
impl<A, B, C> crate::ToSql for (A, B, C)
where
    A: crate::borsh::BorshSerialize + fmt::Debug,
    B: crate::borsh::BorshSerialize + fmt::Debug,
    C: crate::borsh::BorshSerialize + fmt::Debug,
{
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match crate::borsh::to_vec(self) {
            Ok(bytes) => Ok(ToSqlOutput::from(bytes)),
            Err(e) => Err(rusqlite::Error::ToSqlConversionFailure(Box::new(e))),
        }
    }

    fn sql_type(&self) -> rusqlite::types::Type {
        rusqlite::types::Type::Blob
    }
}

impl<A, B, C> crate::FromSql for (A, B, C)
where
    A: crate::borsh::BorshDeserialize,
    B: crate::borsh::BorshDeserialize,
    C: crate::borsh::BorshDeserialize,
{
    fn from_sql(value: rusqlite::types::ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        use rusqlite::types::FromSql;
        Vec::<u8>::column_result(value)
            .and_then(|bytes| {
                crate::borsh::from_slice::<(A, B, C)>(&bytes)
                    .map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(e)))
            })
            .map_err(Into::into)
    }
}

// impl FromSql for Vec<String> {
//     fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
//         match value {
//             ValueRef::Blob(b) => {
//                 let r = jsonb::from_slice::<Self>(b)
//                     .map_err(|_| FromSqlError::InvalidType("Invalid jsonb for Vec<String>".to_string()))?;
//                 Ok(r)
//             }
//             _ => Err(FromSqlError::InvalidType(format!(
//                 "Expected Blob for Vec<String>, got {:?}",
//                 value
//             ))),
//         }
//     }
// }

// impl FromSql for Vec<solana_pubkey::Pubkey> {
//     fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
//         match value {
//             ValueRef::Blob(b) => {
//                 if b.is_empty() {
//                     // If an empty Vec was stored as an empty blob
//                     return Ok(Vec::new());
//                 }
//                 // Check if the blob length is a multiple of 32
//                 if b.len() % 32 != 0 {
//                     return Err(FromSqlError::InvalidType(format!(
//                         "Invalid BLOB length for Vec<Pubkey>: expected multiple of 32, got {}",
//                         b.len()
//                     )));
//                 }

//                 let num_pubkeys = b.len() / 32;
//                 let mut pubkeys_vec = Vec::with_capacity(num_pubkeys);

//                 for chunk in b.chunks_exact(32) {
//                     // chunk is &[u8] of length 32
//                     // solana_pubkey::Pubkey can often be created directly from a [u8; 32] or &[u8] slice of length 32
//                     // Assuming Pubkey::new_from_array or similar exists and takes [u8; 32]
//                     // Or if Pubkey implements TryFrom<&[u8]>
//                     match chunk.try_into() as core::result::Result<&[u8; 32], _> {
//                         Ok(array_ref) => { // array_ref is now &[u8; 32]
//                             pubkeys_vec.push(solana_pubkey::Pubkey::new_from_array(*array_ref));
//                         }
//                         Err(_) => {
//                             // This should not happen if chunks_exact(32) is used and length is a multiple of 32
//                             return Err(FromSqlError::InvalidType(
//                                 "Internal error: chunk conversion to [u8; 32] failed".to_string(),
//                             ));
//                         }
//                     }
//                 }
//                 Ok(pubkeys_vec)
//             }
//             ValueRef::Null => {
//                 // Decide if NULL should be an empty Vec or an error
//                 Ok(Vec::new()) // Or Err(FromSqlError::UnexpectedNull)
//             }
//             _ => Err(FromSqlError::InvalidType(format!(
//                 "Expected BLOB for Vec<Pubkey>, got {:?}",
//                 value
//             ))),
//         }
//     }
// }

// impl FromSql for Vec<u8> {
//     fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
//         match value {
//             ValueRef::Blob(b) => Ok(b.to_vec()),
//             _ => Err(FromSqlError::InvalidType(format!(
//                 "Expected BLOB, got {:?}",
//                 value
//             ))),
//         }
//     }
// }

// impl FromSql for Vec<i64> {
//     fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
//         match value {
//             ValueRef::Blob(b) => {
//                 let mut vec = Vec::new();
//                 if b.is_empty() {
//                     return Ok(vec); // Empty Vec case
//                 }
//                 if b.len() % 8 != 0 {
//                     return Err(FromSqlError::InvalidType(format!(
//                         "Invalid BLOB length for Vec<i64>: expected multiple of 8, got {}",
//                         b.len()
//                     )));
//                 }
//                 for chunk in b.chunks_exact(8) {
//                     let mut array = [0u8; 8];
//                     array.copy_from_slice(chunk);
//                     vec.push(i64::from_le_bytes(array));
//                 }
//                 Ok(vec)
//             }
//             _ => Err(FromSqlError::InvalidType(format!(
//                 "Expected BLOB for Vec<i64>, got {:?}",
//                 value
//             ))),
//         }
//     }
// }

impl FromSql for solana_pubkey::Pubkey {
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        match value {
            ValueRef::Text(t) => {
                let s = std::str::from_utf8(t)
                    .map_err(|_| FromSqlError::InvalidType("Invalid UTF-8 string for Pubkey".to_string()))?;
                solana_pubkey::Pubkey::from_str(s).map_err(|_| FromSqlError::InvalidType("Invalid Pubkey format".to_string()))
            }
            _ => Err(FromSqlError::InvalidType(format!(
                "Expected TEXT for Pubkey, got {:?}",
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
