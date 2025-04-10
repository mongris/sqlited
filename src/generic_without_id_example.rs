use crate::{
    create_table, without_id, 
    connection::{get_connection, new_memory_pool},
    migrations::{Migration, Migrator}
};
use std::error::Error;

// 定义一个带自增 ID 的用户表
create_table!(User {
    #[autoincrement]
    id: i64,
    username: String,
    email: Option<String>,
    active: bool,
});

// 定义一个带自增 ID 的文章表
create_table!(Article {
    #[autoincrement]
    id: i64,
    title: String,
    content: String,
    user_id: i64,
    published: bool,
});

pub fn run_example() -> Result<(), Box<dyn Error>> {
    println!("SQLite 通用 WithoutId<T> 示例");
    
    // 创建内存数据库
    let pool = new_memory_pool()?;
    let mut conn = get_connection(&pool)?;
    
    // 创建迁移器并添加迁移
    let mut migrator = Migrator::new();
    
    // 添加迁移以创建表
    migrator.add_migration(Migration::new(
        1,
        "create_users_table",
        User::create_table_sql(),
        Some("DROP TABLE IF EXISTS user"),
    ))?;
    
    migrator.add_migration(Migration::new(
        2,
        "create_articles_table",
        Article::create_table_sql(),
        Some("DROP TABLE IF EXISTS article"),
    ))?;
    
    // 执行迁移
    let applied = migrator.migrate(conn.raw_connection_mut())?;
    println!("应用的迁移: {:?}", applied);
    
    // 使用 without_id! 宏创建 WithoutId<User>
    let user_data = without_id!(<User> {
        username: "john_doe".to_string(),
        email: Some("john@example.com".to_string()),
        active: true,
    });
    
    // 插入用户（使用自增 ID）
    let result = conn.execute(
        &User::insert_without_id(),
        &user_data.to_params()
    )?;
    
    // 获取插入用户的自增 ID
    let user_id: i64 = conn.raw_connection().last_insert_rowid();
    println!("插入结果: {} 行受影响, ID: {}", result, user_id);
    
    // 创建文章的 WithoutId<Article>
    let article_data = without_id!(<Article> {
        title: "使用通用 WithoutId<T> 结构体".to_string(),
        content: "这篇文章演示了如何使用通用的 WithoutId<T> 结构体...".to_string(),
        user_id: user_id,
        published: true,
    });
    
    // 插入文章
    conn.execute(
        &Article::insert_without_id(),
        &article_data.to_params()
    )?;
    
    let article_id: i64 = conn.raw_connection().last_insert_rowid();
    println!("文章 ID: {}", article_id);
    
    // 查询所有文章
    let mut stmt = conn.raw_connection().prepare(&Article::query())?;
    let articles = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(3)?,
            row.get::<_, bool>(4)?
        ))
    })?;
    
    println!("\n所有文章:");
    for article_result in articles {
        let (id, title, author_id, published) = article_result?;
        let status = if published { "已发布" } else { "草稿" };
        println!("  ID: {}, 标题: {}, 作者ID: {}, 状态: {}", id, title, author_id, status);
    }
    
    println!("\n通用 WithoutId<T> 实现成功!");
    Ok(())
}