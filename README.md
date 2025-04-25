# SQLited

SQLited is a set of macro tools designed to simplify SQLite database operations in Rust. It provides a concise way to handle SQLite database interactions by offering helper macros and utility functions that reduce boilerplate code and enhance type safety.

### Features

- **Table definition macros**: Quickly define table structures using the `table!` macro
- **Custom type serialization**: Support for storing custom Rust types in SQLite
  - Text serialization for simple enum types
  - Binary serialization (using bincode)
  - JSON serialization (using serde_json)
- **SQL query helpers**: Simplify parameterized queries with the `sql!` macro
- **Connection helper functions**: Basic functionality for connection pool management

### Installation

Add the following dependency to your Cargo.toml:

```toml
[dependencies]
sqlited = { git = "https://github.com/mongris/sqlited.git" }
```

### Basic Usage
Table Definition

```rust
use sqlited::table;

// Define table structure using a macro
#[table]
struct User {
    #[autoincrement]
    id: i32,
    name: String,
    email: String,
    age: i32,
}
```
Creating a Connection
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create in-memory database connection
    let pool = sqlited::connection::new_memory_pool()?;
    let conn = sqlited::connection::get_connection(&pool)?;
    
    // Create table
    conn.execute(&User::create_table_sql(), [])?;
    
    Ok(())
}
```
Data Operations
```rust
// Build a query using the sql! macro
let query = sql!(
    INSERT INTO user (name, email, age) VALUES (?, ?, ?),
    User {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 28,
    }
);

// Execute the query
query.execute(&conn)?;

// Query data
let users = conn.query(
    "SELECT * FROM user WHERE age > ?", 
    [20],
    |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            age: row.get(3)?,
        })
    }
)?;
```
### Custom Type Support
SQLited provides simple ways to store custom types in SQLite:
```rust
use serde::{Serialize, Deserialize};
use sqlited::bindable_value;

// Define an enum
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub enum Status {
    #[default]
    Active,
    Inactive,
}

// Implement serialization using a macro
bindable_value!(
    enum SerializedStatus(Status) {
        Active => "active",
        Inactive => "inactive",
    }
);

// Complex struct
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
    pub notifications: bool,
}

// Use JSON serialization
bindable_value!(json JsonSettings(Settings));
```

### Limitations and Notes
+ SQLited is not a complete ORM system; it only provides basic macros and helper tools
+ No relationship mapping, migration management, or other advanced features
+ The main goal is to reduce repetitive code when writing SQLite operations
+ Most SQL queries still need to be written manually
+ Core functionality still relies on the rusqlite library
### Development Notes
SQLited is an experimental project developed with AI assistance, exploring ways to simplify database interaction code using Rust's macro system. + Contributions and suggestions for improvement are welcome!

#### License
MIT

---
*SQLited is an AI-assisted experimental project showcasing how Rust macro systems can be used to simplify database interaction code.*