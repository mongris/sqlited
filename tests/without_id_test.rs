#[cfg(test)]
mod tests {
    use std::thread;

    use rusqlite::params;
    use sqlited::{
        check, define_db, not_null, prelude::*, query, sql, sql_params, sql_str, table, unique, without_id, UtcDateTime
    };

    // 定义一个用于测试的用户模型
    #[table]
    struct User {
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
    }
    
    // 定义一个用于测试的帖子模型
    #[table]
    struct TestPost {
        #[autoincrement]
        id: i32,
        title: String,
        content: String,
        published: bool,
        user_id: i32,
        long_u64: u64,
        long_i64: i64,
    }

    #[table]
    struct ConstrainedUser {
        #[autoincrement]
        id: i32,
        #[unique]
        username: String,
        #[unique]
        email: Option<String>,
        #[check("age >= 18")]
        age: i32,
    }

    // 使用 define_db 定义测试数据库
    define_db!(
        pub static ref TEST_DB: TestDb<()> = [
            User,
            TestPost,
            
            // 创建带有约束的表
            ConstrainedUser,
            
            // 创建表存储二进制数据
            "CREATE TABLE IF NOT EXISTS binary_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                data BLOB NOT NULL,
                metadata TEXT
            )"
        ]
    );

    impl TestDb {
        query! {
            fn get_user_by_name(name: &str) -> Result<User> {
                SELECT * FROM User WHERE name = ?
            }
        }

        query! {
            fn get_user_by_id(id: i32) -> anyhow::Result<User> {
                SELECT * FROM User WHERE id = ? LIMIT 1
            }
        }

        query! {
            fn get_users_by_age(age: i32) -> Result<Vec<User>> {
                SELECT * FROM User WHERE age > ?1
            }
        }

        query! {
            fn get_user_name(id: i32) -> Result<String> {
                SELECT name FROM User WHERE id = ?1
            }
        }

        query! {
            fn get_some_info_by_id(id: i32) -> Result<(i32, String)> {
                SELECT id, name FROM User WHERE id = ?1
            }
        }

        query! {
            fn save_user(id: i32, name: &str, age: i32) -> Result<()> {
                UPDATE User SET name = ?2, age = ?3 WHERE id = ?1
            }
        }

        query! {
            fn get_last_user() -> Result<ConstrainedUser> {
                SELECT * FROM ConstrainedUser ORDER BY id DESC LIMIT 1
            }
        }
        

        pub fn get_user_by_name2(&self, name: String) -> sqlited::Result<User> {
            let s = sql_str!(UPDATE User SET name = ?2 WHERE id = ?1);
            let params = sql_params!(<User> {
                name: name,
            });
            let query = sql!(
                SELECT * FROM user WHERE name = ?1,
                &params
            );
            query.query_row(&self.get_conn()?, User::from_row)
        }

        pub fn get_user_by_age2(&self, age2: i32) -> sqlited::Result<User> {
            let query = sql_str!(SELECT * FROM user WHERE age > ?);
            self.query_row(query, params![age2], User::from_row)
        }

        query! {
            fn get_published_posts_by_user(user_id: i32) -> Result<Vec<TestPost>> {
                SELECT * FROM TestPost WHERE user_id = ? AND published = 1
            }
        }

        pub fn get_published_posts_by_user2(&self, user_id: i32) -> sqlited::Result<Vec<TestPost>> {
            let query = sql_str!(SELECT * FROM test_post WHERE user_id = ? AND published = 1);
            self.query(query, params![user_id], TestPost::from_row)
        }
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
        // 创建内存数据库
        let db = TEST_DB::memory().unwrap();
        
        // 使用 sql_params 宏
        let params = sql_params!(User {
            name: "Jane Smith".to_string(),
            age: 28,
            email: Some("jane@example.com".to_string()),
        });

        let query = sql!(
            INSERT INTO user (name, age, email) VALUES (?, ?, ?),
            User {
                name: "Jane Smith".to_string(),
                age: 28,
                email: Some("jane@example.com".to_string()),
            }
        );
        
        // 使用参数进行插入操作
        eprintln!("SQL2: {}, params count: {}", query.query, params.len());
        let result = query.execute(&db.get_conn().unwrap());
        
        // 验证插入成功
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1); // 应该插入了一行
        
        // 验证数据被正确插入
        let row_data = &db.query("SELECT name, age, email FROM user WHERE rowid = 1", [], 
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?, row.get::<_, Option<String>>(2)?))
        ).unwrap()[0];

        
        let (name, age, email) = row_data;
        assert_eq!(name, "Jane Smith");
        assert_eq!(*age, 28);
        assert_eq!(email, &Some("jane@example.com".to_string()));

        let user = &db.get_user_by_name("Jane Smith").unwrap();

        assert_eq!(user.name, "Jane Smith");
        assert_eq!(user.age, 28);
        assert_eq!(email, &Some("jane@example.com".to_string()));

    }

    #[test]
    fn test_sql() {
        let q = sql_str!(
            INSERT INTO User (
                id,
                name,
                age,
                email
            ) VALUES (
                ?1,
                ?2,
                ?3,
                ?4
            )
            ON CONFLICT DO UPDATE SET
                name = ?2,
                age = ?3,
                email = ?4
        );
    }
    
    #[test]
    fn test_user_crud_operations() {
        // 使用内存数据库
        let db = TEST_DB::memory().unwrap();
        let raw_conn = db.get_conn().expect("获取连接失败");
        
        // 创建用户 - INSERT
        let user_data = sql_params!(<User> {
            name: "Alex Johnson",
            age: 35,
            email: Some("alex@example.com"),
        });

        let query = User::insert_with(&["name", "age", "email"]);
         
        let user_id = db.execute_insert(
            &query,
            &*user_data
        ).unwrap() as i32;
        
        // 读取用户 - SELECT
        let user_query = format!("SELECT id, name, age, email, created_at, created_at_timestamp, active FROM user WHERE id = {}", user_id);
        let user_data = &db.query(&user_query, [], 
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

        let name = db.get_user_name(user_id).unwrap();

        assert_eq!(name, "Alex Johnson");

        let (db_user_id, name) = db.get_some_info_by_id(user_id).unwrap();

        assert_eq!(db_user_id, user_id);
        assert_eq!(name, "Alex Johnson");
        
        // 更新用户 - UPDATE
        // db.execute(
        //     "UPDATE user SET name = ?, age = ? WHERE id = ?",
        //     params![&"Alex Smith", &40, &user_id],
        // ).unwrap();

        db.save_user(user_id, "Alex Smith", 40);
        
        // 验证更新成功
        let updated_data = &db.query("SELECT name, age FROM user WHERE id = ?", &[&user_id], 
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        ).unwrap()[0];
        
        let (_updated_name, _updated_age) = updated_data;

        db.execute("DELETE FROM user WHERE id = ?", &[&user_id]).unwrap();
        
        // 验证删除成功
        let count = db.query("SELECT COUNT(*) FROM user WHERE id = ?", &[&user_id],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 0);
    }
    
    #[test]
    fn test_post_with_foreign_key() {
        // 使用内存数据库
        let db = TEST_DB::memory().unwrap();
        let raw_conn = db.get_conn().expect("获取连接失败");
        
        // 插入测试用户
        let user_data = sql_params!(<User> {
            name: "Blog Writer".to_string(),
            age: 28,
            email: Some("writer@blog.com".to_string()),
        });
        
        db.execute(
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
                long_u64: 9223372036854775807u64,
                long_i64: 1234567890123456789i64,
            });
            
            db.execute(
                &&TestPost::insert_without_id(),
                &*post_data
            ).unwrap();
        }
        
        // 查询所有已发布的帖子
        let published_posts = db.query(
            "SELECT id, title, long_u64 FROM test_post WHERE published = ? AND user_id = ? ORDER BY id",
            params![&true, &user_id],
            |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, u64>(2)?))
        ).unwrap();

        // 验证有两篇已发布的帖子
        assert_eq!(published_posts.len(), 2);
        assert_eq!(published_posts[0].1, "First Post");
        assert_eq!(published_posts[1].1, "Tech Review");
        assert_eq!(published_posts[0].2, 9223372036854775807u64);
        

        let published_posts = db.get_published_posts_by_user(user_id).unwrap();
        
        // 验证有两篇已发布的帖子
        assert_eq!(published_posts.len(), 2);
        assert_eq!(published_posts[0].title, "First Post");
        assert_eq!(published_posts[1].title, "Tech Review");
        assert_eq!(published_posts[0].long_u64, 9223372036854775807u64);
        
        
        // 查询单个帖子并更新
        let first_post_id = published_posts[0].id;
        
        // 更新帖子标题
        db.execute(
            "UPDATE test_post SET title = ? WHERE id = ?",
            params![&"Updated First Post", &first_post_id],
        ).unwrap();
        
        // 验证更新成功
        let updated_title = &db.query("SELECT title FROM test_post WHERE id = ?", &[&first_post_id],
            |row| row.get::<_, String>(0)
        ).unwrap()[0];
        
        assert_eq!(*updated_title, "Updated First Post");
        
        // 批量删除所有帖子
        db.execute("DELETE FROM test_post WHERE user_id = ?", &[&user_id]).unwrap();
        
        // 验证删除成功
        let post_count = db.query("SELECT COUNT(*) FROM test_post WHERE user_id = ?", &[&user_id],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(post_count, 0);
    }
    
    #[test]
    fn test_transaction_and_rollback() {
        // 使用内存数据库
        let db = TEST_DB::memory().unwrap();
        
        // 使用事务支持
        db.transaction(|tx| {
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
                )?;
            }
            
            // 统计事务中插入的用户数
            let count: i32 = tx.query_row(
                "SELECT COUNT(*) FROM user",
                [],
                |row| row.get(0),
            )?;
            
            assert_eq!(count, 2);
            
            // 故意返回错误以回滚事务
            Err::<(), _>(sqlited::SqlitedError::from(rusqlite::Error::StatementChangedRows(0)))
        }).unwrap_err(); // 我们期望事务失败
        
        // 验证用户表是空的（事务已回滚）
        let count = db.query("SELECT COUNT(*) FROM user", [], 
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 0, "事务回滚后用户表应该为空");
        
        // 新事务: 插入然后提交
        db.transaction(|tx| {
            let user_data = sql_params!(<User> {
                name: "Committed User".to_string(),
                age: 40,
                email: Some("committed@example.com".to_string()),
            });
            
            tx.execute(
                &User::insert_with(&["name", "age", "email"]),
                &*user_data
            )?;
            
            Ok(())
        }).unwrap();
        
        // 验证用户已被插入并保存
        let count = db.query("SELECT COUNT(*) FROM user", [], 
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 1, "提交事务后应该有一个用户");
    }

    #[test]
    fn test_data_validation_and_integrity() {
        // 使用内存数据库
        let db = TEST_DB::memory().unwrap();
        
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
            let user_id = db.execute_insert(
                &User::insert_with(&["name", "age", "email"]),
                &*user_data
            ).unwrap();
            
            // 读取数据并验证正确性
            let row_data = &db.query("SELECT name, age, email FROM user WHERE id = ?", &[&user_id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?, row.get::<_, Option<String>>(2)?))
            ).unwrap()[0];
            
            let (db_name, db_age, db_email) = row_data;
            
            assert_eq!(*db_name, *name, "Name should match exactly for {}", test_name);
            assert_eq!(*db_age, *age, "Age should match exactly for {}", test_name);
            assert_eq!(db_email, &email.map(|e| e.to_string()), "Email should match for {}", test_name);
        }
        
        // 清理测试数据
        db.execute("DELETE FROM user", []).unwrap();
    }
    
    #[test]
    fn test_advanced_queries_and_aggregations() {
        // 使用内存数据库
        let db = TEST_DB::memory().unwrap();
        
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
            
            let user_id = db.execute_insert(
                &User::insert_with(&["name", "age", "email"]),
                &*user_data
            ).unwrap() as i32;
            
            user_ids.push(user_id);
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
                    long_u64: 9876543210u64, // 添加缺失的 u64 字段
                    long_i64: -1234567890i64, // 添加缺失的 i64 字段
                });
                
                db.execute(
                    &&TestPost::insert_without_id(),
                    &*post_data
                ).unwrap();
            }
        }
        
        // 测试复杂查询1: 按年龄分组计数用户
        let age_counts = db.query(
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
        let user_post_counts = db.query(
            "SELECT u.name, COUNT(p.id) as post_count 
             FROM user u 
             LEFT JOIN test_post p ON u.id = p.user_id AND p.published = 1
             GROUP BY u.id
             ORDER BY post_count DESC",
            [],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        ).unwrap();
        
        // 验证结果: Eve应该有3篇已发布帖子 (索引4，偶数索引0,2,4)
        assert_eq!(user_post_counts[0].0, "Eve", "Eve应该发布最多帖子");
        assert_eq!(user_post_counts[0].1, 3, "Eve应该有3篇已发布帖子");
        
        // 测试复杂查询3: 空电子邮件的用户帖子数
        let count = db.query(
            "SELECT COUNT(*) FROM test_post p
             JOIN user u ON p.user_id = u.id
             WHERE u.email IS NULL",
            [],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        // David(索引3)没有电子邮件，有4篇帖子
        assert_eq!(count, 4, "没有电子邮件的用户总共应该有4篇帖子");
        
        // 使用事务更新所有未发布的帖子为已发布
        db.transaction(|tx| {
            let updated_count = tx.execute(
                "UPDATE test_post SET published = 1 WHERE published = 0",
                [],
            )?;
            
            // 验证更新计数
            let expected_updates = 1 + 1 + 2 + 2; // 每个用户的未发布帖子 (奇数索引) 数量之和
            assert_eq!(updated_count, expected_updates as usize, "应该更新了{}篇未发布的帖子", expected_updates);
            
            Ok(())
        }).unwrap();
        
        // 验证所有帖子都已发布
        let all_published = db.query(
            "SELECT COUNT(*) = 0 FROM test_post WHERE published = 0",
            [],
            |row| row.get::<_, bool>(0)
        ).unwrap()[0];
        
        assert!(all_published, "所有帖子都应该被标记为已发布");
    }
    
    #[test]
    fn test_error_handling_and_constraints() {
        // 使用内存数据库
        let db = TEST_DB::memory().unwrap();
        
        // 创建一个有效用户
        let result = db.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params!["valid_user", "valid@example.com", &30],
        );
        assert!(result.is_ok(), "有效用户应该可以成功插入");
        
        // 测试唯一约束: 相同用户名
        let result = db.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params!["valid_user", "another@example.com", &25],
        );
        assert!(result.is_err(), "重复的用户名应该导致错误");
        
        // 测试唯一约束: 相同邮箱
        let result = db.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params!["another_user", "valid@example.com", &25],
        );
        assert!(result.is_err(), "重复的邮箱应该导致错误");
        
        // 测试检查约束: 年龄小于18
        let result = db.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params!["young_user", "young@example.com", &17],
        );
        assert!(result.is_err(), "不满足年龄约束应该导致错误");
        
        // 测试非空约束: 用户名为空
        let result = db.execute(
            "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
            params![&Option::<String>::None, "no_username@example.com", &20],
        );
        assert!(result.is_err(), "空用户名应该导致错误");
        
        // 验证只有一个有效用户被插入
        let count = db.query(
            "SELECT COUNT(*) FROM constrained_user",
            [],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 1, "约束测试后应该只有一个有效用户");

        let user = db.get_last_user().unwrap();
        assert_eq!(user.username, "valid_user", "最后插入的用户应该是有效用户");
        assert_eq!(user.age, 30, "有效用户的年龄应该是30");
    }
    
    #[test]
    fn test_blob_and_complex_data_types() {
        // 使用内存数据库
        let db = TEST_DB::memory().unwrap();
        
        // 测试不同的二进制数据
        let test_cases = [
            ("Empty", Vec::<u8>::new(), None),
            ("Small Binary", vec![1, 2, 3, 4, 5], Some("small binary file")),
            ("Large Binary", vec![255; 10000], Some("large binary file")), // 10KB 的数据
            ("Binary with zeros", vec![0, 1, 0, 1, 0], Some("binary with zeros")),
        ];
        
        for (name, data, metadata) in test_cases.iter() {
            let result = db.execute_insert(
                "INSERT INTO binary_data (name, data, metadata) VALUES (?, ?, ?)",
                params![name, &data, &metadata],
            );
            assert!(result.is_ok(), "Binary data insertion should succeed for {}", name);
            
            // 获取插入的ID
            let id: i32 = result.unwrap() as i32;
            
            // 读取并验证数据
            let data_result = &db.query("SELECT data FROM binary_data WHERE id = ?", &[&id],
                |row| row.get::<_, Vec<u8>>(0)
            ).unwrap()[0];
            
            assert_eq!(*data_result, *data, "Retrieved binary data should match for {}", name);
        }
        
        // 验证可以按二进制数据长度查询
        let count = db.query(
            "SELECT COUNT(*) FROM binary_data WHERE length(data) > 1000",
            [],
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 1, "应该只有一个大于1000字节的二进制数据");
    }
    
    #[test]
    fn test_multiple_connections() {
        // 使用共享内存数据库
        let db1 = TEST_DB::shared_memory("test_multi_conn").unwrap();
        
        // 插入测试数据
        db1.execute(
            "INSERT INTO user (name, age, email) VALUES (?, ?, ?)",
            params!["Connection Test", &35, &"conn@test.com"],
        ).unwrap();
        
        // 创建第二个连接到同一数据库
        let db2 = TEST_DB::shared_memory("test_multi_conn").unwrap();
        
        // 使用第二个连接验证数据
        let name = &db2.query(
            "SELECT name FROM user WHERE email = ?",
            &["conn@test.com"],
            |row| row.get::<_, String>(0)
        ).unwrap()[0];
        
        assert_eq!(name, "Connection Test", "第二个连接应该能看到第一个连接插入的数据");
        
        // 使用新连接API
        let db3 = db1.new_connection().unwrap();
        
        // 通过第三个连接修改数据
        db3.execute(
            "UPDATE user SET name = ? WHERE email = ?",
            params!["Updated Name", &"conn@test.com"],
        ).unwrap();
        
        // 验证第一个连接可以看到更改
        let updated_name = &db1.query(
            "SELECT name FROM user WHERE email = ?",
            &["conn@test.com"],
            |row| row.get::<_, String>(0)
        ).unwrap()[0];
        
        assert_eq!(updated_name, "Updated Name", "原始连接应该能看到新连接所做的更改");
    }

    #[test]
    fn test_async_query() {
        // 使用内存数据库
        let db = TEST_DB::memory().unwrap();
        
        thread::spawn(move || {
            let user = db.get_user_by_name("Updated Name");
            assert!(user.is_ok(), "异步查询应该成功");
            let user = user.unwrap();
            assert_eq!(user.name, "Updated Name", "异步查询的用户名称应该匹配");
            assert_eq!(user.age, 35, "异步查询的用户年龄应该匹配");
        });
    }
}