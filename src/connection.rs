use crate::pool::{ConnectionPool, PooledSqliteConnection};
use crate::savepoint::Savepoint;
use rusqlite::{Result, Params};
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
    pub fn execute<P: Params>(&self, query: &str, params: P) -> Result<usize> {
        self.inner.execute(query, params)
    }

    /// Execute a raw SQL query and return the rows as a statement
    pub fn query<F, T, P: Params>(&self, query: &str, params: P, map_fn: F) -> Result<Vec<T>>
    where
        F: FnMut(&rusqlite::Row) -> rusqlite::Result<T>,
    {
        let mut stmt = self.inner.prepare(query)?;
        let rows = stmt.query_map(params, map_fn)?;
        rows.collect()
    }

    /// Begin a new transaction
    pub fn begin_transaction(&mut self) -> Result<rusqlite::Transaction> {
        self.inner.transaction()
    }

    /// Create a new savepoint with the given name
    pub fn savepoint(&self, name: impl Into<String>) -> Result<Savepoint> {
        Savepoint::new(&self.inner, name)
    }

    /// Create a new savepoint with a unique name
    pub fn savepoint_unique(&self) -> Result<Savepoint> {
        Savepoint::new_unique(&self.inner)
    }

    /// Directly access the underlying SQLite connection
    pub fn raw_connection(&self) -> &rusqlite::Connection {
        &self.inner
    }

    pub fn raw_connection_mut(&mut self) -> &mut rusqlite::Connection {
        &mut self.inner
    }
}

/// Helper function to create a new in-memory SQLite database connection pool
pub fn new_memory_pool() -> Result<ConnectionPool, crate::pool::PoolError> {
    ConnectionPool::new_memory()
}

/// Helper function to create a new SQLite database connection pool from a file path
pub fn new_file_pool<P: AsRef<Path>>(path: P) -> Result<ConnectionPool, crate::pool::PoolError> {
    ConnectionPool::new(path)
}

/// Helper function to get a connection from a pool
pub fn get_connection(pool: &ConnectionPool) -> Result<SqliteConnection, r2d2::Error> {
    pool.get().map(SqliteConnection::new)
}