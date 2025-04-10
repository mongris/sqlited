use crate::{
    connection::{get_connection, new_memory_pool},
    migrations::{Migration, Migrator},
};
use rusqlite::params;

// Example demonstrating SQLite usage
pub fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create a memory database connection pool
    let pool = new_memory_pool()?;
    let mut conn = get_connection(&pool)?;
    
    // Set up migrations
    let mut migrator = Migrator::new();
    
    // Add a migration to create a users table
    migrator.add_migration(Migration::new(
        1,
        "create_users_table",
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT UNIQUE,
            created_at INTEGER NOT NULL
        )",
        Some("DROP TABLE users"),
    ))?;
    
    // Add a migration to create a posts table
    migrator.add_migration(Migration::new(
        2,
        "create_posts_table",
        "CREATE TABLE posts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        Some("DROP TABLE posts"),
    ))?;
    
    // Run the migrations
    let applied = migrator.migrate(conn.raw_connection_mut())?;
    println!("Applied migrations: {:?}", applied);
    
    // Insert a user
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    conn.execute(
        "INSERT INTO users (username, email, created_at) VALUES (?, ?, ?)",
        params![&"user1", &"user1@example.com", &now],
    )?;
    
    // Use a savepoint to handle post creation
    let sp = conn.savepoint("create_post")?;
    
    conn.execute(
        "INSERT INTO posts (title, content, user_id, created_at) VALUES (?, ?, ?, ?)",
        params![
            &"First Post", 
            &"This is the content of the first post", 
            &1, 
            &now
        ],
    )?;
    
    // Commit the savepoint
    sp.commit()?;
    
    // Query the data
    let mut stmt = conn.raw_connection().prepare("SELECT id, username FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;
    
    println!("Users:");
    for user in user_iter {
        let (id, username) = user?;
        println!("  {} - {}", id, username);
    }
    
    let mut stmt = conn.raw_connection().prepare("SELECT id, title FROM posts")?;
    let post_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;
    
    println!("Posts:");
    for post in post_iter {
        let (id, title) = post?;
        println!("  {} - {}", id, title);
    }
    
    // Demonstrate a rollback
    let sp = conn.savepoint("create_invalid_post")?;
    
    // This will fail because user_id 999 doesn't exist
    conn.execute(
        "INSERT INTO posts (title, content, user_id, created_at) VALUES (?, ?, ?, ?)",
        params![
            &"Invalid Post", 
            &"This post will be rolled back", 
            &999, 
            &now
        ],
    ).expect_err("This insert should fail");
    
    // The savepoint will be rolled back automatically when dropped
    
    // Verify that the invalid post wasn't inserted
    let count: i64 = conn.raw_connection().query_row(
        "SELECT COUNT(*) FROM posts WHERE title = ?",
        &[&"Invalid Post"],
        |row| row.get(0),
    )?;
    
    println!("Invalid posts count: {}", count); // Should be 0
    
    Ok(())
}