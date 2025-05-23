use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use thiserror::Error;

/// Error type for connection pool operations
#[derive(Debug, Error)]
pub enum PoolError {
    /// Failed to initialize the SQLite connection
    #[error("Failed to initialize SQLite connection: {0}")]
    InitError(#[from] rusqlite::Error),

    /// Failed to build the r2d2 pool
    #[error("Failed to build connection pool: {0}")]
    PoolBuildError(#[from] r2d2::Error),
}

/// A connection pool for SQLite connections
#[derive(Clone)]
pub struct ConnectionPool {
    inner: Pool<SqliteConnectionManager>,
}

/// A pooled SQLite connection
pub struct PooledSqliteConnection {
    conn: PooledConnection<SqliteConnectionManager>,
}

impl ConnectionPool {
    /// Create a new in-memory SQLite connection pool
    pub fn new_memory() -> Result<Self, PoolError> {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::builder().build(manager)?;
        Ok(Self { inner: pool })
    }

    /// Create a new SQLite connection pool from a file path
    pub fn new<P: AsRef<Path>>(path: P, initialize_pragma: String) -> Result<Self, PoolError> {
        let manager = SqliteConnectionManager::file(path).with_init(move |c| c.execute_batch(&initialize_pragma));
        let pool = Pool::builder().build(manager)?;
        Ok(Self { inner: pool })
    }

    /// Get a connection from the pool
    pub fn get(&self) -> Result<PooledSqliteConnection, r2d2::Error> {
        self.inner.get().map(|conn| PooledSqliteConnection { conn })
    }
}

impl Deref for PooledSqliteConnection {
    type Target = rusqlite::Connection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for PooledSqliteConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}