#[cfg(test)]
mod tests {
    use rusqlite::params;
    use uuid::timestamp;
    use crate::{
        connection::{SqliteConnection, new_memory_pool, get_connection},
        table,
        without_id,
        sql,
        sql_str,
        sql_params,
        migrations::{Migration, Migrator},
        WithoutId,
        SqliteTypeName,
        UtcDateTime
    };

    // 定义一个用于测试的用户模型
    table!(User {
        #[autoincrement]
        id: i32,
        name: String,
        age: i32,
        email: Option<String>,
        #[default("now")]
        created_at: UtcDateTime,
        #[default("now")]
        created_at_timestamp: Timestamp,
        #[default("1")]
        active: bool,
    });
    
    // 定义一个用于测试的帖子模型
    table!(TestPost {
        #[autoincrement]
        id: i32,
        title: String,
        content: String,
        published: bool,
        user_id: i32,
    });

    // 辅助函数：创建内存数据库连接池并返回连接
    fn create_test_connection() -> SqliteConnection {
        let pool = new_memory_pool().expect("创建内存连接池失败");
        get_connection(&pool).expect("从连接池获取连接失败")
    }

    #[test]
    fn test_without_id_macro() {
        // 使用宏创建 WithoutId 实例
        let user_data = without_id!(<User> {
            name: "John Doe",
            age: 30,
            email: Some("john@example.com"),
        });

        // 验证字段值
        assert_eq!(user_data.inner.len(), 3);
        assert!(user_data.inner.contains_key("name"));
        assert!(user_data.inner.contains_key("age"));
        assert!(user_data.inner.contains_key("email"));
    }

    #[test]
    fn test_for_insert_macro() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        
        // 创建用户表
        conn.execute(User::create_table_sql().as_str(), []).unwrap();
        
        // 使用 sql_params 宏 - 使用有效的字段
        // let params = sql_params!(<User> {
        //     name: "Jane Smith".to_string(),
        //     age: 28,
        //     email: Some("jane@example.com".to_string())
        // });


        let d = sql!(
            PRAGMA journal_mode=WAL;
            PRAGMA busy_timeout=1;
            PRAGMA case_sensitive_like=TRUE;
            PRAGMA synchronous=NORMAL;
        );

        eprintln!("SQL1: {}", d.query);


        let params = sql_params!(User {
            name: "Jane Smith".to_string(),
            age: 28,
            email: Some("jane@example.com".to_string()),
        });

        let query = sql!(
            INSERT INTO user (name, age, email) VALUES (?, ?, ?),
            &params
        );
        
        // 使用参数进行插入操作
        // let query = sql!(
        //     INSERT INTO user (name, age, email) VALUES (?, ?, ?),
        //     User {
        //         name: "Jane Smith".to_string(),
        //         age: 28,
        //         email: Some("jane@example.com".to_string())
        //     }
        // );
        eprintln!("SQL2: {}, params count: {}", query.query, params.len());
        let result = query.execute(&conn);
        // let result = conn.execute(sql2, &*params);
        
        // 验证插入成功
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1); // 应该插入了一行
        
        // 验证数据被正确插入
        let row_data = &conn.query("SELECT name, age, email FROM user WHERE rowid = 1", [], 
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?, row.get::<_, Option<String>>(2)?))
        ).unwrap()[0];
        
        let (name, age, email) = row_data;
        assert_eq!(name, "Jane Smith");
        assert_eq!(*age, 28);
        assert_eq!(email, &Some("jane@example.com".to_string()));
        
        // 注意：如果尝试以下代码，将会导致编译错误，因为"unknown_field"不存在于User模型中
        // let invalid_params = sql_params_pro!(<User> {
        //     name: "Invalid",
        //     age: 30,
        //     unknown_field: "This will cause compilation error"
        // });
    }
    
    #[test]
    fn test_user_crud_operations() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        let raw_conn = conn.raw_connection();
        
        // 创建用户表
        conn.execute(User::create_table_sql().as_str(), []).unwrap();
        
        // 创建用户 - INSERT
        let user_data = sql_params!(<User>  {
            name: "Alex Johnson".to_string(),
            age: 35,
            email: Some("alex@example.com".to_string()),
        });

        let query = User::insert_with(&["name", "age", "email"]);
         
        conn.execute(
            &query,
            &*user_data
        ).unwrap();
        
        // 获取用户ID（使用last_insert_rowid）
        let user_id: i32 = raw_conn.last_insert_rowid() as i32;
        
        // 读取用户 - SELECT
        let user_query = format!("SELECT id, name, age, email, created_at, created_at_timestamp, active FROM user WHERE id = {}", user_id);
        let user_data = &conn.query(&user_query, [], 
            |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, i32>(2)?, row.get::<_, Option<String>>(3)?, row.get::<_, UtcDateTime>(4)?, row.get::<_, Timestamp>(5)?, row.get::<_, bool>(6)?))
        ).unwrap()[0];

        eprintln!("User data: {:?}", user_data);
        
        let (db_user_id, name, age, email, created_at, created_at_timestamp, active) = user_data;

        eprintln!("created at {:?}", created_at);
        eprintln!("created at timestamp {:?}", created_at_timestamp);
        
        assert_eq!(*db_user_id, user_id);
        assert_eq!(name, "Alex Johnson");
        assert_eq!(*age, 35);
        assert_eq!(email, &Some("alex@example.com".to_string()));
        assert_eq!(*active, true);

        
        // 更新用户 - UPDATE
        conn.execute(
            "UPDATE user SET name = ?, age = ? WHERE id = ?",
            params![&"Alex Smith", &40, &user_id],
        ).unwrap();
        
        // 验证更新成功
        let updated_data = &conn.query("SELECT name, age FROM user WHERE id = ?", &[&user_id], 
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        ).unwrap()[0];
        
        let (updated_name, updated_age) = updated_data;

        conn.execute("DELETE FROM user WHERE id = ?", &[&user_id]).unwrap();
        
        // 验证删除成功
        let count = conn.query("SELECT COUNT(*) FROM user WHERE id = ?", &[&user_id],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 0);
    }
    
    #[test]
    fn test_post_with_foreign_key() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        let raw_conn = conn.raw_connection();
        
        // 创建用户和帖子表
        conn.execute(User::create_table_sql().as_str(), []).unwrap();
        conn.execute(TestPost::create_table_sql().as_str(), []).unwrap();
        
        // 插入测试用户
        let user_data = sql_params!(<User> {
            name: "Blog Writer".to_string(),
            age: 28,
            email: Some("writer@blog.com".to_string()),
        });
        
        conn.execute(
            &User::insert_with(&["name", "age", "email"]),
            &*user_data
        ).unwrap();
        
        let user_id: i32 = raw_conn.last_insert_rowid() as i32;
        
        // 插入多篇帖子
        let posts = [
            ("First Post", "This is my first blog post", true),
            ("Draft Post", "This is an unpublished draft", false),
            ("Tech Review", "Review of latest technology", true),
        ];
        
        for (title, content, published) in posts.iter() {
            let post_data = sql_params!(<TestPost> {
                title: title.to_string(),
                content: content.to_string(),
                published: *published,
                user_id: user_id,
            });
            
            conn.execute(
                &&TestPost::insert_without_id(),
                &*post_data
            ).unwrap();
        }
        
        // 查询所有已发布的帖子
        let published_posts = conn.query(
            "SELECT id, title FROM testpost WHERE published = ? AND user_id = ? ORDER BY id",
            params![&true, &user_id],
            |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
        ).unwrap();
        
        // 验证有两篇已发布的帖子
        assert_eq!(published_posts.len(), 2);
        assert_eq!(published_posts[0].1, "First Post");
        assert_eq!(published_posts[1].1, "Tech Review");
        
        // 查询单个帖子并更新
        let first_post_id = published_posts[0].0;
        
        // 更新帖子标题
        conn.execute(
            "UPDATE testpost SET title = ? WHERE id = ?",
            params![&"Updated First Post", &first_post_id],
        ).unwrap();
        
        // 验证更新成功
        let updated_title = &conn.query("SELECT title FROM testpost WHERE id = ?", &[&first_post_id],
            |row| row.get::<_, String>(0)
        ).unwrap()[0];
        
        assert_eq!(*updated_title, "Updated First Post");
        
        // 批量删除所有帖子
        conn.execute("DELETE FROM testpost WHERE user_id = ?", &[&user_id]).unwrap();
        
        // 验证删除成功
        let post_count = conn.query("SELECT COUNT(*) FROM testpost WHERE user_id = ?", &[&user_id],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(post_count, 0);
    }
    
    #[test]
    fn test_transaction_and_rollback() {
        // 创建测试数据库连接
        let mut conn = create_test_connection();
        
        // 创建表
        conn.execute(User::create_table_sql().as_str(), []).unwrap();
        
        // 开始事务
        let tx = conn.begin_transaction().unwrap();
        
        // 在事务中插入两个用户
        let users = [
            ("Transaction User 1", 25, Some("tx1@example.com")),
            ("Transaction User 2", 30, Some("tx2@example.com")),
        ];
        
        for (name, age, email) in users.iter() {
            let user_data = sql_params!(<User> {
                name: name.to_string(),
                age: *age,
                email: email.map(|e| e.to_string()),
            });
            
            tx.execute(
                &User::insert_with(&["name", "age", "email"]),
                &*user_data
            ).unwrap();
        }
        
        // 统计事务中插入的用户数
        let count: i32 = tx.query_row(
            "SELECT COUNT(*) FROM user",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(count, 2);
        
        // 回滚事务
        tx.rollback().unwrap();
        
        // 验证用户表是空的（事务已回滚）
        let count = conn.query("SELECT COUNT(*) FROM user", [], 
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 0, "事务回滚后用户表应该为空");
        
        // 新事务: 插入然后提交
        let tx = conn.begin_transaction().unwrap();
        
        let user_data = sql_params!(<User> {
            name: "Committed User".to_string(),
            age: 40,
            email: Some("committed@example.com".to_string()),
        });
        
        tx.execute(
            &User::insert_with(&["name", "age", "email"]),
            &*user_data
        ).unwrap();
        
        // 提交事务
        tx.commit().unwrap();
        
        // 验证用户已被插入并保存
        let count = conn.query("SELECT COUNT(*) FROM user", [], 
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 1, "提交事务后应该有一个用户");
    }

    #[test]
    fn test_data_validation_and_integrity() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        let raw_conn = conn.raw_connection();
        
        // 创建表
        conn.execute(User::create_table_sql().as_str(), []).unwrap();
        conn.execute(TestPost::create_table_sql().as_str(), []).unwrap();
        
        // 测试边界值和特殊字符
        let test_cases = [
            ("Empty Name", "", 0, None),
            ("Max Integer", "Max Value", i32::MAX, Some("max@example.com")),
            ("Min Integer", "Min Value", i32::MIN, Some("min@example.com")),
            ("Special Chars", "O'Neil; DROP TABLE users;--", 25, Some("sql\"injection'test@example.com")),
            ("Unicode Test", "测试用户 👨‍💻", 30, Some("unicode@测试.com")),
            ("Very Long Name", &"A".repeat(1000), 42, Some("long@example.com")),
        ];
        
        for (test_name, name, age, email) in test_cases.iter() {
            println!("Testing: {}", test_name);
            
            let user_data = sql_params!(<User> {
                name: name.to_string(),
                age: *age,
                email: email.map(|e| e.to_string()),
            });
            
            // 插入数据并获取ID
            conn.execute(
                &User::insert_with(&["name", "age", "email"]),
                &*user_data
            ).unwrap();
            
            let user_id: i32 = raw_conn.last_insert_rowid() as i32;
            
            // 读取数据并验证正确性
            let row_data = &conn.query("SELECT name, age, email FROM user WHERE id = ?", &[&user_id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?, row.get::<_, Option<String>>(2)?))
            ).unwrap()[0];
            
            let (db_name, db_age, db_email) = row_data;
            
            assert_eq!(*db_name, *name, "Name should match exactly for {}", test_name);
            assert_eq!(*db_age, *age, "Age should match exactly for {}", test_name);
            assert_eq!(db_email, &email.map(|e| e.to_string()), "Email should match for {}", test_name);
        }
        
        // 清理测试数据
        conn.execute("DELETE FROM user", []).unwrap();
    }
    
    #[test]
    fn test_advanced_queries_and_aggregations() {
        // 创建测试数据库连接
        let mut conn = create_test_connection();
        let raw_conn = conn.raw_connection();
        
        // 创建表
        conn.execute(&User::create_table_sql(), []).unwrap();
        conn.execute(&TestPost::create_table_sql(), []).unwrap();
        
        // 插入多个用户
        let users = [
            ("Alice", 25, Some("alice@example.com")),
            ("Bob", 30, Some("bob@example.com")),
            ("Charlie", 35, Some("charlie@example.com")),
            ("David", 25, None),
            ("Eve", 40, Some("eve@example.com")),
        ];
        
        let mut user_ids = Vec::new();
        
        for (name, age, email) in users.iter() {
            let user_data = sql_params!(<User> {
                name: name.to_string(),
                age: *age,
                email: email.map(|e| e.to_string()),
            });
            
            conn.execute(
                &User::insert_with(&["name", "age", "email"]),
                &*user_data
            ).unwrap();
            
            user_ids.push(raw_conn.last_insert_rowid() as i32);
        }
        
        // 为每个用户添加帖子
        for (idx, user_id) in user_ids.iter().enumerate() {
            // 每个用户添加 idx+1 篇帖子
            for post_idx in 0..=idx {
                let published = post_idx % 2 == 0; // 偶数索引的帖子已发布
                
                let post_data = sql_params!(<TestPost> {
                    title: format!("Post {} by User {}", post_idx, idx),
                    content: format!("Content for post {} by user {}", post_idx, idx),
                    published: published,
                    user_id: *user_id,
                });
                
                conn.execute(
                    &&TestPost::insert_without_id(),
                    &*post_data
                ).unwrap();
            }
        }
        
        // 测试复杂查询1: 按年龄分组计数用户
        let age_counts = conn.query(
            "SELECT age, COUNT(*) FROM user GROUP BY age ORDER BY age",
            [],
            |row| Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
        ).unwrap();
        
        // 验证分组结果
        assert_eq!(age_counts.len(), 4, "应该有4个不同的年龄组");
        assert_eq!(age_counts[0], (25, 2), "年龄25应该有2个用户");
        assert_eq!(age_counts[1], (30, 1), "年龄30应该有1个用户");
        assert_eq!(age_counts[2], (35, 1), "年龄35应该有1个用户");
        assert_eq!(age_counts[3], (40, 1), "年龄40应该有1个用户");
        
        // 测试复杂查询2: 按用户统计已发布的帖子数
        let user_post_counts = conn.query(
            "SELECT u.name, COUNT(p.id) as post_count 
             FROM user u 
             LEFT JOIN testpost p ON u.id = p.user_id AND p.published = 1
             GROUP BY u.id
             ORDER BY post_count DESC",
            [],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        ).unwrap();
        
        // 验证结果: Eve应该有3篇已发布帖子 (索引4，偶数索引0,2,4)
        assert_eq!(user_post_counts[0].0, "Eve", "Eve应该发布最多帖子");
        assert_eq!(user_post_counts[0].1, 3, "Eve应该有3篇已发布帖子");
        
        // 测试复杂查询3: 空电子邮件的用户帖子数
        let count = conn.query(
            "SELECT COUNT(*) FROM testpost p
             JOIN user u ON p.user_id = u.id
             WHERE u.email IS NULL",
            [],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        // David(索引3)没有电子邮件，有4篇帖子
        assert_eq!(count, 4, "没有电子邮件的用户总共应该有4篇帖子");
        
        // 事务测试：更新所有未发布的帖子为已发布
        let tx = conn.begin_transaction().unwrap();
        
        let updated_count = tx.execute(
            "UPDATE testpost SET published = 1 WHERE published = 0",
            [],
        ).unwrap();
        
        // 验证更新计数
        let expected_updates = 1 + 1 + 2 + 2; // 每个用户的未发布帖子 (奇数索引) 数量之和
        assert_eq!(updated_count, expected_updates as usize, "应该更新了{}篇未发布的帖子", expected_updates);
        
        // 提交事务
        tx.commit().unwrap();
        
        // 验证所有帖子都已发布
        let all_published = conn.query(
            "SELECT COUNT(*) = 0 FROM testpost WHERE published = 0",
            [],
            |row| row.get::<_, bool>(0)
        ).unwrap()[0];
        
        assert!(all_published, "所有帖子都应该被标记为已发布");
    }
    
    #[test]
    fn test_error_handling_and_constraints() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        
        // 创建带有约束的表
        conn.execute(
            "CREATE TABLE constrained_user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                email TEXT UNIQUE,
                age INTEGER NOT NULL CHECK(age >= 18)
            )",
            [],
        ).unwrap();
        
        // 创建一个有效用户
        let result = conn.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params!["valid_user", "valid@example.com", &30],
        );
        assert!(result.is_ok(), "有效用户应该可以成功插入");
        
        // 测试唯一约束: 相同用户名
        let result = conn.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params!["valid_user", "another@example.com", &25],
        );
        assert!(result.is_err(), "重复的用户名应该导致错误");
        
        // 测试唯一约束: 相同邮箱
        let result = conn.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params!["another_user", "valid@example.com", &25],
        );
        assert!(result.is_err(), "重复的邮箱应该导致错误");
        
        // 测试检查约束: 年龄小于18
        let result = conn.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params!["young_user", "young@example.com", &17],
        );
        assert!(result.is_err(), "不满足年龄约束应该导致错误");
        
        // 测试非空约束: 用户名为空
        let result = conn.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params![&Option::<String>::None, "no_username@example.com", &20],
        );
        assert!(result.is_err(), "空用户名应该导致错误");
        
        // 验证只有一个有效用户被插入
        let count = conn.query(
            "SELECT COUNT(*) FROM constrained_user",
            [],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 1, "约束测试后应该只有一个有效用户");
    }
    
    #[test]
    fn test_blob_and_complex_data_types() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        let raw_conn = conn.raw_connection();
        
        // 创建表存储二进制数据
        conn.execute(
            "CREATE TABLE binary_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                data BLOB NOT NULL,
                metadata TEXT
            )",
            [],
        ).unwrap();
        
        // 测试不同的二进制数据
        let test_cases = [
            ("Empty", Vec::<u8>::new(), None),
            ("Small Binary", vec![1, 2, 3, 4, 5], Some("small binary file")),
            ("Large Binary", vec![255; 10000], Some("large binary file")), // 10KB 的数据
            ("Binary with zeros", vec![0, 1, 0, 1, 0], Some("binary with zeros")),
        ];
        
        for (name, data, metadata) in test_cases.iter() {
            let result = conn.execute(
                "INSERT INTO binary_data (name, data, metadata) VALUES (?, ?, ?)",
                params![name, &data, &metadata],
            );
            assert!(result.is_ok(), "Binary data insertion should succeed for {}", name);
            
            // 获取插入的ID
            let id: i32 = raw_conn.last_insert_rowid() as i32;
            
            // 读取并验证数据
            let data_result = &conn.query("SELECT data FROM binary_data WHERE id = ?", &[&id],
                |row| row.get::<_, Vec<u8>>(0)
            ).unwrap()[0];
            
            assert_eq!(*data_result, *data, "Retrieved binary data should match for {}", name);
        }
        
        // 验证可以按二进制数据长度查询
        let count = conn.query(
            "SELECT COUNT(*) FROM binary_data WHERE length(data) > 1000",
            [],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 1, "应该只有一个大于1000字节的二进制数据");
    }
    
    #[test]
    fn test_savepoint_operations() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        
        // 创建表
        conn.execute(&User::create_table_sql(), []).unwrap();
        
        // 创建命名保存点
        let sp1 = conn.savepoint("sp1").unwrap();
        
        // 在保存点中插入第一个用户
        let user1 = sql_params!(<User> {
            name: "Savepoint User 1".to_string(),
            age: 31,
            email: Some("sp1@example.com".to_string()),
        });
        sp1.execute(&&User::insert_with(&["name", "age", "email"]), &*user1).unwrap();
        
        // 提交第一个保存点
        sp1.commit().unwrap();
        
        // 验证用户1已插入
        let count = conn.query(
            "SELECT COUNT(*) FROM user WHERE name = ?", 
            &["Savepoint User 1"],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 1, "保存点提交后应该有一个用户");
        
        // 创建自动命名的保存点
        let sp2 = conn.savepoint_unique().unwrap();
        
        // 在保存点中插入第二个用户
        let user2 = sql_params!(<User> {
            name: "Savepoint User 2".to_string(),
            age: 32,
            email: Some("sp2@example.com".to_string()),
        });
        sp2.execute(&&User::insert_with(&["name", "age", "email"]), &*user2).unwrap();
        
        // 回滚第二个保存点
        sp2.rollback().unwrap();
        
        // 验证用户2未被插入
        let count = conn.query(
            "SELECT COUNT(*) FROM user WHERE name = ?", 
            &["Savepoint User 2"],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 0, "保存点回滚后不应该有第二个用户");
        
        // 验证用户1仍然存在
        let count = conn.query(
            "SELECT COUNT(*) FROM user WHERE name = ?", 
            &["Savepoint User 1"],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 1, "保存点回滚后第一个用户应该仍然存在");
        
        // 测试嵌套保存点
        let sp_outer = conn.savepoint("outer").unwrap();
        
        // 在外层保存点中插入用户
        let user_outer = sql_params!(<User> {
            name: "Outer User".to_string(),
            age: 41,
            email: Some("outer@example.com".to_string()),
        });
        sp_outer.execute(&User::insert_with(&["name", "age", "email"]), &*user_outer).unwrap();
        
        // 创建内层保存点
        let sp_inner = conn.savepoint("inner").unwrap();
        
        // 在内层保存点中插入用户
        let user_inner = sql_params!(<User> {
            name: "Inner User".to_string(),
            age: 42,
            email: Some("inner@example.com".to_string()),
        });
        sp_inner.execute(&User::insert_with(&["name", "age", "email"]), &*user_inner).unwrap();
        
        // 回滚内层保存点
        sp_inner.rollback().unwrap();
        
        // 提交外层保存点
        sp_outer.commit().unwrap();
        
        // 验证外层用户被插入但内层用户未被插入
        let outer_count = conn.query(
            "SELECT COUNT(*) FROM user WHERE name = ?", 
            &["Outer User"],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        let inner_count = conn.query(
            "SELECT COUNT(*) FROM user WHERE name = ?", 
            &["Inner User"],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(outer_count, 1, "外层保存点的用户应该被插入");
        assert_eq!(inner_count, 0, "内层保存点的用户应该未被插入");
    }
}