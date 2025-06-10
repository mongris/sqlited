use crate::FromSql;
use rusqlite::types::ValueRef;

/// A wrapper around `rusqlite::Row` that provides methods for deserializing
/// column values using the custom `sqlited::FromSql` trait.
pub struct Row<'stmt_row> {
    inner_row: &'stmt_row rusqlite::Row<'stmt_row>,
}

impl<'stmt_row> Row<'stmt_row> {
    /// Creates a new `sqlited::Row` wrapper from a `rusqlite::Row`.
    pub fn new(rusqlite_row: &'stmt_row rusqlite::Row<'stmt_row>) -> Self {
        Self { inner_row: rusqlite_row }
    }

    /// Retrieves the value of a column by its numerical index, converting it
    /// using the `sqlited::FromSql` trait.
    ///
    /// Returns `rusqlite::Result<T>` to be compatible with `rusqlite`'s mapping functions.
    /// Errors from `sqlited::FromSql` are converted to `rusqlite::Error::FromSqlConversionFailure`.
    fn get_by_index<T: FromSql>(&self, index: usize) -> rusqlite::Result<T> {
        let value_ref = self.inner_row.get_ref(index)?; // This can return rusqlite::Error
        T::from_sql(value_ref).map_err(|custom_err| { // custom_err is crate::FromSqlError
            let sqlite_type = value_ref.data_type();
            rusqlite::Error::FromSqlConversionFailure(index, sqlite_type, Box::new(custom_err))
        })
    }

    /// Retrieves the value of a column by its name, converting it
    /// using the `sqlited::FromSql` trait.
    ///
    /// Returns `rusqlite::Result<T>`.
    fn get_by_name<T: FromSql>(&self, column_name: &str) -> rusqlite::Result<T> {
        let index = self.inner_row.get(column_name)?; // Get rusqlite::Error if name not found
        self.get_by_index(index)
    }

    /// Retrieves the value of a column, converting it using the `sqlited::FromSql` trait.
    /// The column can be specified by its numerical index (`usize`) or by its name (`&str`).
    pub fn get<I, T>(&self, idx: I) -> rusqlite::Result<T>
    where
        I: SqlitedRowIndex,
        T: FromSql,
    {
        idx.get_from_sqlite_row(self)
    }

    /// Provides access to the underlying `rusqlite::Row` if needed for methods
    /// not covered by this wrapper.
    pub fn as_rusqlite_row(&self) -> &rusqlite::Row<'stmt_row> {
        self.inner_row
    }
}

/// A helper trait to allow `Row::get` to accept either `usize` or `&str` as an index.
pub trait SqlitedRowIndex: Sized {
    /// Retrieves a value of type `T` from the given `sqlited::Row`.
    fn get_from_sqlite_row<T: FromSql>(self, row: &Row<'_>) -> rusqlite::Result<T>;
}

impl SqlitedRowIndex for usize {
    fn get_from_sqlite_row<T: FromSql>(self, row: &Row<'_>) -> rusqlite::Result<T> {
        row.get_by_index(self)
    }
}

impl<'a> SqlitedRowIndex for &'a str {
    fn get_from_sqlite_row<T: FromSql>(self, row: &Row<'_>) -> rusqlite::Result<T> {
        row.get_by_name(self)
    }
}