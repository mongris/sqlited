use std::ops::{Deref, DerefMut};

use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::types::{ToSqlOutput, ValueRef};
use crate::{SqliteBindableValue, SqliteTypeName, ToSql, FromSql, FromSqlError as SqlitedFromSqlError};
use rusqlite::types::FromSqlError as RusqliteFromSqlError;

// New wrapper type for DateTime<Utc>
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UtcDateTime(pub DateTime<Utc>);

// Implement Deref to make UtcDateTime behave like DateTime<Utc>
impl Deref for UtcDateTime {
    type Target = DateTime<Utc>;
    
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UtcDateTime {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Implement From conversions
impl From<DateTime<Utc>> for UtcDateTime {
    fn from(dt: DateTime<Utc>) -> Self {
        UtcDateTime(dt)
    }
}

impl From<UtcDateTime> for DateTime<Utc> {
    fn from(dt: UtcDateTime) -> Self {
        dt.0
    }
}

impl SqliteBindableValue for UtcDateTime {
    fn to_sql_value(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        // Always convert to RFC3339 format when storing
        Ok(ToSqlOutput::from(self.to_rfc3339()))
    }

    fn from_sql_value(value: ValueRef<'_>) -> Result<Self, RusqliteFromSqlError> { // Explicitly use RusqliteFromSqlError
        let text = value.as_str()?;
        
        // Try parsing as RFC3339 format first
        if let Ok(dt) = DateTime::parse_from_rfc3339(text) {
            return Ok(dt.with_timezone(&Utc).into());
        }
        
        // If that fails, try SQLite's default datetime format
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S") {
            return Ok(UtcDateTime(DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc)));
        }
        
        // As a last resort, try parsing with milliseconds
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S%.f") {
            return Ok(UtcDateTime(DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc)));
        }
        
        Err(RusqliteFromSqlError::InvalidType)
    }
}

impl ToSql for UtcDateTime {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.to_sql_value()
    }

    fn sql_type(&self) -> crate::rq::types::Type { // Assuming rq is an alias for rusqlite
        crate::rq::types::Type::Text
    }
}

// Implement FromSql directly
impl FromSql for UtcDateTime { // This is crate::FromSql
    fn from_sql(value: ValueRef<'_>) -> std::result::Result<Self, SqlitedFromSqlError> { // Expects crate::FromSqlError
        Self::from_sql_value(value).map_err(Into::into)
    }
}

impl SqliteTypeName for UtcDateTime {
    fn sql_type_name() -> &'static str {
        "TEXT"
    }
}