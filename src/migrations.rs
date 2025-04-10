use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// Error type for migration operations
#[derive(Debug, Error)]
pub enum MigrationError {
    /// An error from SQLite
    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),
    
    /// A migration with the given version already exists
    #[error("Migration version {0} already exists")]
    VersionAlreadyExists(i64),
    
    /// A migration failed to apply
    #[error("Failed to apply migration {0}: {1}")]
    MigrationFailed(i64, String),
}

/// A migration to be applied to a database
pub struct Migration {
    /// The version of this migration (should be unique and sequential)
    pub version: i64,
    /// The name of this migration
    pub name: String,
    /// SQL to run when applying this migration
    pub up: String,
    /// SQL to run when rolling back this migration (optional)
    pub down: Option<String>,
}

impl Migration {
    /// Create a new migration
    pub fn new(version: i64, name: impl Into<String>, up: impl Into<String>, down: Option<impl Into<String>>) -> Self {
        Self {
            version,
            name: name.into(),
            up: up.into(),
            down: down.map(|d| d.into()),
        }
    }
}

impl fmt::Debug for Migration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Migration")
            .field("version", &self.version)
            .field("name", &self.name)
            .finish()
    }
}

/// A migrator to apply migrations to a database
pub struct Migrator {
    migrations: HashMap<i64, Migration>,
}

impl Migrator {
    /// Create a new migrator
    pub fn new() -> Self {
        Self {
            migrations: HashMap::new(),
        }
    }
    
    /// Add a migration to this migrator
    pub fn add_migration(&mut self, migration: Migration) -> Result<&mut Self, MigrationError> {
        if self.migrations.contains_key(&migration.version) {
            return Err(MigrationError::VersionAlreadyExists(migration.version));
        }
        
        self.migrations.insert(migration.version, migration);
        Ok(self)
    }
    
    /// Create the migrations table if it doesn't exist
    fn ensure_migrations_table(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS _migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        Ok(())
    }
    
    /// Get all applied migrations
    fn get_applied_migrations(&self, conn: &Connection) -> Result<Vec<i64>> {
        let mut stmt = conn.prepare("SELECT version FROM _migrations ORDER BY version ASC")?;
        let versions = stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<i64>>>()?;
        
        Ok(versions)
    }
    
    /// Apply all unapplied migrations
    pub fn migrate(&self, conn: &mut Connection) -> Result<Vec<i64>, MigrationError> {
        self.ensure_migrations_table(conn)?;
        let applied = self.get_applied_migrations(conn)?;
        
        let mut to_apply: Vec<&Migration> = self.migrations.values()
            .filter(|m| !applied.contains(&m.version))
            .collect();
        
        to_apply.sort_by_key(|m| m.version);
        
        let mut applied_versions = Vec::new();
        
        for migration in to_apply {
            println!("Applying migration {}: {}", migration.version, migration.name);
            
            let tx = conn.transaction()?;
            
            // Apply the migration
            match tx.execute_batch(&migration.up) {
                Ok(_) => {
                    // Record the migration in the migrations table
                    let now = SystemTime::now().duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as i64;
                    
                    tx.execute(
                        "INSERT INTO _migrations (version, name, applied_at) VALUES (?, ?, ?)",
                        params![&migration.version, &migration.name, &now],
                    )?;
                    
                    tx.commit()?;
                    applied_versions.push(migration.version);
                },
                Err(e) => {
                    tx.rollback()?;
                    return Err(MigrationError::MigrationFailed(
                        migration.version,
                        format!("{}", e),
                    ));
                }
            }
        }
        
        Ok(applied_versions)
    }
    
    /// Roll back the last applied migration
    pub fn rollback(&self, conn: &mut Connection) -> Result<Option<i64>, MigrationError> {
        self.ensure_migrations_table(conn)?;
        let applied = self.get_applied_migrations(conn)?;
        
        if let Some(&last_version) = applied.last() {
            if let Some(migration) = self.migrations.get(&last_version) {
                if let Some(down) = &migration.down {
                    println!("Rolling back migration {}: {}", migration.version, migration.name);
                    
                    let tx = conn.transaction()?;
                    
                    // Roll back the migration
                    match tx.execute_batch(down) {
                        Ok(_) => {
                            // Remove the migration from the migrations table
                            tx.execute(
                                "DELETE FROM _migrations WHERE version = ?",
                                &[&migration.version],
                            )?;
                            
                            tx.commit()?;
                            return Ok(Some(migration.version));
                        },
                        Err(e) => {
                            tx.rollback()?;
                            return Err(MigrationError::MigrationFailed(
                                migration.version,
                                format!("{}", e),
                            ));
                        }
                    }
                } else {
                    return Err(MigrationError::MigrationFailed(
                        migration.version,
                        "No down migration provided".to_string(),
                    ));
                }
            }
        }
        
        Ok(None)
    }
}

impl Default for Migrator {
    fn default() -> Self {
        Self::new()
    }
}