#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rusqlite::{types::{FromSql, ValueRef}, ToSql, Result as SqliteResult};
    use serde::{Deserialize, Serialize};
    use crate::{
        connection::{SqliteConnection, new_memory_pool, get_connection},
        table,
        sql,
        bindable_value
    };

    // 定义枚举
    #[derive(Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Axis {
        #[default]
        Horizontal,
        Vertical,
    }

    // 使用宏生成序列化类型
    bindable_value!(
        enum SerializedAxis(Axis) {
            Horizontal => "Horizontal",
            Vertical => "Vertical",
        }
    );

    // 定义复杂结构体，用于二进制序列化
    #[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Layout {
        pub x: i32,
        pub y: i32,
        pub width: Option<i32>,
        pub height: Option<i32>,
        pub fullscreen: bool,
        pub axis: Axis,
    }

    // 使用二进制序列化
    bindable_value!(binary BinaryLayout(Layout));

    // 定义复杂结构体，用于 JSON 序列化
    #[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Config {
        pub name: String,
        pub settings: HashMap<String, String>,
        pub enabled_features: Vec<String>,
    }

    // 使用 JSON 序列化
    bindable_value!(json JsonConfig(Config));

    // 定义测试表
    table! {
        CustomTypes {
            #[autoincrement]
            id: i32,
            name: String,
            axis: SerializedAxis,
            layout: BinaryLayout,
            config: JsonConfig
        }
    }

    // 定义测试表
    table! {
        TestCustom {
            #[autoincrement]
            id: i32,
            name: String, 
            axis: SerializedAxis
        }
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
            INSERT INTO testcustom (name, axis) VALUES (?, ?),
            TestCustom {
                name: "水平测试".to_string(),
                axis: SerializedAxis(Axis::Vertical),
            }
        );
        
        let result = query.execute(&conn);
        assert!(result.is_ok());
        
        // 验证插入的数据
        let (data_name, data_axis) = &conn.query(
            "SELECT name, axis FROM testcustom WHERE id = 1", 
            [], 
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, SerializedAxis>(1)?))
        ).unwrap()[0];
        
        assert_eq!(data_name, "水平测试");
        assert_eq!(*data_axis, SerializedAxis(Axis::Vertical));
    }

    #[test]
    fn test_bindable_value_types() {
        // 创建连接和表
        let conn = create_test_connection();
        conn.execute(CustomTypes::create_table_sql().as_str(), []).unwrap();
        
        // 创建测试数据
        let axis = SerializedAxis(Axis::Vertical);
        
        let layout = Layout {
            x: 100,
            y: 200,
            width: Some(800),
            height: Some(600),
            fullscreen: true,
            axis: Axis::Horizontal,
        };
        
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
            INSERT INTO customtypes (name, axis, layout, config) VALUES (?, ?, ?, ?),
            CustomTypes {
                name: "混合类型测试".to_string(),
                axis,
                layout: BinaryLayout(layout.clone()),
                config: JsonConfig(config.clone()),
            }
        );
        
        let result = query.execute(&conn);
        assert!(result.is_ok());
        
        // 验证插入的数据
        let results = conn.query(
            "SELECT name, axis, layout, config FROM customtypes WHERE id = 1", 
            [], 
            |row| Ok((
                row.get::<_, String>(0)?,
                row.get::<_, SerializedAxis>(1)?,
                row.get::<_, BinaryLayout>(2)?,
                row.get::<_, JsonConfig>(3)?
            ))
        ).unwrap();
        
        let data = results.get(0).expect("未找到查询结果");
        
        // 验证各字段
        assert_eq!(data.0, "混合类型测试");
        assert_eq!(data.1, axis);
        assert_eq!(data.2.0, layout);
        assert_eq!(data.3.0, config);
    }
}
