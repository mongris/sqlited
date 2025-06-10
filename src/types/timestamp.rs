use std::ops::{Deref, DerefMut};

use chrono::{DateTime, Utc};
use rusqlite::types::{ToSqlOutput, ValueRef};
use crate::{
    SqliteBindableValue, 
    SqliteTypeName, 
    ToSql,                  // Use crate::ToSql
    FromSql,                // Use crate::FromSql
    FromSqlError as SqlitedFromSqlError // Use crate::FromSqlError
};
use rusqlite::types::FromSqlError as RusqliteFromSqlError; // For SqliteBindableValue

// New wrapper type for DateTime<Utc>
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Timestamp(pub DateTime<Utc>);

// Implement Deref to make Timestamp behave like DateTime<Utc>
impl Deref for Timestamp {
    type Target = DateTime<Utc>;
    
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Timestamp {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Implement From conversions
impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Timestamp(dt)
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(dt: Timestamp) -> Self {
        dt.0
    }
}

impl SqliteBindableValue for Timestamp {
    fn to_sql_value(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        // Always convert to unix timestamp format when storing
        Ok(ToSqlOutput::from(self.timestamp()))
    }

    fn from_sql_value(value: ValueRef<'_>) -> Result<Self, RusqliteFromSqlError> { // Returns RusqliteFromSqlError
        let timestamp = match value.as_i64() {
            Ok(ts) => ts,
            Err(e) => {
                log::error!("Failed to convert value to i64: {:?}", e);
                return Err(RusqliteFromSqlError::InvalidType);
            }
        };
        
        // 使用秒级 timestamp
        if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
            return Ok(dt.into());
        }
        
        // 尝试毫秒级 timestamp
        // Note: DateTime::from_timestamp_millis expects i64, which `timestamp` already is.
        if let Some(dt) = DateTime::from_timestamp_millis(timestamp) {
            return Ok(dt.into());
        }
        
        Err(RusqliteFromSqlError::InvalidType)
    }
}

impl ToSql for Timestamp { // Implement crate::ToSql
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.to_sql_value()
    }

    fn sql_type(&self) -> crate::rq::types::Type {
        crate::rq::types::Type::Integer // Timestamps are stored as INTEGER
    }
}

// Implement crate::FromSql
impl FromSql for Timestamp { // Implement crate::FromSql
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, SqlitedFromSqlError> { // Expects crate::FromSqlError
        Self::from_sql_value(value).map_err(Into::into)
    }
}

impl SqliteTypeName for Timestamp {
    fn sql_type_name() -> &'static str {
        "INTEGER"
    }

    fn is_integer_type() -> bool {
        true
    }
}