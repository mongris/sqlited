use crate::FromSql;
use std::any::TypeId;

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
    /// If `T` is `Vec<u8>`, this method will use `rusqlite`'s native BLOB to `Vec<u8>`
    /// conversion, bypassing `sqlited::FromSql for Vec<u8>`.
    /// For other types, it uses `sqlited::FromSql`.
    ///
    /// Returns `rusqlite::Result<T>` to be compatible with `rusqlite`'s mapping functions.
    /// Errors from `sqlited::FromSql` are converted to `rusqlite::Error::FromSqlConversionFailure`.
    fn get_by_index<T: FromSql + 'static>(&self, index: usize) -> rusqlite::Result<T> {
        let value_ref = self.inner_row.get_ref(index)?;

        if TypeId::of::<T>() == TypeId::of::<Vec<u8>>() {
            // Specific handling for T = Vec<u8>: use rusqlite's direct BLOB to Vec<u8> conversion.
            // This ensures Vec<u8> is treated as raw bytes.
            let result_vec_u8: Result<Vec<u8>, rusqlite::types::FromSqlError> =
                value_ref.as_blob().map(|b| b.to_vec());

            match result_vec_u8 {
                Ok(vec_val) => {
                    // We know T is Vec<u8> due to TypeId check.
                    // To safely cast Vec<u8> to T, we use Box<dyn Any>.
                    let any_val = Box::new(vec_val) as Box<dyn std::any::Any>;
                    match any_val.downcast::<T>() {
                        Ok(t_val) => Ok(*t_val),
                        Err(_) => {
                            // This should be logically impossible if TypeId matched.
                            unreachable!("TypeId matched Vec<u8> but downcast to T failed.");
                        }
                    }
                }
                Err(rusqlite_from_sql_err) => {
                    // Convert rusqlite::types::FromSqlError to rusqlite::Error
                    Err(rusqlite::Error::FromSqlConversionFailure(
                        index,
                        value_ref.data_type(),
                        Box::new(rusqlite_from_sql_err), // rusqlite::types::FromSqlError implements std::error::Error
                    ))
                }
            }
        } else {
            // Generic path: use the custom sqlited::FromSql trait for other types T
            T::from_sql(value_ref).map_err(|custom_err| { // custom_err is crate::FromSqlError
                let sqlite_type = value_ref.data_type();
                rusqlite::Error::FromSqlConversionFailure(index, sqlite_type, Box::new(custom_err))
            })
        }
    }

    /// Retrieves the value of a column by its name, converting it
    /// using the `sqlited::FromSql` trait.
    ///
    /// Returns `rusqlite::Result<T>`.
    fn get_by_name<T: FromSql + 'static>(&self, column_name: &str) -> rusqlite::Result<T> { // Added 'static bound
        // First, get the index from the name using rusqlite's internal mechanism.
        // rusqlite::Row::get itself does this: self.index(column_name).and_then(|i| self.get(i))
        // We need the index to pass to get_by_index for error reporting consistency.
        let column_idx = self.inner_row.get(column_name)?; // Get rusqlite::Error if name not found
        self.get_by_index(column_idx)
    }

    /// Retrieves the value of a column, converting it using the `sqlited::FromSql` trait.
    /// The column can be specified by its numerical index (`usize`) or by its name (`&str`).
    pub fn get<I, T>(&self, idx: I) -> rusqlite::Result<T>
    where
        I: SqlitedRowIndex,
        T: FromSql + 'static,
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
    fn get_from_sqlite_row<T: FromSql + 'static>(self, row: &Row<'_>) -> rusqlite::Result<T>;
}

impl SqlitedRowIndex for usize {
    fn get_from_sqlite_row<T: FromSql + 'static>(self, row: &Row<'_>) -> rusqlite::Result<T> {
        row.get_by_index(self)
    }
}

impl<'a> SqlitedRowIndex for &'a str {
    fn get_from_sqlite_row<T: FromSql + 'static>(self, row: &Row<'_>) -> rusqlite::Result<T> {
        row.get_by_name(self)
    }
}