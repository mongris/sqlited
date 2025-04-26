use crate::pool::{ConnectionPool, PoolError, PooledSqliteConnection};
use crate::savepoint::Savepoint;
use crate::error::{Result, SqlitedError};
use rusqlite::Params;
use std::path::Path;

/// A SQLite connection wrapper
pub struct SqliteConnection {
    inner: PooledSqliteConnection,
}

impl SqliteConnection {
    /// Create a new SQLite connection from a pool
    pub fn new(conn: PooledSqliteConnection) -> Self {
        Self { inner: conn }
    }

    /// Execute a raw SQL query and return the number of rows affected
    // Update the return type to use the custom Result
    pub fn execute<P: Params>(&self, query: &str, params: P) -> Result<usize> {
        self.inner.execute(query, params).map_err(Into::into)
    }

    /// Execute a raw SQL query and return the rows as a statement
    // Update the return type to use the custom Result
    pub fn query<F, T, P: Params>(&self, query: &str, params: P, map_fn: F) -> Result<Vec<T>>
    where
        F: FnMut(&rusqlite::Row) -> rusqlite::Result<T>, // Keep inner map_fn result as rusqlite::Result
    {
        // prepare returns rusqlite::Result, ? converts error via From
        let mut stmt = self.inner.prepare(query)?;
        // query_map returns rusqlite::Result, ? converts error via From
        let rows = stmt.query_map(params, map_fn)?;
        // Explicitly specify the collection type for collect using turbofish
        rows.collect::<rusqlite::Result<Vec<T>>>().map_err(SqlitedError::from)
    }

    // Update the return type to use the custom Result
    pub fn query_row<P, F, T>(&self, sql: &str, params: P, f: F) -> Result<T>
    where
        P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        // prepare returns rusqlite::Result, ? converts error via From
        let mut stmt = self.raw_connection().prepare(sql)?;
        // query_row returns rusqlite::Result, map_err converts error via From
        stmt.query_row(params, f).map_err(SqlitedError::from)
    }
    
    /// Begin a new transaction
    // Update the return type to use the custom Result
    pub fn begin_transaction(&mut self) -> Result<rusqlite::Transaction> {
        // transaction returns rusqlite::Result, map_err converts error via From
        self.inner.transaction().map_err(SqlitedError::from)
    }

    /// Create a new savepoint with the given name
    // Update the return type to use the custom Result
    pub fn savepoint(&self, name: impl Into<String>) -> Result<Savepoint> {
        // Assuming Savepoint::new returns rusqlite::Result or your custom Result
        Savepoint::new(&self.inner, name)
            .map_err(SqlitedError::from) // Ensure conversion if Savepoint::new returns rusqlite::Result
    }

    /// Create a new savepoint with a unique name
    // Update the return type to use the custom Result
    pub fn savepoint_unique(&self) -> Result<Savepoint> {
        // Assuming Savepoint::new_unique returns rusqlite::Result or your custom Result
        Savepoint::new_unique(&self.inner)
            .map_err(SqlitedError::from) // Ensure conversion if Savepoint::new_unique returns rusqlite::Result
    }

    /// Directly access the underlying SQLite connection
    pub fn raw_connection(&self) -> &rusqlite::Connection {
        &self.inner
    }

    pub fn raw_connection_mut(&mut self) -> &mut rusqlite::Connection {
        &mut self.inner
    }

    /// Get the last inserted row ID. No error handling needed here.
    pub fn last_insert_rowid(&self) -> i64 {
        self.inner.last_insert_rowid()
    }
}

/// Helper function to create a new in-memory SQLite database connection pool
// Update the return type to use the custom Result for PoolError
pub fn new_memory_pool() -> Result<ConnectionPool> {
    ConnectionPool::new_memory().map_err(SqlitedError::from)
}

/// Helper function to create a new SQLite database connection pool from a file path
// Update the return type to use the custom Result for PoolError
pub fn new_file_pool<P: AsRef<Path>>(path: P) -> Result<ConnectionPool> {
    ConnectionPool::new(path).map_err(SqlitedError::from)
}

/// Helper function to get a connection from a pool
// Update the return type to use the custom Result
pub fn get_connection(pool: &ConnectionPool) -> Result<SqliteConnection> {
    // pool.get() returns Result<_, r2d2::Error>
    // map_err converts the r2d2::Error into SqlitedError::Pool via From
    pool.get()
        .map_err(SqlitedError::from)
        .map(SqliteConnection::new)
}

impl From<PoolError> for SqlitedError {
    fn from(err: PoolError) -> Self {
        match err {
            PoolError::InitError(e) => SqlitedError::Rusqlite(e),
            PoolError::PoolBuildError(e) => SqlitedError::Pool(e),
        }
    }
}