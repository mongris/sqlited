use crate::pool::{ConnectionPool, PoolError, PooledSqliteConnection};
use crate::savepoint::Savepoint;
use crate::error::{Result, SqlitedError};
use crate::row::Row as SqlitedRow;
use crate::StaticParamsHolder;
use rq::Params;
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
        self.inner.execute(query, params).map_err(SqlitedError::from)
    }

    pub fn execute2(&self, query: &str, params: StaticParamsHolder) -> Result<usize> {
        // Use the StaticParamsHolder to execute the query
        self.execute(query, &*params)
    }

    /// Execute a raw SQL query and return the rows as a statement
    // Update the return type to use the custom Result
    pub fn query<F, T, P: Params>(&self, query_str: &str, params: P, mut map_fn: F) -> Result<Vec<T>>
    where
        F: FnMut(&SqlitedRow) -> rq::Result<T>, // map_fn now takes &SqlitedRow
    {
        let mut stmt = self.inner.prepare(query_str).map_err(SqlitedError::from)?;
        let iter = stmt.query_map(params, |rusqlite_row| {
            // Wrap rusqlite::Row with our SqlitedRow
            let sl_row = SqlitedRow::new(rusqlite_row);
            // Call the user's mapping function
            map_fn(&sl_row)
        }).map_err(SqlitedError::from)?;
        
        iter.collect::<rq::Result<Vec<T>>>().map_err(SqlitedError::from)
    }

    pub fn query2<F, T>(&self, query_str: &str, params: StaticParamsHolder, mut map_fn: F) -> Result<Vec<T>>
    where
        F: FnMut(&SqlitedRow) -> rq::Result<T>, // map_fn now takes &SqlitedRow
    {
        self.query(query_str, &*params, map_fn)
    }


    // Update the return type to use the custom Result
    pub fn query_row<P, F, T>(&self, sql: &str, params: P, map_fn: F) -> Result<T>
    where
        P: rq::Params,
        F: FnOnce(&SqlitedRow<'_>) -> rq::Result<T>, // map_fn now takes &SqlitedRow
    {
        // query_row on rusqlite::Connection takes a closure that receives &rusqlite::Row
        // and returns rusqlite::Result<T>.
        self.raw_connection().query_row(sql, params, |rusqlite_row| {
            let sl_row = SqlitedRow::new(rusqlite_row);
            map_fn(&sl_row)
        }).map_err(SqlitedError::from)
    }

    pub fn query_row2<F, T>(&self, sql: &str, params: StaticParamsHolder, map_fn: F) -> Result<T>
    where
        F: FnOnce(&SqlitedRow<'_>) -> rq::Result<T>, // map_fn now takes &SqlitedRow
    {
        self.query_row(sql, &*params, map_fn)
    }
    
    /// Begin a new transaction
    // Update the return type to use the custom Result
    pub fn begin_transaction(&mut self) -> Result<rq::Transaction> {
        // transaction returns rq::Result, map_err converts error via From
        self.inner.transaction().map_err(SqlitedError::from)
    }

    /// Create a new savepoint with the given name
    // Update the return type to use the custom Result
    pub fn savepoint(&self, name: impl Into<String>) -> Result<Savepoint> {
        // Assuming Savepoint::new returns rq::Result or your custom Result
        Savepoint::new(&self.inner, name)
            .map_err(SqlitedError::from) // Ensure conversion if Savepoint::new returns rq::Result
    }

    /// Create a new savepoint with a unique name
    // Update the return type to use the custom Result
    pub fn savepoint_unique(&self) -> Result<Savepoint> {
        // Assuming Savepoint::new_unique returns rq::Result or your custom Result
        Savepoint::new_unique(&self.inner)
            .map_err(SqlitedError::from) // Ensure conversion if Savepoint::new_unique returns rq::Result
    }

    /// Directly access the underlying SQLite connection
    pub fn raw_connection(&self) -> &rq::Connection {
        &self.inner
    }

    pub fn raw_connection_mut(&mut self) -> &mut rq::Connection {
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
pub fn new_file_pool<P: AsRef<Path>>(path: P, initialize_pragma: &str) -> Result<ConnectionPool> {
    ConnectionPool::new(path, initialize_pragma.to_string()).map_err(SqlitedError::from)
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