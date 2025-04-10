use rusqlite::{Connection, Result, Params};
use std::fmt;
use thiserror::Error;

/// An error that occurred while working with a savepoint
#[derive(Debug, Error)]
pub enum SavepointError {
    /// An error from SQLite
    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),
    
    /// The savepoint has already been committed or rolled back
    #[error("Savepoint already finished")]
    SavepointAlreadyFinished,
}

/// The status of a savepoint
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SavepointStatus {
    /// The savepoint is active
    Active,
    /// The savepoint has been committed
    Committed,
    /// The savepoint has been rolled back
    RolledBack,
}

/// A savepoint in a SQLite transaction
pub struct Savepoint<'a> {
    conn: &'a Connection,
    name: String,
    status: SavepointStatus,
}

impl<'a> Savepoint<'a> {
    /// Create a new savepoint with the given name
    pub fn new(conn: &'a Connection, name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        conn.execute(&format!("SAVEPOINT {}", name), [])?;
        
        Ok(Self {
            conn,
            name,
            status: SavepointStatus::Active,
        })
    }
    
    /// Create a new savepoint with a unique name
    pub fn new_unique(conn: &'a Connection) -> Result<Self> {
        let name = format!("sp_{}", uuid::Uuid::new_v4().to_string().replace('-', "_"));
        Self::new(conn, name)
    }
    
    /// Commit this savepoint
    pub fn commit(mut self) -> std::result::Result<(), SavepointError> {
        if self.status != SavepointStatus::Active {
            return Err(SavepointError::SavepointAlreadyFinished);
        }
        
        self.conn.execute(&format!("RELEASE {}", self.name), [])?;
        self.status = SavepointStatus::Committed;
        Ok(())
    }
    
    /// Roll back this savepoint
    pub fn rollback(mut self) -> std::result::Result<(), SavepointError> {
        if self.status != SavepointStatus::Active {
            return Err(SavepointError::SavepointAlreadyFinished);
        }
        
        self.conn.execute(&format!("ROLLBACK TO {}", self.name), [])?;
        self.conn.execute(&format!("RELEASE {}", self.name), [])?;
        self.status = SavepointStatus::RolledBack;
        Ok(())
    }
    
    /// Execute a SQL statement within this savepoint
    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        if self.status != SavepointStatus::Active {
            return Err(rusqlite::Error::ExecuteReturnedResults);
        }
        self.conn.execute(sql, params)
    }
    
    /// Execute a SQL query within this savepoint and return the results
    pub fn query_row<P, F, T>(&self, sql: &str, params: P, f: F) -> Result<T>
    where
        P: Params,
        F: FnOnce(&rusqlite::Row<'_>) -> Result<T>,
    {
        if self.status != SavepointStatus::Active {
            return Err(rusqlite::Error::ExecuteReturnedResults);
        }
        self.conn.query_row(sql, params, f)
    }

    /// Get the name of this savepoint
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get the status of this savepoint
    pub fn status(&self) -> SavepointStatus {
        self.status
    }
}

impl<'a> fmt::Debug for Savepoint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Savepoint")
            .field("name", &self.name)
            .field("status", &self.status)
            .finish()
    }
}

impl<'a> Drop for Savepoint<'a> {
    fn drop(&mut self) {
        if self.status == SavepointStatus::Active {
            // Try to roll back the savepoint if it hasn't been committed or rolled back
            let _ = self.conn.execute(&format!("ROLLBACK TO {}", self.name), []);
            let _ = self.conn.execute(&format!("RELEASE {}", self.name), []);
        }
    }
}