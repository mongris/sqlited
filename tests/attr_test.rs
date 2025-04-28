#[cfg(test)]
mod tests {
    use rusqlite::params;
    use sqlited::{
        prelude::*,
        define_db,
        table,
        UtcDateTime,
        Timestamp,
    };

    // 测试自增主键和非空约束
    #[table]
    struct BasicTable {
        #[autoincrement]
        id: i32,
        #[not_null]
        name: String,
        description: Option<String>
    }

    // 测试默认值属性
    #[table]
    struct DefaultValuesTable {
        #[autoincrement]
        id: i32,
        name: String,
        #[default("Guest")]
        username: String,
        #[default("now")]
        created_at: UtcDateTime,
        #[default("now")]
        timestamp: Timestamp,
        #[default("1")]
        is_active: bool,
        #[default("0")]
        is_deleted: bool,
        #[default("3.14")]
        pi_value: f64
    }

    // 测试唯一约束
    #[table]
    struct UniqueTable {
        #[autoincrement]
        id: i32,
        #[unique]
        username: String,
        email: String
    }

    // 测试检查约束
    #[table]
    struct CheckConstraintTable {
        #[autoincrement]
        id: i32,
        name: String,
        #[check("age > 0 AND age < 150")]
        age: i32,
        #[check("salary >= 0")]
        salary: f64
    }

    // 测试外键约束
    #[table]
    struct ParentTable {
        #[autoincrement]
        id: i32,
        name: String
    }

    #[table]
    struct ForeignKeyTable {
        #[autoincrement]
        id: i32,
        name: String,
        #[foreign_key("parent_table", "id", "CASCADE", "CASCADE")]
        parent_id: i32
    }

    // 测试表级约束和索引
    #[table]
    #[constraint("UNIQUE(first_name, last_name)")]
    #[index("name_idx", "first_name, last_name")]
    #[unique_index("email_idx", "email")]
    struct TableConstraintsTable {
        #[autoincrement]
        id: i32,
        first_name: String,
        last_name: String,
        email: String
    }

    // 测试主键非自增
    #[table]
    struct CustomPrimaryKeyTable {
        #[primary_key]
        uuid: String,
        data: String
    }

    // 定义一个综合多种属性的表
    #[table]
    struct CombinedAttributesTable {
        #[autoincrement]
        id: i32,
        #[not_null]
        #[unique]
        name: String,
        #[default("now")]
        created_at: UtcDateTime,
        #[check("age >= 18")]
        age: i32,
        #[default("true")]
        is_active: bool
    }

    // 定义测试数据库
    define_db!(
        pub static ref ATTR_TEST_DB: AttrTestDb<()> = [
            BasicTable,
            DefaultValuesTable,
            UniqueTable,
            CheckConstraintTable,
            ParentTable,
            ForeignKeyTable,
            TableConstraintsTable,
            CustomPrimaryKeyTable,
            CombinedAttributesTable
        ]
    );

    // 辅助函数：获取表的列信息
    fn get_column_info(conn: &SqliteConnection, table_name: &str) -> Vec<(String, String, bool, Option<String>)> {
        conn.query(&format!("PRAGMA table_info({})", table_name), [],
            |row| {
                Ok((
                    row.get::<_, String>(1)?, // 列名
                    row.get::<_, String>(2)?, // 类型
                    row.get::<_, i32>(3)? == 1, // 非空约束
                    row.get::<_, Option<String>>(4)? // 默认值
                ))
            })
            .unwrap()
    }

    #[test]
    fn test_autoincrement_and_not_null() {
        let db = ATTR_TEST_DB::memory().unwrap();
        
        // 创建表
        // db.execute(&BasicTable::create_table_sql(), []).unwrap();
        
        // 检查生成的SQL表结构
        let columns = get_column_info(&db.raw_connection(), "basic_table");
        
        // 验证id列是自增主键
        let id_column = columns.iter().find(|(name, _, _, _)| name == "id").unwrap();
        assert_eq!(id_column.1, "INTEGER"); // INTEGER 类型
        
        // 验证name列是非空的
        let name_column = columns.iter().find(|(name, _, _, _)| name == "name").unwrap();
        assert!(name_column.2); // NOT NULL
        
        // 验证description列是可空的
        let desc_column = columns.iter().find(|(name, _, _, _)| name == "description").unwrap();
        assert!(!desc_column.2); // NULL 允许
        
        // 测试插入操作
        db.execute(
            "INSERT INTO basic_table (name) VALUES (?)",
            params!["Test Name"],
        ).unwrap();
        
        // 验证自增ID工作正常
        let id: i32 = db.query_row(
            "SELECT id FROM basic_table WHERE name = ?",
            params!["Test Name"],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(id, 1);
    }

    #[test]
    fn test_default_values() {
        let db = ATTR_TEST_DB::memory().unwrap();
        
        // 创建表并打印SQL以便调试
        // let sql = DefaultValuesTable::create_table_sql();
        // println!("Default values table SQL: {}", sql);
        // db.execute(&sql, []).unwrap();
        
        // 检查生成的SQL表结构
        let columns = get_column_info(&db.raw_connection(), "default_values_table");
        
        // 验证username有默认值
        let username_column = columns.iter().find(|(name, _, _, _)| name == "username").unwrap();
        assert!(username_column.3.is_some());
        assert_eq!(username_column.3.as_ref().unwrap(), "Guest");
        
        // 验证is_active有默认值为1
        let is_active_column = columns.iter().find(|(name, _, _, _)| name == "is_active").unwrap();
        assert!(is_active_column.3.is_some());
        assert!(is_active_column.3.as_ref().unwrap().contains("1"));
        
        // 测试插入操作 - 只提供name，其他使用默认值
        db.execute(
            "INSERT INTO default_values_table (name) VALUES (?)",
            params!["Test User"],
        ).unwrap();
        
        // 验证默认值被正确应用
        let row = db.query_row(
            "SELECT name, username, is_active, is_deleted, pi_value FROM default_values_table WHERE id = 1",
            [],
            |row| Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, bool>(2)?,
                row.get::<_, bool>(3)?,
                row.get::<_, f64>(4)?
            )),
        ).unwrap();
        
        assert_eq!(row.0, "Test User");
        assert_eq!(row.1, "Guest");       // 默认值
        assert_eq!(row.2, true);          // 默认值1
        assert_eq!(row.3, false);         // 默认值0
        assert_eq!(row.4, 3.14);          // 数字默认值
        
        // 验证日期时间默认值 - 只需检查非空即可，因为时间戳会不同
        let datetime_result = db.query_row(
            "SELECT created_at, timestamp FROM default_values_table WHERE id = 1",
            [],
            |row| Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?
            )),
        ).unwrap();
        
        assert!(!datetime_result.0.is_empty());
        assert!(datetime_result.1 > 0);
    }

    #[test]
    fn test_unique_constraint() {
        let db = ATTR_TEST_DB::memory().unwrap();
        
        // 创建表
        // db.execute(&UniqueTable::create_table_sql(), []).unwrap();
        
        // 测试插入操作
        db.execute(
            "INSERT INTO unique_table (username, email) VALUES (?, ?)",
            params!["user1", "user1@example.com"],
        ).unwrap();
        
        // 尝试插入重复的username，应该失败
        let result = db.execute(
            "INSERT INTO unique_table (username, email) VALUES (?, ?)",
            params!["user1", "different@example.com"],
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("UNIQUE constraint failed"));
    }

    #[test]
    fn test_check_constraint() {
        let db = ATTR_TEST_DB::memory().unwrap();
        
        // 创建表
        // let sql = CheckConstraintTable::create_table_sql();
        // println!("Check constraint table SQL: {}", sql);
        // db.execute(&sql, []).unwrap();
        
        // 测试有效值
        db.execute(
            "INSERT INTO check_constraint_table (name, age, salary) VALUES (?, ?, ?)",
            params!["Valid User", 30, 5000.0],
        ).unwrap();
        
        // 测试无效的年龄（负数）
        let result = db.execute(
            "INSERT INTO check_constraint_table (name, age, salary) VALUES (?, ?, ?)",
            params!["Invalid Age", -1, 1000.0],
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("CHECK constraint failed"));
        
        // 测试无效的薪资（负数）
        let result = db.execute(
            "INSERT INTO check_constraint_table (name, age, salary) VALUES (?, ?, ?)",
            params!["Invalid Salary", 25, -100.0],
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("CHECK constraint failed"));
    }

    #[test]
    fn test_foreign_key() {
        let db = ATTR_TEST_DB::memory().unwrap();
        
        // 启用外键约束
        db.execute("PRAGMA foreign_keys = ON", []).unwrap();
        
        // 创建父表和子表
        // db.execute(&ParentTable::create_table_sql(), []).unwrap();
        // db.execute(&ForeignKeyTable::create_table_sql(), []).unwrap();
        
        // 在父表中插入数据
        db.execute(
            "INSERT INTO parent_table (name) VALUES (?)",
            params!["Parent 1"],
        ).unwrap();
        
        // 在子表中插入有效的外键
        db.execute(
            "INSERT INTO foreign_key_table (name, parent_id) VALUES (?, ?)",
            params!["Child 1", 1],
        ).unwrap();
        
        // 测试无效的外键引用
        let result = db.execute(
            "INSERT INTO foreign_key_table (name, parent_id) VALUES (?, ?)",
            params!["Invalid Child", 999],
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("FOREIGN KEY constraint failed"));
        
        // 测试级联删除
        db.execute("DELETE FROM parent_table WHERE id = 1", []).unwrap();
        
        // 子记录应该也被删除
        let count: i32 = db.query_row(
            "SELECT COUNT(*) FROM foreign_key_table WHERE parent_id = 1",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(count, 0);
    }

    #[test]
    fn test_table_constraints_and_indexes() {
        let db = ATTR_TEST_DB::memory().unwrap();
        
        // 创建表
        // let sql = TableConstraintsTable::create_table_sql();
        // println!("SQL Length: {}", sql.len());
        // println!("Final SQL:\n{}", sql);

        // 检查SQL是否包含索引创建语句
        // assert!(sql.contains("CREATE INDEX IF NOT EXISTS name_idx"), "SQL missing name_idx");
        // assert!(sql.contains("CREATE UNIQUE INDEX IF NOT EXISTS email_idx"), "SQL missing email_idx");

        // 使用 db 直接执行SQL
        // 不需要再调用 raw_connection 然后执行
        
        // 检查是否创建了索引
        let indexes: Vec<String> = db.query(
            "SELECT name FROM sqlite_master WHERE type='index' AND tbl_name='table_constraints_table'",
            [],
            |row| row.get::<_, String>(0)
        ).unwrap();
        
        // 应该有两个索引：name_idx 和 email_idx
        assert!(indexes.contains(&"name_idx".to_string()));
        assert!(indexes.contains(&"email_idx".to_string()));
        
        // 测试名字和姓氏的联合唯一约束
        db.execute(
            "INSERT INTO table_constraints_table (first_name, last_name, email) VALUES (?, ?, ?)",
            params!["John", "Doe", "john@example.com"]
        ).unwrap();
        
        // 不同的email，但相同的名字和姓氏应该失败
        let result = db.execute(
            "INSERT INTO table_constraints_table (first_name, last_name, email) VALUES (?, ?, ?)",
            params!["John", "Doe", "john2@example.com"]
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("UNIQUE constraint failed"));
        
        // 测试email唯一索引
        let result = db.execute(
            "INSERT INTO table_constraints_table (first_name, last_name, email) VALUES (?, ?, ?)",
            params!["Jane", "Doe", "john@example.com"]
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("UNIQUE constraint failed"));
    }

    #[test]
    fn test_custom_primary_key() {
        let db = ATTR_TEST_DB::memory().unwrap();
        
        // 创建表
        // db.execute(&CustomPrimaryKeyTable::create_table_sql(), []).unwrap();
        
        // 插入数据
        db.execute(
            "INSERT INTO custom_primary_key_table (uuid, data) VALUES (?, ?)",
            params!["unique-id-1", "Some data"],
        ).unwrap();
        
        // 尝试插入相同的主键应该失败
        let result = db.execute(
            "INSERT INTO custom_primary_key_table (uuid, data) VALUES (?, ?)",
            params!["unique-id-1", "Different data"],
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("UNIQUE constraint failed"));
        
        // 测试非自增行为 - 不同的主键值应该成功
        db.execute(
            "INSERT INTO custom_primary_key_table (uuid, data) VALUES (?, ?)",
            params!["unique-id-2", "More data"],
        ).unwrap();
        
        // 验证数据已被正确插入
        let count: i32 = db.query_row(
            "SELECT COUNT(*) FROM custom_primary_key_table",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(count, 2);
    }

    #[test]
    fn test_attribute_combination() {
        
        let db = ATTR_TEST_DB::memory().unwrap();
        
        // 创建表
        // let sql = CombinedAttributesTable::create_table_sql();
        // println!("Combined attributes table SQL: {}", sql);
        // db.execute(&sql, []).unwrap();
        
        // 测试多种约束的组合效果
        
        // 1. 成功案例：有效数据
        db.execute(
            "INSERT INTO combined_attributes_table (name, age) VALUES (?, ?)",
            params!["Valid User", 21],
        ).unwrap();
        
        // 2. 违反唯一约束
        let result = db.execute(
            "INSERT INTO combined_attributes_table (name, age) VALUES (?, ?)",
            params!["Valid User", 25],
        );
        assert!(result.is_err());
        
        // 3. 违反检查约束
        let result = db.execute(
            "INSERT INTO combined_attributes_table (name, age) VALUES (?, ?)",
            params!["Young User", 16],
        );
        assert!(result.is_err());
        
        // 4. 验证默认值
        let (is_active, created_at): (bool, String) = db.query_row(
            "SELECT is_active, created_at FROM combined_attributes_table WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).unwrap();
        
        assert!(is_active);  // 默认值为true
        assert!(!created_at.is_empty());  // 创建时间不为空
    }
}