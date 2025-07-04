use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlitedError {
    #[error("SQLite error: {0}")]
    Rusqlite(#[from] rusqlite::Error),

    #[error("Error: {0}")]
    Error(#[from] anyhow::Error),

    // Use r2d2::Error directly as the source for pool errors encountered *after* pool creation
    #[error("Connection pool operation error: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("Connection pool creation error: {0}")]
    AsyncJoinError(String), // Add this variant for async join errors

    
    #[error("Parameter to SQL conversion error: {0}")]
    ToSqlConversionError(Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("SQL to Rust conversion error: {0}")]
    FromSqlConversionError(Box<dyn std::error::Error + Send + Sync + 'static>),

    // You could potentially still include your original PoolError if needed for creation errors,
    // but mapping r2d2::Error directly is often sufficient for get() errors.
    // #[error("Pool creation error: {0}")]
    // PoolCreation(#[from] crate::pool::PoolError),

    // Add other specific errors if necessary
}

// Define a Result type alias for convenience
pub type Result<T> = anyhow::Result<T, SqlitedError>;