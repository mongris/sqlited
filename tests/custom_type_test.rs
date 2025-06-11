#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use solana_pubkey::Pubkey;
    use sqlited::{
        prelude::*, sql, sql_as, table
    };

    // 定义枚举
    #[sql_as(binary)]
    pub enum Axis {
        #[default]
        Horizontal,
        Vertical(i32),
    }

    // 定义复杂结构体，用于二进制序列化
    #[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct OriginalLayout {
        pub x: i32,
        pub y: i32,
        pub width: Option<i32>,
        pub height: Option<i32>,
        pub fullscreen: bool,
        pub axis: Axis,
        pub key: Option<Pubkey>,
    }

    #[sql_as(binary)]
    pub struct Layout(pub OriginalLayout);

    // 定义复杂结构体，用于 JSON 序列化
    #[sql_as(jsonb)]
    pub struct Config {
        pub name: String,
        pub settings: HashMap<String, String>,
        pub enabled_features: Vec<String>,
    }

    // 定义测试表
    #[table]
    struct CustomTypes {
        #[autoincrement]
        id: i32,
        name: String,
        axis: Axis,
        layout: Layout,
        config: Config,
        tags: Option<Vec<String>>, // 可选字段，存储字符串数组
        signers: Option<Vec<Pubkey>>, // 可选字段，存储 Pubkey 数组
    }

    // 定义测试表
    #[table]
    struct TestCustom {
        #[autoincrement]
        id: i32,
        name: String, 
        axis: Axis
    }

    // 辅助函数：创建内存数据库连接
    fn create_test_connection() -> SqliteConnection {
        let pool = new_memory_pool().expect("创建内存连接池失败");
        get_connection(&pool).expect("从连接池获取连接失败")
    }
    
    #[test]
    fn test_custom_type() {
        // 创建连接和表
        let conn = create_test_connection();
        conn.execute(TestCustom::create_table_sql().as_str(), []).unwrap();
        
        // 插入数据
        let query = sql!(
            INSERT INTO test_custom (name, axis) VALUES (?, ?),
            TestCustom {
                name: "水平测试",
                axis: Axis::Vertical(1),
            }
        );
        
        let result = query.execute(&conn);
        assert!(result.is_ok());
        
        // 验证插入的数据
        let (data_name, data_axis) = &conn.query(
            "SELECT name, axis FROM test_custom WHERE id = 1", 
            [], 
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, Axis>(1)?))
        ).unwrap()[0];
        
        assert_eq!(data_name, "水平测试");
        assert_eq!(*data_axis, Axis::Vertical(1));
    }

    #[test]
    fn test_bindable_types() {
        // 创建连接和表
        let conn = create_test_connection();
        conn.execute(CustomTypes::create_table_sql().as_str(), []).unwrap();
        
        // 创建测试数据
        let axis = Axis::Vertical(1);

        let pubkey = Pubkey::new_unique();
        
        let mut layout = Layout(OriginalLayout {
            x: 1,
            y: 200,
            width: Some(800),
            height: Some(600),
            fullscreen: true,
            axis: Axis::Horizontal,
            key: Some(pubkey),
        });

        layout.x = 100;
        
        let mut settings = HashMap::new();
        settings.insert("theme".to_string(), "dark".to_string());
        settings.insert("font_size".to_string(), "12".to_string());
        
        let config = Config {
            name: "测试配置".to_string(),
            settings,
            enabled_features: vec!["search".to_string(), "preview".to_string()],
        };
        
        // 插入数据
        let query = sql!(
            INSERT INTO custom_types (name, axis, layout, config, tags, signers) VALUES (?, ?, ?, ?, ?, ?),
            CustomTypes {
                name: "混合类型测试".to_string(),
                axis,
                layout: layout.clone(),
                config: config.clone(),
                tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
                signers: Some(vec![pubkey]),
            }
        );
        
        let result = query.execute(&conn);
        assert!(result.is_ok());
        
        // 验证插入的数据
        let results = conn.query(
            "SELECT name, axis, layout, config, signers FROM custom_types WHERE id = 1", 
            [], 
            |row| Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Axis>(1)?,
                row.get::<_, Layout>(2)?,
                row.get::<_, Config>(3)?,
                row.get::<_, Option<Vec<Pubkey>>>(4)?,
            ))
        ).unwrap();
        
        let data = results.get(0).expect("未找到查询结果");
        
        // 验证各字段
        assert_eq!(data.0, "混合类型测试");
        assert_eq!(data.1, axis);
        assert_eq!(data.2, layout);
        assert_eq!(data.2.0.key.unwrap(), pubkey);
        assert_eq!(data.3, config);
        assert_eq!(data.4.as_ref().unwrap()[0], pubkey);
    }
}
