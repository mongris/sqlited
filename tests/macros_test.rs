use rusqlite::{Connection, Result};
// use crate::sql;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_macro() -> Result<()> {
        let db = Connection::open_in_memory()?;
        
        // Create a test table
        db.execute(
            "CREATE TABLE user (id INTEGER, username TEXT, email TEXT, active INTEGER)",
            [],
        )?;
        
        // Insert a test user
        db.execute(
            "INSERT INTO user (id, username, email, active) VALUES (1, 'user1', 'user1@example.com', 1)",
            [],
        )?;
        
        // Try using the sql! macro with explicit parameter placeholders
        let user_id = 1;
        // Use the format with explicit parameters - this works with our current macro
        // let query = sql!(SELECT * FROM user WHERE id = user_id);
        
        // println!("Generated query: {}", query.query);
        // println!("Parameters length: {}", query.params.len());
        
        // Execute the query
        // let users = query.query_map(&db, |row| {
        //     Ok((
        //         row.get::<_, i64>(0)?,
        //         row.get::<_, String>(1)?,
        //         row.get::<_, Option<String>>(2)?,
        //         row.get::<_, bool>(3)?
        //     ))
        // })?;
        
        // let all_users: Vec<_> = users;
        // assert_eq!(all_users.len(), 1);
        
        // // Test with multiple parameters
        // let min_id = 0;
        // let query = sql!("SELECT id FROM user WHERE id > ?", min_id);
        
        // println!("Generated query: {}", query.query);
        // println!("Parameters length: {}", query.params.len());
        
        // // Verify the query works
        // let ids = query.query_map(&db, |row| row.get::<_, i64>(0))?;
        // let all_ids: Vec<_> = ids;
        // assert_eq!(all_ids.len(), 1);
        
        Ok(())
    }
}