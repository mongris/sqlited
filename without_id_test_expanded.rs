#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
#[cfg(test)]
mod tests {
    use rusqlite::params;
    use sqlited::{
        prelude::*, define_db, table, query, without_id, sql, sql_str, sql_params,
        UtcDateTime,
    };
    pub struct User {
        pub id: i32,
        pub name: String,
        pub age: i32,
        pub email: Option<String>,
        pub created_at: UtcDateTime,
        pub created_at_timestamp: Timestamp,
        pub active: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for User {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "name",
                "age",
                "email",
                "created_at",
                "created_at_timestamp",
                "active",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.name,
                &self.age,
                &self.email,
                &self.created_at,
                &self.created_at_timestamp,
                &&self.active,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "User", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for User {
        #[inline]
        fn clone(&self) -> User {
            User {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                age: ::core::clone::Clone::clone(&self.age),
                email: ::core::clone::Clone::clone(&self.email),
                created_at: ::core::clone::Clone::clone(&self.created_at),
                created_at_timestamp: ::core::clone::Clone::clone(
                    &self.created_at_timestamp,
                ),
                active: ::core::clone::Clone::clone(&self.active),
            }
        }
    }
    impl Default for User {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
                age: <i32>::default(),
                email: <Option<String>>::default(),
                created_at: <UtcDateTime>::default(),
                created_at_timestamp: <Timestamp>::default(),
                active: <bool>::default(),
            }
        }
    }
    impl User {
        /// Create a new instance from a database row
        pub fn from_row(row: &sqlited::rq::Row) -> sqlited::rq::Result<Self> {
            Ok(Self {
                id: row.get::<_, i32>(0usize)?,
                name: row.get::<_, String>(1usize)?,
                age: row.get::<_, i32>(2usize)?,
                email: row.get::<_, Option<String>>(3usize)?,
                created_at: row.get::<_, UtcDateTime>(4usize)?,
                created_at_timestamp: row.get::<_, Timestamp>(5usize)?,
                active: row.get::<_, bool>(6usize)?,
            })
        }
        pub fn from_rows(rows: &[sqlited::rq::Row]) -> sqlited::rq::Result<Vec<Self>> {
            rows.iter().map(Self::from_row).collect()
        }
    }
    impl sqlited::WithoutIdTableInfo for User {
        fn table_name() -> &'static str {
            "user"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    "id",
                    "name",
                    "age",
                    "email",
                    "created_at",
                    "created_at_timestamp",
                    "active",
                ]),
            )
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("age", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    (
                        "email",
                        <Option<String> as sqlited::SqliteTypeName>::sql_type_name(),
                    ),
                    (
                        "created_at",
                        <UtcDateTime as sqlited::SqliteTypeName>::sql_type_name(),
                    ),
                    (
                        "created_at_timestamp",
                        <Timestamp as sqlited::SqliteTypeName>::sql_type_name(),
                    ),
                    ("active", <bool as sqlited::SqliteTypeName>::sql_type_name()),
                ]),
            )
        }
        fn create_table_sql() -> String {
            let mut sql = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!(
                        "CREATE TABLE IF NOT EXISTS {0} (\n",
                        Self::table_name(),
                    ),
                );
                res
            });
            let mut field_constraints = String::new();
            field_constraints.push_str(&" PRIMARY KEY AUTOINCREMENT");
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "id",
                            <i32 as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "name",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "age",
                            <i32 as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            field_constraints.push_str(&" NULL");
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "email",
                            <Option<String> as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            field_constraints.push_str(&" DEFAULT CURRENT_TIMESTAMP");
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "created_at",
                            <UtcDateTime as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            field_constraints.push_str(&" DEFAULT (strftime('%s','now'))");
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "created_at_timestamp",
                            <Timestamp as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            field_constraints.push_str(&" DEFAULT 1");
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "active",
                            <bool as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            if sql.ends_with(",\n") {
                sql.pop();
                sql.pop();
                sql.push_str("\n");
            }
            sql.push_str(")");
            let mut indexes = String::new();
            if !indexes.is_empty() {
                sql.push_str(";\n");
                sql.push_str(&indexes);
            }
            sql
        }
    }
    impl User {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct TestPost {
        pub id: i32,
        pub title: String,
        pub content: String,
        pub published: bool,
        pub user_id: i32,
        pub long_u64: u64,
        pub long_i64: i64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TestPost {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "title",
                "content",
                "published",
                "user_id",
                "long_u64",
                "long_i64",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.title,
                &self.content,
                &self.published,
                &self.user_id,
                &self.long_u64,
                &&self.long_i64,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "TestPost",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TestPost {
        #[inline]
        fn clone(&self) -> TestPost {
            TestPost {
                id: ::core::clone::Clone::clone(&self.id),
                title: ::core::clone::Clone::clone(&self.title),
                content: ::core::clone::Clone::clone(&self.content),
                published: ::core::clone::Clone::clone(&self.published),
                user_id: ::core::clone::Clone::clone(&self.user_id),
                long_u64: ::core::clone::Clone::clone(&self.long_u64),
                long_i64: ::core::clone::Clone::clone(&self.long_i64),
            }
        }
    }
    impl Default for TestPost {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                title: <String>::default(),
                content: <String>::default(),
                published: <bool>::default(),
                user_id: <i32>::default(),
                long_u64: <u64>::default(),
                long_i64: <i64>::default(),
            }
        }
    }
    impl TestPost {
        /// Create a new instance from a database row
        pub fn from_row(row: &sqlited::rq::Row) -> sqlited::rq::Result<Self> {
            Ok(Self {
                id: row.get::<_, i32>(0usize)?,
                title: row.get::<_, String>(1usize)?,
                content: row.get::<_, String>(2usize)?,
                published: row.get::<_, bool>(3usize)?,
                user_id: row.get::<_, i32>(4usize)?,
                long_u64: row.get::<_, u64>(5usize)?,
                long_i64: row.get::<_, i64>(6usize)?,
            })
        }
        pub fn from_rows(rows: &[sqlited::rq::Row]) -> sqlited::rq::Result<Vec<Self>> {
            rows.iter().map(Self::from_row).collect()
        }
    }
    impl sqlited::WithoutIdTableInfo for TestPost {
        fn table_name() -> &'static str {
            "test_post"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    "id",
                    "title",
                    "content",
                    "published",
                    "user_id",
                    "long_u64",
                    "long_i64",
                ]),
            )
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("title", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("content", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("published", <bool as sqlited::SqliteTypeName>::sql_type_name()),
                    ("user_id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("long_u64", <u64 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("long_i64", <i64 as sqlited::SqliteTypeName>::sql_type_name()),
                ]),
            )
        }
        fn create_table_sql() -> String {
            let mut sql = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!(
                        "CREATE TABLE IF NOT EXISTS {0} (\n",
                        Self::table_name(),
                    ),
                );
                res
            });
            let mut field_constraints = String::new();
            field_constraints.push_str(&" PRIMARY KEY AUTOINCREMENT");
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "id",
                            <i32 as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "title",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "content",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "published",
                            <bool as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "user_id",
                            <i32 as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "long_u64",
                            <u64 as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            let mut field_constraints = String::new();
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "long_i64",
                            <i64 as sqlited::SqliteTypeName>::sql_type_name(),
                            field_constraints,
                        ),
                    );
                    res
                }),
            );
            if sql.ends_with(",\n") {
                sql.pop();
                sql.pop();
                sql.push_str("\n");
            }
            sql.push_str(")");
            let mut indexes = String::new();
            if !indexes.is_empty() {
                sql.push_str(";\n");
                sql.push_str(&indexes);
            }
            sql
        }
    }
    impl TestPost {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct TestDb {
        db: Database,
    }
    impl std::ops::Deref for TestDb {
        type Target = Database;
        fn deref(&self) -> &Self::Target {
            &self.db
        }
    }
    pub struct Database {
        conn: ::sqlited::connection::SqliteConnection,
        pool: std::sync::Arc<::sqlited::pool::ConnectionPool>,
    }
    impl std::ops::Deref for Database {
        type Target = ::sqlited::connection::SqliteConnection;
        fn deref(&self) -> &Self::Target {
            &self.conn
        }
    }
    impl Database {
        fn new(
            conn: ::sqlited::connection::SqliteConnection,
            pool: std::sync::Arc<::sqlited::pool::ConnectionPool>,
        ) -> Self {
            Self { conn, pool }
        }
        pub fn raw_connection(&self) -> &::sqlited::connection::SqliteConnection {
            &self.conn
        }
        fn get_all_table_migrations() -> Vec<(String, String, Option<String>)> {
            let mut migrations = Vec::new();
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| User::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| TestPost::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = None::<
                Vec<(String, String, Option<String>)>,
            > {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = None::<
                Vec<(String, String, Option<String>)>,
            > {
                migrations.extend(table_migrations);
            }
            migrations
        }
        fn get_migrations() -> Vec<String> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    User::create_table_sql().to_string(),
                    TestPost::create_table_sql().to_string(),
                    "CREATE TABLE IF NOT EXISTS constrained_user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                email TEXT UNIQUE,
                age INTEGER NOT NULL CHECK(age >= 18)
            )"
                        .to_string(),
                    "CREATE TABLE IF NOT EXISTS binary_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                data BLOB NOT NULL,
                metadata TEXT
            )"
                        .to_string(),
                ]),
            )
        }
        pub fn apply_migrations(&self) -> ::sqlited::error::Result<()> {
            self.conn
                .execute(
                    "CREATE TABLE IF NOT EXISTS _sqlited_migrations (
                        id INTEGER PRIMARY KEY,
                        name TEXT NOT NULL,
                        applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
                    )",
                    [],
                )?;
            let table_migrations = Self::get_all_table_migrations();
            self.conn.execute("BEGIN TRANSACTION", [])?;
            let mut success = true;
            for (name, up_sql, _) in table_migrations {
                if name.starts_with("error") {
                    {
                        ::std::io::_print(
                            format_args!("Skipping invalid migration: {0}\n", up_sql),
                        );
                    };
                    continue;
                }
                let already_applied = self
                    .conn
                    .query_row(
                        "SELECT COUNT(*) FROM _sqlited_migrations WHERE name = ?",
                        [&name],
                        |row| row.get::<_, i32>(0),
                    )
                    .unwrap_or(0) > 0;
                if !already_applied {
                    let statements = up_sql
                        .split(';')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>();
                    for statement in &statements {
                        match self.conn.execute(statement, []) {
                            Ok(_) => {}
                            Err(e) => {
                                {
                                    ::std::io::_eprint(
                                        format_args!(
                                            "Failed to apply migration {0}: {1}\n",
                                            name,
                                            e,
                                        ),
                                    );
                                };
                                success = false;
                                break;
                            }
                        }
                    }
                    if success {
                        if let Err(e) = self
                            .conn
                            .execute(
                                "INSERT INTO _sqlited_migrations (name) VALUES (?)",
                                [&name],
                            )
                        {
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "Failed to record migration {0}: {1}\n",
                                        name,
                                        e,
                                    ),
                                );
                            };
                            success = false;
                        }
                    }
                }
                if !success {
                    break;
                }
            }
            if success {
                for migration in Self::get_migrations() {
                    let statements = migration
                        .split(';')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>();
                    for statement in &statements {
                        if statement.is_empty() {
                            continue;
                        }
                        let statement_hash = ::sqlited::macros::get_statement_key(
                            statement,
                        );
                        let count = self
                            .conn
                            .query_row(
                                "SELECT COUNT(*) FROM _sqlited_migrations WHERE name = ?",
                                [&statement_hash],
                                |row| row.get::<_, i32>(0),
                            )
                            .unwrap_or(0);
                        let already_applied = count > 0;
                        if !already_applied {
                            match self.conn.execute(statement, []) {
                                Ok(_) => {
                                    if let Err(e) = self
                                        .conn
                                        .execute(
                                            "INSERT INTO _sqlited_migrations (name) VALUES (?)",
                                            [&statement_hash],
                                        )
                                    {
                                        {
                                            ::std::io::_eprint(
                                                format_args!("Failed to record migration: {0}\n", e),
                                            );
                                        };
                                        success = false;
                                        break;
                                    }
                                }
                                Err(e) => {
                                    {
                                        ::std::io::_eprint(
                                            format_args!("Failed to apply migration: {0}\n", e),
                                        );
                                    };
                                    success = false;
                                    break;
                                }
                            }
                        }
                    }
                    if !success {
                        break;
                    }
                }
            }
            if success {
                self.conn.execute("COMMIT", [])?;
            } else {
                let _ = self.conn.execute("ROLLBACK", []);
                return Err(
                    ::sqlited::error::SqlitedError::Rusqlite(
                        ::sqlited::rq::Error::SqliteFailure(
                            ::sqlited::rq::ffi::Error {
                                code: ::sqlited::rq::ffi::ErrorCode::InternalMalfunction,
                                extended_code: 1,
                            },
                            Some("Failed to apply migrations".to_string()),
                        ),
                    ),
                );
            }
            Ok(())
        }
        /// 返回一个新的连接到同一数据库
        pub fn new_connection(&self) -> ::sqlited::error::Result<Self> {
            let conn = ::sqlited::connection::get_connection(&self.pool)?;
            Ok(Self::new(conn, self.pool.clone()))
        }
        /// 在事务中执行闭包，自动处理提交和回滚
        pub fn transaction<T, F>(&self, f: F) -> ::sqlited::error::Result<T>
        where
            F: FnOnce(&Self) -> ::sqlited::error::Result<T>,
        {
            self.conn.execute("BEGIN TRANSACTION", [])?;
            match f(self) {
                Ok(result) => {
                    self.conn.execute("COMMIT", [])?;
                    Ok(result)
                }
                Err(e) => {
                    let _ = self.conn.execute("ROLLBACK", []);
                    Err(e)
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    impl TestDb {
        /// 打开给定路径的数据库（如果为None则使用内存模式）
        fn _open(
            path: Option<impl AsRef<std::path::Path>>,
        ) -> ::sqlited::error::Result<Self> {
            match path {
                Some(path) => {
                    let path_buf = path.as_ref().to_path_buf();
                    let canonical_path = if path_buf.exists() {
                        std::fs::canonicalize(&path_buf).unwrap_or(path_buf)
                    } else {
                        if let Some(parent) = path_buf.parent() {
                            std::fs::create_dir_all(parent)
                                .map_err(|e| ::sqlited::error::SqlitedError::Rusqlite(
                                    ::sqlited::rq::Error::SqliteFailure(
                                        ::sqlited::rq::ffi::Error {
                                            code: ::sqlited::rq::ffi::ErrorCode::CannotOpen,
                                            extended_code: 1,
                                        },
                                        Some(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("Failed to create database directory: {0}", e),
                                                );
                                                res
                                            }),
                                        ),
                                    ),
                                ))?;
                        }
                        path_buf
                    };
                    let pool = {
                        let mut pools = ::sqlited::CONNECTION_POOLS.lock().unwrap();
                        if let Some(existing_pool) = pools.get(&canonical_path) {
                            existing_pool.clone()
                        } else {
                            let pool = ::sqlited::pool::ConnectionPool::new(
                                    &canonical_path,
                                )
                                .map(std::sync::Arc::new)
                                .map_err(|pool_err| ::sqlited::error::SqlitedError::Rusqlite(
                                    ::sqlited::rq::Error::SqliteFailure(
                                        ::sqlited::rq::ffi::Error {
                                            code: ::sqlited::rq::ffi::ErrorCode::InternalMalfunction,
                                            extended_code: 1,
                                        },
                                        Some(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("Pool creation error: {0}", pool_err),
                                                );
                                                res
                                            }),
                                        ),
                                    ),
                                ))?;
                            pools.insert(canonical_path, pool.clone());
                            pool
                        }
                    };
                    let conn = ::sqlited::connection::get_connection(&pool)?;
                    let db = Database::new(conn, pool);
                    db.apply_migrations()?;
                    Ok(Self { db })
                }
                None => {
                    let pool = ::sqlited::pool::ConnectionPool::new_memory()
                        .map(std::sync::Arc::new)
                        .map_err(|pool_err| ::sqlited::error::SqlitedError::Rusqlite(
                            ::sqlited::rq::Error::SqliteFailure(
                                ::sqlited::rq::ffi::Error {
                                    code: ::sqlited::rq::ffi::ErrorCode::InternalMalfunction,
                                    extended_code: 1,
                                },
                                Some(
                                    ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("Memory pool error: {0}", pool_err),
                                        );
                                        res
                                    }),
                                ),
                            ),
                        ))?;
                    let conn = ::sqlited::connection::get_connection(&pool)?;
                    let db = Database::new(conn, pool);
                    db.apply_migrations()?;
                    Ok(Self { db })
                }
            }
        }
        /// 打开指定路径的数据库
        pub fn open(path: impl AsRef<std::path::Path>) -> ::sqlited::Result<Self> {
            Self::_open(Some(path))
        }
        /// 打开内存数据库
        pub fn memory() -> ::sqlited::Result<Self> {
            Self::_open(None::<&std::path::Path>)
        }
        /// 在临时位置创建数据库
        pub fn temp() -> ::sqlited::Result<Self> {
            let temp_dir = std::env::temp_dir();
            let db_file = temp_dir
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("sqlited_{0}.db", uuid::Uuid::new_v4()),
                        );
                        res
                    }),
                );
            Self::_open(Some(db_file))
        }
        /// 打开共享内存数据库（使用命名内存数据库）
        pub fn shared_memory(name: &str) -> ::sqlited::Result<Self> {
            let memory_path = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!("file:{0}?mode=memory&cache=shared", name),
                );
                res
            });
            Self::_open(Some(memory_path))
        }
        /// 获取一个到同一数据库的新连接
        pub fn new_connection(&self) -> ::sqlited::error::Result<Self> {
            let new_db = self.db.new_connection()?;
            Ok(Self { db: new_db })
        }
        /// 在事务中执行闭包，自动处理提交和回滚
        pub fn transaction<T, F>(&self, f: F) -> ::sqlited::error::Result<T>
        where
            F: FnOnce(&Self) -> ::sqlited::error::Result<T>,
        {
            self.conn.execute("BEGIN TRANSACTION", [])?;
            match f(self) {
                Ok(result) => {
                    self.conn.execute("COMMIT", [])?;
                    Ok(result)
                }
                Err(e) => {
                    let _ = self.conn.execute("ROLLBACK", []);
                    Err(e)
                }
            }
        }
    }
    #[allow(non_upper_case_globals)]
    #[allow(non_camel_case_types)]
    pub type TEST_DB = TestDb;
    impl TestDb {
        fn get_user_by_name(&self, name: String) -> sqlited::Result<User> {
            let query = "SELECT\n  *\nFROM\n  user\nWHERE\n  name = ?;";
            self.query_row(
                query,
                &[&name as &dyn ::rusqlite::ToSql] as &[&dyn ::rusqlite::ToSql],
                User::from_row,
            )
        }
        fn get_user_by_id(&self, id: i32) -> sqlited::Result<User> {
            let query = "SELECT\n  *\nFROM\n  user\nWHERE\n  id = ?\nLIMIT\n  1;";
            self.query_row(
                query,
                &[&id as &dyn ::rusqlite::ToSql] as &[&dyn ::rusqlite::ToSql],
                User::from_row,
            )
        }
        fn get_users_by_age(&self, age: i32) -> sqlited::Result<Vec<User>> {
            let query = "SELECT\n  *\nFROM\n  user\nWHERE\n  age > ?1;";
            self.query(
                query,
                &[&age as &dyn ::rusqlite::ToSql] as &[&dyn ::rusqlite::ToSql],
                User::from_row,
            )
        }
        fn get_next_user_id(&self) -> sqlited::Result<i32> {
            let query = "SELECT\n  id\nFROM\n  user\nORDER BY\n  id DESC\nLIMIT\n  1;";
            self.query_row(query, &[] as &[&dyn ::rusqlite::ToSql], |row| row.get(0))
        }
        fn get_some_info_by_id(&self, id: i32) -> sqlited::Result<(i32, String)> {
            let query = "SELECT\n  id;";
            self.query_row(
                query,
                &[&id as &dyn ::rusqlite::ToSql] as &[&dyn ::rusqlite::ToSql],
                |row| { Ok((row.get(0usize)?, row.get(1usize)?)) },
            )
        }
        pub fn get_user_by_name2(&self, name: String) -> sqlited::Result<User> {
            let params = {
                let _temp_name = std::rc::Rc::new(name);
                #[allow(unused_variables, unreachable_code, unused_must_use)]
                {
                    if false {
                        let mut _model = <User>::default();
                        _model.name = (*_temp_name).clone().into();
                    }
                }
                let mut result = sqlited::WithoutId::<User>::new();
                {
                    let value = (*_temp_name).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("name".to_lowercase(), boxed_value);
                }
                let ordered_field_names = <[_]>::into_vec(
                    ::alloc::boxed::box_new(["name".to_string()]),
                );
                result.create_static_params_for_fields(&ordered_field_names)
            };
            let query = {
                sqlited::SqlQuery {
                    query: "SELECT\n  *\nFROM\n  user\nWHERE\n  name = ?1;".to_string(),
                    params: (&params).to_boxed_vec(),
                }
            };
            query.query_row(self.raw_connection(), User::from_row)
        }
        pub fn get_user_by_age2(&self, age2: i32) -> sqlited::Result<User> {
            let query = "SELECT\n  *\nFROM\n  user\nWHERE\n  age > ?;";
            self.query_row(
                query,
                &[&age2 as &dyn ::rusqlite::ToSql] as &[&dyn ::rusqlite::ToSql],
                User::from_row,
            )
        }
        fn get_published_posts_by_user(
            &self,
            user_id: i32,
        ) -> sqlited::Result<Vec<TestPost>> {
            let query = "SELECT\n  *\nFROM\n  test_post\nWHERE\n  user_id = ?\n  AND published = 1;";
            self.query(
                query,
                &[&user_id as &dyn ::rusqlite::ToSql] as &[&dyn ::rusqlite::ToSql],
                TestPost::from_row,
            )
        }
        pub fn get_published_posts_by_user2(
            &self,
            user_id: i32,
        ) -> sqlited::Result<Vec<TestPost>> {
            let query = "SELECT\n  *\nFROM\n  test_post\nWHERE\n  user_id = ?\n  AND published = 1;";
            self.query(
                query,
                &[&user_id as &dyn ::rusqlite::ToSql] as &[&dyn ::rusqlite::ToSql],
                TestPost::from_row,
            )
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_without_id_macro"]
    #[doc(hidden)]
    pub const test_without_id_macro: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_without_id_macro"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 130usize,
            start_col: 8usize,
            end_line: 130usize,
            end_col: 29usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_without_id_macro()),
        ),
    };
    fn test_without_id_macro() {
        let user_data = {
            let mut result = ::sqlited::WithoutId::<User>::new();
            result.set("name", "John Doe");
            result.set("age", 30);
            result.set("email", Some("john@example.com"));
            result
        };
        match (&user_data.inner.len(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        if !user_data.inner.contains_key("name") {
            ::core::panicking::panic(
                "assertion failed: user_data.inner.contains_key(\"name\")",
            )
        }
        if !user_data.inner.contains_key("age") {
            ::core::panicking::panic(
                "assertion failed: user_data.inner.contains_key(\"age\")",
            )
        }
        if !user_data.inner.contains_key("email") {
            ::core::panicking::panic(
                "assertion failed: user_data.inner.contains_key(\"email\")",
            )
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_for_insert_macro"]
    #[doc(hidden)]
    pub const test_for_insert_macro: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_for_insert_macro"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 146usize,
            start_col: 8usize,
            end_line: 146usize,
            end_col: 29usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_for_insert_macro()),
        ),
    };
    fn test_for_insert_macro() {
        let db = TEST_DB::memory().unwrap();
        let params = {
            let _temp_name = std::rc::Rc::new("Jane Smith".to_string());
            let _temp_age = std::rc::Rc::new(28);
            let _temp_email = std::rc::Rc::new(Some("jane@example.com".to_string()));
            #[allow(unused_variables, unreachable_code, unused_must_use)]
            {
                if false {
                    let mut _model = <User>::default();
                    _model.name = (*_temp_name).clone().into();
                    _model.age = (*_temp_age).clone().into();
                    _model.email = (*_temp_email).clone().into();
                }
            }
            let mut result = sqlited::WithoutId::<User>::new();
            {
                let value = (*_temp_name).clone();
                let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                result.inner.insert("name".to_lowercase(), boxed_value);
            }
            {
                let value = (*_temp_age).clone();
                let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                result.inner.insert("age".to_lowercase(), boxed_value);
            }
            {
                let value = (*_temp_email).clone();
                let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                result.inner.insert("email".to_lowercase(), boxed_value);
            }
            let ordered_field_names = <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    "name".to_string(),
                    "age".to_string(),
                    "email".to_string(),
                ]),
            );
            result.create_static_params_for_fields(&ordered_field_names)
        };
        let query = {
            sqlited::SqlQuery {
                query: "INSERT INTO\n  user(name, age, email)\nVALUES\n(?, ?, ?);"
                    .to_string(),
                params: (&params).to_boxed_vec(),
            }
        };
        {
            ::std::io::_eprint(
                format_args!("SQL2: {0}, params count: {1}\n", query.query, params.len()),
            );
        };
        let result = query.execute(&db);
        if !result.is_ok() {
            ::core::panicking::panic("assertion failed: result.is_ok()")
        }
        match (&result.unwrap(), &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let row_data = &db
            .query(
                "SELECT name, age, email FROM user WHERE rowid = 1",
                [],
                |row| Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i32>(1)?,
                    row.get::<_, Option<String>>(2)?,
                )),
            )
            .unwrap()[0];
        let (name, age, email) = row_data;
        match (&name, &"Jane Smith") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&*age, &28) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&email, &&Some("jane@example.com".to_string())) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let user = &db.get_user_by_name("Jane Smith".to_string()).unwrap();
        match (&user.name, &"Jane Smith") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&user.age, &28) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&email, &&Some("jane@example.com".to_string())) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_user_crud_operations"]
    #[doc(hidden)]
    pub const test_user_crud_operations: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_user_crud_operations"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 190usize,
            start_col: 8usize,
            end_line: 190usize,
            end_col: 33usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_user_crud_operations()),
        ),
    };
    fn test_user_crud_operations() {
        let db = TEST_DB::memory().unwrap();
        let raw_conn = db.raw_connection();
        let user_data = {
            let _temp_name = std::rc::Rc::new("Alex Johnson".to_string());
            let _temp_age = std::rc::Rc::new(35);
            let _temp_email = std::rc::Rc::new(Some("alex@example.com".to_string()));
            #[allow(unused_variables, unreachable_code, unused_must_use)]
            {
                if false {
                    let mut _model = <User>::default();
                    _model.name = (*_temp_name).clone().into();
                    _model.age = (*_temp_age).clone().into();
                    _model.email = (*_temp_email).clone().into();
                }
            }
            let mut result = sqlited::WithoutId::<User>::new();
            {
                let value = (*_temp_name).clone();
                let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                result.inner.insert("name".to_lowercase(), boxed_value);
            }
            {
                let value = (*_temp_age).clone();
                let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                result.inner.insert("age".to_lowercase(), boxed_value);
            }
            {
                let value = (*_temp_email).clone();
                let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                result.inner.insert("email".to_lowercase(), boxed_value);
            }
            let ordered_field_names = <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    "name".to_string(),
                    "age".to_string(),
                    "email".to_string(),
                ]),
            );
            result.create_static_params_for_fields(&ordered_field_names)
        };
        let query = User::insert_with(&["name", "age", "email"]);
        db.execute(&query, &*user_data).unwrap();
        let user_id: i32 = raw_conn.last_insert_rowid() as i32;
        let user_query = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!(
                    "SELECT id, name, age, email, created_at, created_at_timestamp, active FROM user WHERE id = {0}",
                    user_id,
                ),
            );
            res
        });
        let user_data = &db
            .query(
                &user_query,
                [],
                |row| Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i32>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, UtcDateTime>(4)?,
                    row.get::<_, Timestamp>(5)?,
                    row.get::<_, bool>(6)?,
                )),
            )
            .unwrap()[0];
        {
            ::std::io::_eprint(format_args!("User data: {0:?}\n", user_data));
        };
        let (db_user_id, name, age, email, created_at, created_at_timestamp, active) = user_data;
        {
            ::std::io::_eprint(format_args!("created at {0:?}\n", created_at));
        };
        {
            ::std::io::_eprint(
                format_args!("created at timestamp {0:?}\n", created_at_timestamp),
            );
        };
        match (&*db_user_id, &user_id) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&name, &"Alex Johnson") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&*age, &35) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&email, &&Some("alex@example.com".to_string())) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&*active, &true) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        db.execute(
                "UPDATE user SET name = ?, age = ? WHERE id = ?",
                &[
                    &&"Alex Smith" as &dyn ::rusqlite::ToSql,
                    &&40 as &dyn ::rusqlite::ToSql,
                    &&user_id as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            )
            .unwrap();
        let updated_data = &db
            .query(
                "SELECT name, age FROM user WHERE id = ?",
                &[&user_id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?)),
            )
            .unwrap()[0];
        let (_updated_name, _updated_age) = updated_data;
        db.execute("DELETE FROM user WHERE id = ?", &[&user_id]).unwrap();
        let count = db
            .query(
                "SELECT COUNT(*) FROM user WHERE id = ?",
                &[&user_id],
                |row| row.get::<_, i32>(0),
            )
            .unwrap()[0];
        match (&count, &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_post_with_foreign_key"]
    #[doc(hidden)]
    pub const test_post_with_foreign_key: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_post_with_foreign_key"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 256usize,
            start_col: 8usize,
            end_line: 256usize,
            end_col: 34usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_post_with_foreign_key()),
        ),
    };
    fn test_post_with_foreign_key() {
        let db = TEST_DB::memory().unwrap();
        let raw_conn = db.raw_connection();
        let user_data = {
            let _temp_name = std::rc::Rc::new("Blog Writer".to_string());
            let _temp_age = std::rc::Rc::new(28);
            let _temp_email = std::rc::Rc::new(Some("writer@blog.com".to_string()));
            #[allow(unused_variables, unreachable_code, unused_must_use)]
            {
                if false {
                    let mut _model = <User>::default();
                    _model.name = (*_temp_name).clone().into();
                    _model.age = (*_temp_age).clone().into();
                    _model.email = (*_temp_email).clone().into();
                }
            }
            let mut result = sqlited::WithoutId::<User>::new();
            {
                let value = (*_temp_name).clone();
                let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                result.inner.insert("name".to_lowercase(), boxed_value);
            }
            {
                let value = (*_temp_age).clone();
                let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                result.inner.insert("age".to_lowercase(), boxed_value);
            }
            {
                let value = (*_temp_email).clone();
                let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                result.inner.insert("email".to_lowercase(), boxed_value);
            }
            let ordered_field_names = <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    "name".to_string(),
                    "age".to_string(),
                    "email".to_string(),
                ]),
            );
            result.create_static_params_for_fields(&ordered_field_names)
        };
        db.execute(&User::insert_with(&["name", "age", "email"]), &*user_data).unwrap();
        let user_id: i32 = raw_conn.last_insert_rowid() as i32;
        let posts = [
            ("First Post", "This is my first blog post", true),
            ("Draft Post", "This is an unpublished draft", false),
            ("Tech Review", "Review of latest technology", true),
        ];
        for (title, content, published) in posts.iter() {
            let post_data = {
                let _temp_title = std::rc::Rc::new(title.to_string());
                let _temp_content = std::rc::Rc::new(content.to_string());
                let _temp_published = std::rc::Rc::new(*published);
                let _temp_user_id = std::rc::Rc::new(user_id);
                let _temp_long_u64 = std::rc::Rc::new(9223372036854775807u64);
                let _temp_long_i64 = std::rc::Rc::new(1234567890123456789i64);
                #[allow(unused_variables, unreachable_code, unused_must_use)]
                {
                    if false {
                        let mut _model = <TestPost>::default();
                        _model.title = (*_temp_title).clone().into();
                        _model.content = (*_temp_content).clone().into();
                        _model.published = (*_temp_published).clone().into();
                        _model.user_id = (*_temp_user_id).clone().into();
                        _model.long_u64 = (*_temp_long_u64).clone().into();
                        _model.long_i64 = (*_temp_long_i64).clone().into();
                    }
                }
                let mut result = sqlited::WithoutId::<TestPost>::new();
                {
                    let value = (*_temp_title).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("title".to_lowercase(), boxed_value);
                }
                {
                    let value = (*_temp_content).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("content".to_lowercase(), boxed_value);
                }
                {
                    let value = (*_temp_published).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("published".to_lowercase(), boxed_value);
                }
                {
                    let value = (*_temp_user_id).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("user_id".to_lowercase(), boxed_value);
                }
                {
                    let value = (*_temp_long_u64).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("long_u64".to_lowercase(), boxed_value);
                }
                {
                    let value = (*_temp_long_i64).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("long_i64".to_lowercase(), boxed_value);
                }
                let ordered_field_names = <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        "title".to_string(),
                        "content".to_string(),
                        "published".to_string(),
                        "user_id".to_string(),
                        "long_u64".to_string(),
                        "long_i64".to_string(),
                    ]),
                );
                result.create_static_params_for_fields(&ordered_field_names)
            };
            db.execute(&&TestPost::insert_without_id(), &*post_data).unwrap();
        }
        let published_posts = db
            .query(
                "SELECT id, title, long_u64 FROM test_post WHERE published = ? AND user_id = ? ORDER BY id",
                &[&&true as &dyn ::rusqlite::ToSql, &&user_id as &dyn ::rusqlite::ToSql]
                    as &[&dyn ::rusqlite::ToSql],
                |row| Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, u64>(2)?,
                )),
            )
            .unwrap();
        match (&published_posts.len(), &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&published_posts[0].1, &"First Post") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&published_posts[1].1, &"Tech Review") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&published_posts[0].2, &9223372036854775807u64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let published_posts = db.get_published_posts_by_user(user_id).unwrap();
        match (&published_posts.len(), &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&published_posts[0].title, &"First Post") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&published_posts[1].title, &"Tech Review") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&published_posts[0].long_u64, &9223372036854775807u64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let first_post_id = published_posts[0].id;
        db.execute(
                "UPDATE test_post SET title = ? WHERE id = ?",
                &[
                    &&"Updated First Post" as &dyn ::rusqlite::ToSql,
                    &&first_post_id as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            )
            .unwrap();
        let updated_title = &db
            .query(
                "SELECT title FROM test_post WHERE id = ?",
                &[&first_post_id],
                |row| row.get::<_, String>(0),
            )
            .unwrap()[0];
        match (&*updated_title, &"Updated First Post") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        db.execute("DELETE FROM test_post WHERE user_id = ?", &[&user_id]).unwrap();
        let post_count = db
            .query(
                "SELECT COUNT(*) FROM test_post WHERE user_id = ?",
                &[&user_id],
                |row| row.get::<_, i32>(0),
            )
            .unwrap()[0];
        match (&post_count, &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_transaction_and_rollback"]
    #[doc(hidden)]
    pub const test_transaction_and_rollback: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_transaction_and_rollback"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 349usize,
            start_col: 8usize,
            end_line: 349usize,
            end_col: 37usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_transaction_and_rollback()),
        ),
    };
    fn test_transaction_and_rollback() {
        let db = TEST_DB::memory().unwrap();
        db.transaction(|tx| {
                let users = [
                    ("Transaction User 1", 25, Some("tx1@example.com")),
                    ("Transaction User 2", 30, Some("tx2@example.com")),
                ];
                for (name, age, email) in users.iter() {
                    let user_data = {
                        let _temp_name = std::rc::Rc::new(name.to_string());
                        let _temp_age = std::rc::Rc::new(*age);
                        let _temp_email = std::rc::Rc::new(email.map(|e| e.to_string()));
                        #[allow(unused_variables, unreachable_code, unused_must_use)]
                        {
                            if false {
                                let mut _model = <User>::default();
                                _model.name = (*_temp_name).clone().into();
                                _model.age = (*_temp_age).clone().into();
                                _model.email = (*_temp_email).clone().into();
                            }
                        }
                        let mut result = sqlited::WithoutId::<User>::new();
                        {
                            let value = (*_temp_name).clone();
                            let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(
                                value,
                            );
                            result.inner.insert("name".to_lowercase(), boxed_value);
                        }
                        {
                            let value = (*_temp_age).clone();
                            let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(
                                value,
                            );
                            result.inner.insert("age".to_lowercase(), boxed_value);
                        }
                        {
                            let value = (*_temp_email).clone();
                            let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(
                                value,
                            );
                            result.inner.insert("email".to_lowercase(), boxed_value);
                        }
                        let ordered_field_names = <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                "name".to_string(),
                                "age".to_string(),
                                "email".to_string(),
                            ]),
                        );
                        result.create_static_params_for_fields(&ordered_field_names)
                    };
                    tx.execute(
                        &User::insert_with(&["name", "age", "email"]),
                        &*user_data,
                    )?;
                }
                let count: i32 = tx
                    .query_row("SELECT COUNT(*) FROM user", [], |row| row.get(0))?;
                match (&count, &2) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                let users = &db.get_users_by_age(2).unwrap();
                match (&users.len(), &2) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                Err::<
                    (),
                    _,
                >(sqlited::SqlitedError::from(rusqlite::Error::StatementChangedRows(0)))
            })
            .unwrap_err();
        let count = db
            .query("SELECT COUNT(*) FROM user", [], |row| row.get::<_, i32>(0))
            .unwrap()[0];
        match (&count, &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("事务回滚后用户表应该为空"),
                        ),
                    );
                }
            }
        };
        db.transaction(|tx| {
                let user_data = {
                    let _temp_name = std::rc::Rc::new("Committed User".to_string());
                    let _temp_age = std::rc::Rc::new(40);
                    let _temp_email = std::rc::Rc::new(
                        Some("committed@example.com".to_string()),
                    );
                    #[allow(unused_variables, unreachable_code, unused_must_use)]
                    {
                        if false {
                            let mut _model = <User>::default();
                            _model.name = (*_temp_name).clone().into();
                            _model.age = (*_temp_age).clone().into();
                            _model.email = (*_temp_email).clone().into();
                        }
                    }
                    let mut result = sqlited::WithoutId::<User>::new();
                    {
                        let value = (*_temp_name).clone();
                        let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                        result.inner.insert("name".to_lowercase(), boxed_value);
                    }
                    {
                        let value = (*_temp_age).clone();
                        let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                        result.inner.insert("age".to_lowercase(), boxed_value);
                    }
                    {
                        let value = (*_temp_email).clone();
                        let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                        result.inner.insert("email".to_lowercase(), boxed_value);
                    }
                    let ordered_field_names = <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            "name".to_string(),
                            "age".to_string(),
                            "email".to_string(),
                        ]),
                    );
                    result.create_static_params_for_fields(&ordered_field_names)
                };
                tx.execute(&User::insert_with(&["name", "age", "email"]), &*user_data)?;
                Ok(())
            })
            .unwrap();
        let count = db
            .query("SELECT COUNT(*) FROM user", [], |row| row.get::<_, i32>(0))
            .unwrap()[0];
        match (&count, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("提交事务后应该有一个用户"),
                        ),
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_data_validation_and_integrity"]
    #[doc(hidden)]
    pub const test_data_validation_and_integrity: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_data_validation_and_integrity"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 424usize,
            start_col: 8usize,
            end_line: 424usize,
            end_col: 42usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_data_validation_and_integrity()),
        ),
    };
    fn test_data_validation_and_integrity() {
        let db = TEST_DB::memory().unwrap();
        let raw_conn = db.raw_connection();
        let test_cases = [
            ("Empty Name", "", 0, None),
            ("Max Integer", "Max Value", i32::MAX, Some("max@example.com")),
            ("Min Integer", "Min Value", i32::MIN, Some("min@example.com")),
            (
                "Special Chars",
                "O'Neil; DROP TABLE users;--",
                25,
                Some("sql\"injection'test@example.com"),
            ),
            ("Unicode Test", "测试用户 👨‍💻", 30, Some("unicode@测试.com")),
            ("Very Long Name", &"A".repeat(1000), 42, Some("long@example.com")),
        ];
        for (test_name, name, age, email) in test_cases.iter() {
            {
                ::std::io::_print(format_args!("Testing: {0}\n", test_name));
            };
            let user_data = {
                let _temp_name = std::rc::Rc::new(name.to_string());
                let _temp_age = std::rc::Rc::new(*age);
                let _temp_email = std::rc::Rc::new(email.map(|e| e.to_string()));
                #[allow(unused_variables, unreachable_code, unused_must_use)]
                {
                    if false {
                        let mut _model = <User>::default();
                        _model.name = (*_temp_name).clone().into();
                        _model.age = (*_temp_age).clone().into();
                        _model.email = (*_temp_email).clone().into();
                    }
                }
                let mut result = sqlited::WithoutId::<User>::new();
                {
                    let value = (*_temp_name).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("name".to_lowercase(), boxed_value);
                }
                {
                    let value = (*_temp_age).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("age".to_lowercase(), boxed_value);
                }
                {
                    let value = (*_temp_email).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("email".to_lowercase(), boxed_value);
                }
                let ordered_field_names = <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        "name".to_string(),
                        "age".to_string(),
                        "email".to_string(),
                    ]),
                );
                result.create_static_params_for_fields(&ordered_field_names)
            };
            db.execute(&User::insert_with(&["name", "age", "email"]), &*user_data)
                .unwrap();
            let user_id: i32 = raw_conn.last_insert_rowid() as i32;
            let row_data = &db
                .query(
                    "SELECT name, age, email FROM user WHERE id = ?",
                    &[&user_id],
                    |row| Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i32>(1)?,
                        row.get::<_, Option<String>>(2)?,
                    )),
                )
                .unwrap()[0];
            let (db_name, db_age, db_email) = row_data;
            match (&*db_name, &*name) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(
                                format_args!("Name should match exactly for {0}", test_name),
                            ),
                        );
                    }
                }
            };
            match (&*db_age, &*age) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(
                                format_args!("Age should match exactly for {0}", test_name),
                            ),
                        );
                    }
                }
            };
            match (&db_email, &&email.map(|e| e.to_string())) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(
                                format_args!("Email should match for {0}", test_name),
                            ),
                        );
                    }
                }
            };
        }
        db.execute("DELETE FROM user", []).unwrap();
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_advanced_queries_and_aggregations"]
    #[doc(hidden)]
    pub const test_advanced_queries_and_aggregations: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_advanced_queries_and_aggregations"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 473usize,
            start_col: 8usize,
            end_line: 473usize,
            end_col: 46usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_advanced_queries_and_aggregations()),
        ),
    };
    fn test_advanced_queries_and_aggregations() {
        let db = TEST_DB::memory().unwrap();
        let raw_conn = db.raw_connection();
        let users = [
            ("Alice", 25, Some("alice@example.com")),
            ("Bob", 30, Some("bob@example.com")),
            ("Charlie", 35, Some("charlie@example.com")),
            ("David", 25, None),
            ("Eve", 40, Some("eve@example.com")),
        ];
        let mut user_ids = Vec::new();
        for (name, age, email) in users.iter() {
            let user_data = {
                let _temp_name = std::rc::Rc::new(name.to_string());
                let _temp_age = std::rc::Rc::new(*age);
                let _temp_email = std::rc::Rc::new(email.map(|e| e.to_string()));
                #[allow(unused_variables, unreachable_code, unused_must_use)]
                {
                    if false {
                        let mut _model = <User>::default();
                        _model.name = (*_temp_name).clone().into();
                        _model.age = (*_temp_age).clone().into();
                        _model.email = (*_temp_email).clone().into();
                    }
                }
                let mut result = sqlited::WithoutId::<User>::new();
                {
                    let value = (*_temp_name).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("name".to_lowercase(), boxed_value);
                }
                {
                    let value = (*_temp_age).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("age".to_lowercase(), boxed_value);
                }
                {
                    let value = (*_temp_email).clone();
                    let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                    result.inner.insert("email".to_lowercase(), boxed_value);
                }
                let ordered_field_names = <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        "name".to_string(),
                        "age".to_string(),
                        "email".to_string(),
                    ]),
                );
                result.create_static_params_for_fields(&ordered_field_names)
            };
            db.execute(&User::insert_with(&["name", "age", "email"]), &*user_data)
                .unwrap();
            user_ids.push(raw_conn.last_insert_rowid() as i32);
        }
        for (idx, user_id) in user_ids.iter().enumerate() {
            for post_idx in 0..=idx {
                let published = post_idx % 2 == 0;
                let post_data = {
                    let _temp_title = std::rc::Rc::new(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Post {0} by User {1}", post_idx, idx),
                            );
                            res
                        }),
                    );
                    let _temp_content = std::rc::Rc::new(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Content for post {0} by user {1}",
                                    post_idx,
                                    idx,
                                ),
                            );
                            res
                        }),
                    );
                    let _temp_published = std::rc::Rc::new(published);
                    let _temp_user_id = std::rc::Rc::new(*user_id);
                    let _temp_long_u64 = std::rc::Rc::new(9876543210u64);
                    let _temp_long_i64 = std::rc::Rc::new(-1234567890i64);
                    #[allow(unused_variables, unreachable_code, unused_must_use)]
                    {
                        if false {
                            let mut _model = <TestPost>::default();
                            _model.title = (*_temp_title).clone().into();
                            _model.content = (*_temp_content).clone().into();
                            _model.published = (*_temp_published).clone().into();
                            _model.user_id = (*_temp_user_id).clone().into();
                            _model.long_u64 = (*_temp_long_u64).clone().into();
                            _model.long_i64 = (*_temp_long_i64).clone().into();
                        }
                    }
                    let mut result = sqlited::WithoutId::<TestPost>::new();
                    {
                        let value = (*_temp_title).clone();
                        let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                        result.inner.insert("title".to_lowercase(), boxed_value);
                    }
                    {
                        let value = (*_temp_content).clone();
                        let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                        result.inner.insert("content".to_lowercase(), boxed_value);
                    }
                    {
                        let value = (*_temp_published).clone();
                        let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                        result.inner.insert("published".to_lowercase(), boxed_value);
                    }
                    {
                        let value = (*_temp_user_id).clone();
                        let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                        result.inner.insert("user_id".to_lowercase(), boxed_value);
                    }
                    {
                        let value = (*_temp_long_u64).clone();
                        let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                        result.inner.insert("long_u64".to_lowercase(), boxed_value);
                    }
                    {
                        let value = (*_temp_long_i64).clone();
                        let boxed_value: Box<dyn sqlited::rq::ToSql> = Box::new(value);
                        result.inner.insert("long_i64".to_lowercase(), boxed_value);
                    }
                    let ordered_field_names = <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            "title".to_string(),
                            "content".to_string(),
                            "published".to_string(),
                            "user_id".to_string(),
                            "long_u64".to_string(),
                            "long_i64".to_string(),
                        ]),
                    );
                    result.create_static_params_for_fields(&ordered_field_names)
                };
                db.execute(&&TestPost::insert_without_id(), &*post_data).unwrap();
            }
        }
        let age_counts = db
            .query(
                "SELECT age, COUNT(*) FROM user GROUP BY age ORDER BY age",
                [],
                |row| Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?)),
            )
            .unwrap();
        match (&age_counts.len(), &4) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("应该有4个不同的年龄组"),
                        ),
                    );
                }
            }
        };
        match (&age_counts[0], &(25, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("年龄25应该有2个用户"),
                        ),
                    );
                }
            }
        };
        match (&age_counts[1], &(30, 1)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("年龄30应该有1个用户"),
                        ),
                    );
                }
            }
        };
        match (&age_counts[2], &(35, 1)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("年龄35应该有1个用户"),
                        ),
                    );
                }
            }
        };
        match (&age_counts[3], &(40, 1)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("年龄40应该有1个用户"),
                        ),
                    );
                }
            }
        };
        let user_post_counts = db
            .query(
                "SELECT u.name, COUNT(p.id) as post_count 
             FROM user u 
             LEFT JOIN test_post p ON u.id = p.user_id AND p.published = 1
             GROUP BY u.id
             ORDER BY post_count DESC",
                [],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?)),
            )
            .unwrap();
        match (&user_post_counts[0].0, &"Eve") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Eve应该发布最多帖子"),
                        ),
                    );
                }
            }
        };
        match (&user_post_counts[0].1, &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Eve应该有3篇已发布帖子"),
                        ),
                    );
                }
            }
        };
        let count = db
            .query(
                "SELECT COUNT(*) FROM test_post p
             JOIN user u ON p.user_id = u.id
             WHERE u.email IS NULL",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap()[0];
        match (&count, &4) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!(
                                "没有电子邮件的用户总共应该有4篇帖子",
                            ),
                        ),
                    );
                }
            }
        };
        db.transaction(|tx| {
                let updated_count = tx
                    .execute(
                        "UPDATE test_post SET published = 1 WHERE published = 0",
                        [],
                    )?;
                let expected_updates = 1 + 1 + 2 + 2;
                match (&updated_count, &(expected_updates as usize)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    format_args!(
                                        "应该更新了{0}篇未发布的帖子",
                                        expected_updates,
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(())
            })
            .unwrap();
        let all_published = db
            .query(
                "SELECT COUNT(*) = 0 FROM test_post WHERE published = 0",
                [],
                |row| row.get::<_, bool>(0),
            )
            .unwrap()[0];
        if !all_published {
            {
                ::core::panicking::panic_fmt(
                    format_args!("所有帖子都应该被标记为已发布"),
                );
            }
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_error_handling_and_constraints"]
    #[doc(hidden)]
    pub const test_error_handling_and_constraints: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_error_handling_and_constraints"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 592usize,
            start_col: 8usize,
            end_line: 592usize,
            end_col: 43usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_error_handling_and_constraints()),
        ),
    };
    fn test_error_handling_and_constraints() {
        let db = TEST_DB::memory().unwrap();
        let result = db
            .execute(
                "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
                &[
                    &"valid_user" as &dyn ::rusqlite::ToSql,
                    &"valid@example.com" as &dyn ::rusqlite::ToSql,
                    &&30 as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            );
        if !result.is_ok() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("有效用户应该可以成功插入"),
                );
            }
        }
        let result = db
            .execute(
                "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
                &[
                    &"valid_user" as &dyn ::rusqlite::ToSql,
                    &"another@example.com" as &dyn ::rusqlite::ToSql,
                    &&25 as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            );
        if !result.is_err() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("重复的用户名应该导致错误"),
                );
            }
        }
        let result = db
            .execute(
                "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
                &[
                    &"another_user" as &dyn ::rusqlite::ToSql,
                    &"valid@example.com" as &dyn ::rusqlite::ToSql,
                    &&25 as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            );
        if !result.is_err() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("重复的邮箱应该导致错误"),
                );
            }
        }
        let result = db
            .execute(
                "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
                &[
                    &"young_user" as &dyn ::rusqlite::ToSql,
                    &"young@example.com" as &dyn ::rusqlite::ToSql,
                    &&17 as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            );
        if !result.is_err() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("不满足年龄约束应该导致错误"),
                );
            }
        }
        let result = db
            .execute(
                "INSERT INTO constrained_user (username, email, age) VALUES (?, ?, ?)",
                &[
                    &&Option::<String>::None as &dyn ::rusqlite::ToSql,
                    &"no_username@example.com" as &dyn ::rusqlite::ToSql,
                    &&20 as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            );
        if !result.is_err() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("空用户名应该导致错误"),
                );
            }
        }
        let count = db
            .query(
                "SELECT COUNT(*) FROM constrained_user",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap()[0];
        match (&count, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("约束测试后应该只有一个有效用户"),
                        ),
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_blob_and_complex_data_types"]
    #[doc(hidden)]
    pub const test_blob_and_complex_data_types: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_blob_and_complex_data_types"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 642usize,
            start_col: 8usize,
            end_line: 642usize,
            end_col: 40usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_blob_and_complex_data_types()),
        ),
    };
    fn test_blob_and_complex_data_types() {
        let db = TEST_DB::memory().unwrap();
        let raw_conn = db.raw_connection();
        let test_cases = [
            ("Empty", Vec::<u8>::new(), None),
            (
                "Small Binary",
                <[_]>::into_vec(::alloc::boxed::box_new([1, 2, 3, 4, 5])),
                Some("small binary file"),
            ),
            (
                "Large Binary",
                ::alloc::vec::from_elem(255, 10000),
                Some("large binary file"),
            ),
            (
                "Binary with zeros",
                <[_]>::into_vec(::alloc::boxed::box_new([0, 1, 0, 1, 0])),
                Some("binary with zeros"),
            ),
        ];
        for (name, data, metadata) in test_cases.iter() {
            let result = db
                .execute(
                    "INSERT INTO binary_data (name, data, metadata) VALUES (?, ?, ?)",
                    &[
                        &name as &dyn ::rusqlite::ToSql,
                        &&data as &dyn ::rusqlite::ToSql,
                        &&metadata as &dyn ::rusqlite::ToSql,
                    ] as &[&dyn ::rusqlite::ToSql],
                );
            if !result.is_ok() {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Binary data insertion should succeed for {0}",
                            name,
                        ),
                    );
                }
            }
            let id: i32 = raw_conn.last_insert_rowid() as i32;
            let data_result = &db
                .query(
                    "SELECT data FROM binary_data WHERE id = ?",
                    &[&id],
                    |row| row.get::<_, Vec<u8>>(0),
                )
                .unwrap()[0];
            match (&*data_result, &*data) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(
                                format_args!(
                                    "Retrieved binary data should match for {0}",
                                    name,
                                ),
                            ),
                        );
                    }
                }
            };
        }
        let count = db
            .query(
                "SELECT COUNT(*) FROM binary_data WHERE length(data) > 1000",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap()[0];
        match (&count, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!(
                                "应该只有一个大于1000字节的二进制数据",
                            ),
                        ),
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_multiple_connections"]
    #[doc(hidden)]
    pub const test_multiple_connections: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_multiple_connections"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/without_id_test.rs",
            start_line: 684usize,
            start_col: 8usize,
            end_line: 684usize,
            end_col: 33usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_multiple_connections()),
        ),
    };
    fn test_multiple_connections() {
        let db1 = TEST_DB::shared_memory("test_multi_conn").unwrap();
        db1.execute(
                "INSERT INTO user (name, age, email) VALUES (?, ?, ?)",
                &[
                    &"Connection Test" as &dyn ::rusqlite::ToSql,
                    &&35 as &dyn ::rusqlite::ToSql,
                    &&"conn@test.com" as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            )
            .unwrap();
        let db2 = TEST_DB::shared_memory("test_multi_conn").unwrap();
        let name = &db2
            .query(
                "SELECT name FROM user WHERE email = ?",
                &["conn@test.com"],
                |row| row.get::<_, String>(0),
            )
            .unwrap()[0];
        match (&name, &"Connection Test") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!(
                                "第二个连接应该能看到第一个连接插入的数据",
                            ),
                        ),
                    );
                }
            }
        };
        let db3 = db1.new_connection().unwrap();
        db3.execute(
                "UPDATE user SET name = ? WHERE email = ?",
                &[
                    &"Updated Name" as &dyn ::rusqlite::ToSql,
                    &&"conn@test.com" as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            )
            .unwrap();
        let updated_name = &db1
            .query(
                "SELECT name FROM user WHERE email = ?",
                &["conn@test.com"],
                |row| row.get::<_, String>(0),
            )
            .unwrap()[0];
        match (&updated_name, &"Updated Name") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!(
                                "原始连接应该能看到新连接所做的更改",
                            ),
                        ),
                    );
                }
            }
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &test_advanced_queries_and_aggregations,
            &test_blob_and_complex_data_types,
            &test_data_validation_and_integrity,
            &test_error_handling_and_constraints,
            &test_for_insert_macro,
            &test_multiple_connections,
            &test_post_with_foreign_key,
            &test_transaction_and_rollback,
            &test_user_crud_operations,
            &test_without_id_macro,
        ],
    )
}
