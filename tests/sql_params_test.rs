#[cfg(test)]
mod tests {
    use sqlited::{
        prelude::*,
        table,
        sql_params
    };

    // 重新定义用户模型
    table! {
        struct User {
            #[autoincrement]
            id: i32,
            name: String,
            age: i32,
            email: Option<String>,
        }
    }

    // 创建测试数据库连接的辅助函数
    fn create_test_connection() -> SqliteConnection {
        let pool = new_memory_pool().expect("创建内存连接池失败");
        get_connection(&pool).expect("从连接池获取连接失败")
    }

    #[test]
    fn test_sql_params_macro() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        
        // 创建用户表
        conn.execute(User::create_table_sql().as_str(), []).unwrap();
        
        // 使用新的 sql_params 宏，使用一致的格式
        let params = sql_params!(<User> {
            name: "John Smith".to_string(),
            age: 35,
            email: Some("john@example.com".to_string())
        });
        
        // 使用参数进行插入操作
        let sql = "INSERT INTO user (name, age, email) VALUES (?, ?, ?)";
        let result = conn.execute(sql, &*params);
        
        // 验证插入成功
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        
        // 验证数据被正确插入
        let row_data = &conn.query(
            "SELECT name, age, email FROM user WHERE rowid = 1", 
            [], 
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?, row.get::<_, Option<String>>(2)?))
        ).unwrap()[0];
        
        let (name, age, email) = row_data;
        assert_eq!(name, "John Smith");
        assert_eq!(*age, 35);
        assert_eq!(email, &Some("john@example.com".to_string()));
    }
    
    #[test]
    fn test_compile_time_field_validation() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        
        // 创建用户表
        conn.execute(User::create_table_sql().as_str(), []).unwrap();
        
        // 使用 sql_params 宏，所有字段都是有效的，为 None 提供显式类型注解
        let params = sql_params!(<User> {
            name: "Alice Smith".to_string(),
            age: 28,
            email: None::<String>
        });
        
        // 执行插入并验证成功
        let result = conn.execute(
            "INSERT INTO user (name, age, email) VALUES (?, ?, ?)",
            &*params
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        
        // 注意：以下代码会导致编译错误，这是预期行为
        // 取消注释下面的代码块将导致编译失败
        /*
        let invalid_params = sql_params!(<User> {
            name: "Invalid User",
            age: 25,
            address: "123 Main St" // 编译错误！这个字段在 User 模型中不存在
        });
        */
    }
    
    #[test]
    fn test_compare_with_original_macro() {
        // 创建测试数据库连接
        let conn = create_test_connection();
        
        // 创建用户表
        conn.execute(User::create_table_sql().as_str(), []).unwrap();
        
        // 使用新的过程宏
        let params_pro = sql_params!(<User> {
            name: "Pro User".to_string(),
            age: 40,
            email: Some("pro@example.com".to_string())
        });
        
        // 使用原始宏
        let params_original = sql_params!(User {
            name: "Original User".to_string(),
            age: 45,
            email: Some("original@example.com".to_string())
        });
        
        // 插入两条记录
        let sql = "INSERT INTO user (name, age, email) VALUES (?, ?, ?)";
        
        conn.execute(sql, &*params_pro).unwrap();
        conn.execute(sql, &*params_original).unwrap();
        
        // 验证两条记录都被正确插入
        let count = conn.query(
            "SELECT COUNT(*) FROM user", 
            [], 
            |row| row.get::<_, i32>(0)
        ).unwrap()[0];
        
        assert_eq!(count, 2);
        
        // 验证两种方法的结果类型相同
        assert_eq!(std::any::type_name_of_val(&params_pro), std::any::type_name_of_val(&params_original));
    }
}