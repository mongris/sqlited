#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
/// 初始表结构定义模块
mod initial_schema {
    use rusqlite::params;
    use sqlited::{prelude::*, define_db, table};
    use std::path::PathBuf;
    pub struct User {
        pub id: i32,
        pub name: String,
        pub email: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for User {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "User",
                "id",
                &self.id,
                "name",
                &self.name,
                "email",
                &&self.email,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for User {
        #[inline]
        fn clone(&self) -> User {
            User {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                email: ::core::clone::Clone::clone(&self.email),
            }
        }
    }
    impl Default for User {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
                email: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for User {
        fn table_name() -> &'static str {
            "user"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "name", "email"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("email", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "email",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
    pub struct Post {
        pub id: i32,
        pub title: String,
        pub content: String,
        pub active: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Post {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Post",
                "id",
                &self.id,
                "title",
                &self.title,
                "content",
                &self.content,
                "active",
                &&self.active,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Post {
        #[inline]
        fn clone(&self) -> Post {
            Post {
                id: ::core::clone::Clone::clone(&self.id),
                title: ::core::clone::Clone::clone(&self.title),
                content: ::core::clone::Clone::clone(&self.content),
                active: ::core::clone::Clone::clone(&self.active),
            }
        }
    }
    impl Default for Post {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                title: <String>::default(),
                content: <String>::default(),
                active: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Post {
        fn table_name() -> &'static str {
            "post"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new(["id", "title", "content", "active"]),
            )
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("title", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("content", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("active", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "active",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Post {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct Contact {
        pub id: i32,
        pub name: String,
        pub contact_email: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Contact {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Contact",
                "id",
                &self.id,
                "name",
                &self.name,
                "contact_email",
                &&self.contact_email,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Contact {
        #[inline]
        fn clone(&self) -> Contact {
            Contact {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                contact_email: ::core::clone::Clone::clone(&self.contact_email),
            }
        }
    }
    impl Default for Contact {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
                contact_email: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Contact {
        fn table_name() -> &'static str {
            "contact"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "name", "contact_email"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    (
                        "contact_email",
                        <String as sqlited::SqliteTypeName>::sql_type_name(),
                    ),
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
                            "contact_email",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Contact {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct Article {
        pub id: i32,
        pub title: String,
        pub content: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Article {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Article",
                "id",
                &self.id,
                "title",
                &self.title,
                "content",
                &&self.content,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Article {
        #[inline]
        fn clone(&self) -> Article {
            Article {
                id: ::core::clone::Clone::clone(&self.id),
                title: ::core::clone::Clone::clone(&self.title),
                content: ::core::clone::Clone::clone(&self.content),
            }
        }
    }
    impl Default for Article {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                title: <String>::default(),
                content: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Article {
        fn table_name() -> &'static str {
            "article"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "title", "content"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("title", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("content", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
    impl Article {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct Product {
        pub id: i32,
        pub name: String,
        pub price: f64,
        pub description: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Product {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Product",
                "id",
                &self.id,
                "name",
                &self.name,
                "price",
                &self.price,
                "description",
                &&self.description,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Product {
        #[inline]
        fn clone(&self) -> Product {
            Product {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                price: ::core::clone::Clone::clone(&self.price),
                description: ::core::clone::Clone::clone(&self.description),
            }
        }
    }
    impl Default for Product {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
                price: <f64>::default(),
                description: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Product {
        fn table_name() -> &'static str {
            "product"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new(["id", "name", "price", "description"]),
            )
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("price", <f64 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("description", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "price",
                            <f64 as sqlited::SqliteTypeName>::sql_type_name(),
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
                            "description",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Product {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct Category {
        pub id: i32,
        pub name: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Category {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Category",
                "id",
                &self.id,
                "name",
                &&self.name,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Category {
        #[inline]
        fn clone(&self) -> Category {
            Category {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    impl Default for Category {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Category {
        fn table_name() -> &'static str {
            "category"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "name"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
    impl Category {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct Comment {
        pub id: i32,
        pub post_id: i32,
        pub content: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Comment {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Comment",
                "id",
                &self.id,
                "post_id",
                &self.post_id,
                "content",
                &&self.content,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Comment {
        #[inline]
        fn clone(&self) -> Comment {
            Comment {
                id: ::core::clone::Clone::clone(&self.id),
                post_id: ::core::clone::Clone::clone(&self.post_id),
                content: ::core::clone::Clone::clone(&self.content),
            }
        }
    }
    impl Default for Comment {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                post_id: <i32>::default(),
                content: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Comment {
        fn table_name() -> &'static str {
            "comment"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "post_id", "content"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("post_id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("content", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "post_id",
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
                            "content",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Comment {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct Task {
        pub id: i32,
        pub title: String,
        pub description: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Task {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Task",
                "id",
                &self.id,
                "title",
                &self.title,
                "description",
                &&self.description,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Task {
        #[inline]
        fn clone(&self) -> Task {
            Task {
                id: ::core::clone::Clone::clone(&self.id),
                title: ::core::clone::Clone::clone(&self.title),
                description: ::core::clone::Clone::clone(&self.description),
            }
        }
    }
    impl Default for Task {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                title: <String>::default(),
                description: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Task {
        fn table_name() -> &'static str {
            "task"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "title", "description"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("title", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("description", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "description",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Task {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            ::alloc::vec::Vec::new()
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
                match std::panic::catch_unwind(|| Post::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Contact::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Article::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Product::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Category::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Comment::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Task::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            migrations
        }
        fn get_migrations() -> Vec<String> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    User::create_table_sql().to_string(),
                    Post::create_table_sql().to_string(),
                    Contact::create_table_sql().to_string(),
                    Article::create_table_sql().to_string(),
                    Product::create_table_sql().to_string(),
                    Category::create_table_sql().to_string(),
                    Comment::create_table_sql().to_string(),
                    Task::create_table_sql().to_string(),
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
                        let statement_hash = get_statement_key(statement);
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
                        rusqlite::Error::SqliteFailure(
                            rusqlite::ffi::Error {
                                code: rusqlite::ffi::ErrorCode::InternalMalfunction,
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
    pub struct INITIAL_DB {}
    impl INITIAL_DB {
        /// 打开给定路径的数据库（如果为None则使用内存模式）
        pub fn open(
            path: Option<impl AsRef<std::path::Path>>,
        ) -> ::sqlited::error::Result<Database> {
            match path {
                Some(path) => {
                    let path_buf = path.as_ref().to_path_buf();
                    let canonical_path = if path_buf.exists() {
                        std::fs::canonicalize(&path_buf).unwrap_or(path_buf)
                    } else {
                        if let Some(parent) = path_buf.parent() {
                            std::fs::create_dir_all(parent)
                                .map_err(|e| ::sqlited::error::SqlitedError::Rusqlite(
                                    rusqlite::Error::SqliteFailure(
                                        rusqlite::ffi::Error {
                                            code: rusqlite::ffi::ErrorCode::CannotOpen,
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
                                    rusqlite::Error::SqliteFailure(
                                        rusqlite::ffi::Error {
                                            code: rusqlite::ffi::ErrorCode::InternalMalfunction,
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
                    Ok(db)
                }
                None => {
                    let pool = ::sqlited::pool::ConnectionPool::new_memory()
                        .map(std::sync::Arc::new)
                        .map_err(|pool_err| ::sqlited::error::SqlitedError::Rusqlite(
                            rusqlite::Error::SqliteFailure(
                                rusqlite::ffi::Error {
                                    code: rusqlite::ffi::ErrorCode::InternalMalfunction,
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
                    Ok(db)
                }
            }
        }
        /// 打开内存数据库
        pub fn memory() -> ::sqlited::Result<Database> {
            Self::open(None::<&std::path::Path>)
        }
        /// 在临时位置创建数据库
        pub fn temp() -> ::sqlited::Result<Database> {
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
            Self::open(Some(db_file))
        }
        /// 打开共享内存数据库（使用命名内存数据库）
        pub fn shared_memory(name: &str) -> ::sqlited::Result<Database> {
            let memory_path = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!("file:{0}?mode=memory&cache=shared", name),
                );
                res
            });
            Self::open(Some(memory_path))
        }
    }
    #[allow(non_upper_case_globals)]
    pub static INITIAL_DB: INITIAL_DB = INITIAL_DB {};
    pub fn initialize_test_db(db_path: &str) {
        let db = INITIAL_DB::open(Some(db_path)).unwrap();
        db.execute(
                "INSERT INTO post (title, content, active) VALUES (?, ?, ?)",
                ["Initial Post", "Content", "yes"],
            )
            .unwrap();
        db.execute(
                "INSERT INTO product (name, price, description) VALUES (?, ?, ?)",
                &[
                    &"Low Price Item" as &dyn ::rusqlite::ToSql,
                    &50.0 as &dyn ::rusqlite::ToSql,
                    &"Old description" as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            )
            .unwrap();
        db.execute(
                "INSERT INTO product (name, price, description) VALUES (?, ?, ?)",
                &[
                    &"High Price Item" as &dyn ::rusqlite::ToSql,
                    &200.0 as &dyn ::rusqlite::ToSql,
                    &"Expensive product" as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            )
            .unwrap();
    }
}
/// 测试迁移后的结构
mod migration_tests {
    use rusqlite::params;
    use sqlited::{prelude::*, define_db, table, UtcDateTime};
    use std::path::PathBuf;
    use std::fs;
    pub struct User {
        pub id: i32,
        pub name: String,
        pub email: String,
        pub bio: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for User {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "User",
                "id",
                &self.id,
                "name",
                &self.name,
                "email",
                &self.email,
                "bio",
                &&self.bio,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for User {
        #[inline]
        fn clone(&self) -> User {
            User {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                email: ::core::clone::Clone::clone(&self.email),
                bio: ::core::clone::Clone::clone(&self.bio),
            }
        }
    }
    impl Default for User {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
                email: <String>::default(),
                bio: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for User {
        fn table_name() -> &'static str {
            "user"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "name", "email", "bio"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("email", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("bio", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "email",
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
                            "bio",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    (
                        "error".to_string(),
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "-- Column {0} not found in struct definition",
                                    "add_column",
                                ),
                            );
                            res
                        }),
                        None,
                    ),
                ]),
            )
        }
    }
    pub struct Post {
        pub id: i32,
        pub title: String,
        pub content: String,
        pub active: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Post {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Post",
                "id",
                &self.id,
                "title",
                &self.title,
                "content",
                &self.content,
                "active",
                &&self.active,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Post {
        #[inline]
        fn clone(&self) -> Post {
            Post {
                id: ::core::clone::Clone::clone(&self.id),
                title: ::core::clone::Clone::clone(&self.title),
                content: ::core::clone::Clone::clone(&self.content),
                active: ::core::clone::Clone::clone(&self.active),
            }
        }
    }
    impl Default for Post {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                title: <String>::default(),
                content: <String>::default(),
                active: <bool>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Post {
        fn table_name() -> &'static str {
            "post"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new(["id", "title", "content", "active"]),
            )
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("title", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("content", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
    impl Post {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    (
                        "error".to_string(),
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "-- Column {0} not found in struct definition",
                                    "modify_column",
                                ),
                            );
                            res
                        }),
                        None,
                    ),
                ]),
            )
        }
    }
    pub struct Contact {
        pub id: i32,
        pub name: String,
        pub email: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Contact {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Contact",
                "id",
                &self.id,
                "name",
                &self.name,
                "email",
                &&self.email,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Contact {
        #[inline]
        fn clone(&self) -> Contact {
            Contact {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                email: ::core::clone::Clone::clone(&self.email),
            }
        }
    }
    impl Default for Contact {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
                email: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Contact {
        fn table_name() -> &'static str {
            "contact"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "name", "email"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("email", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "email",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Contact {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    (
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "migration_{0}_rename_{1}_to_{2}",
                                    "contact",
                                    "rename_column",
                                    "contact_email",
                                ),
                            );
                            res
                        }),
                        "ALTER TABLE contact RENAME COLUMN rename_column TO contact_email"
                            .to_string(),
                        Some(
                            "ALTER TABLE contact RENAME COLUMN contact_email TO rename_column"
                                .to_string(),
                        ),
                    ),
                ]),
            )
        }
    }
    pub struct Article {
        pub id: i32,
        pub title: String,
        pub content: String,
        pub created_at: UtcDateTime,
        pub updated_at: UtcDateTime,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Article {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Article",
                "id",
                &self.id,
                "title",
                &self.title,
                "content",
                &self.content,
                "created_at",
                &self.created_at,
                "updated_at",
                &&self.updated_at,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Article {
        #[inline]
        fn clone(&self) -> Article {
            Article {
                id: ::core::clone::Clone::clone(&self.id),
                title: ::core::clone::Clone::clone(&self.title),
                content: ::core::clone::Clone::clone(&self.content),
                created_at: ::core::clone::Clone::clone(&self.created_at),
                updated_at: ::core::clone::Clone::clone(&self.updated_at),
            }
        }
    }
    impl Default for Article {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                title: <String>::default(),
                content: <String>::default(),
                created_at: <UtcDateTime>::default(),
                updated_at: <UtcDateTime>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Article {
        fn table_name() -> &'static str {
            "article"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    "id",
                    "title",
                    "content",
                    "created_at",
                    "updated_at",
                ]),
            )
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("title", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("content", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    (
                        "created_at",
                        <UtcDateTime as sqlited::SqliteTypeName>::sql_type_name(),
                    ),
                    (
                        "updated_at",
                        <UtcDateTime as sqlited::SqliteTypeName>::sql_type_name(),
                    ),
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
            field_constraints.push_str(&" DEFAULT CURRENT_TIMESTAMP");
            sql.push_str(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "    {0} {1}{2},\n",
                            "updated_at",
                            <UtcDateTime as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Article {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    (
                        "error".to_string(),
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "-- Column {0} not found in struct definition",
                                    "add_column",
                                ),
                            );
                            res
                        }),
                        None,
                    ),
                    (
                        "error".to_string(),
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "-- Column {0} not found in struct definition",
                                    "add_column",
                                ),
                            );
                            res
                        }),
                        None,
                    ),
                ]),
            )
        }
    }
    pub struct Product {
        pub id: i32,
        pub name: String,
        pub price: f64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Product {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Product",
                "id",
                &self.id,
                "name",
                &self.name,
                "price",
                &&self.price,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Product {
        #[inline]
        fn clone(&self) -> Product {
            Product {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                price: ::core::clone::Clone::clone(&self.price),
            }
        }
    }
    impl Default for Product {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
                price: <f64>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Product {
        fn table_name() -> &'static str {
            "product"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "name", "price"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    ("price", <f64 as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "price",
                            <f64 as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Product {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    (
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "migration_{0}_drop_{1}",
                                    "product",
                                    "drop_column",
                                ),
                            );
                            res
                        }),
                        "ALTER TABLE product DROP COLUMN drop_column".to_string(),
                        None,
                    ),
                ]),
            )
        }
    }
    pub struct Category {
        pub id: i32,
        pub name: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Category {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Category",
                "id",
                &self.id,
                "name",
                &&self.name,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Category {
        #[inline]
        fn clone(&self) -> Category {
            Category {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    impl Default for Category {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Category {
        fn table_name() -> &'static str {
            "category"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "name"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
    impl Category {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    (
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "migration_{0}_add_index_{1}",
                                    "category",
                                    "add_index",
                                ),
                            );
                            res
                        }),
                        "CREATE INDEX add_index ON category (categories_name_unique)"
                            .to_string(),
                        Some("DROP INDEX add_index".to_string()),
                    ),
                ]),
            )
        }
    }
    pub struct Comment {
        pub id: i32,
        pub post_id: i32,
        pub content: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Comment {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Comment",
                "id",
                &self.id,
                "post_id",
                &self.post_id,
                "content",
                &&self.content,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Comment {
        #[inline]
        fn clone(&self) -> Comment {
            Comment {
                id: ::core::clone::Clone::clone(&self.id),
                post_id: ::core::clone::Clone::clone(&self.post_id),
                content: ::core::clone::Clone::clone(&self.content),
            }
        }
    }
    impl Default for Comment {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                post_id: <i32>::default(),
                content: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Comment {
        fn table_name() -> &'static str {
            "comment"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "post_id", "content"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("post_id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("content", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "post_id",
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
                            "content",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Comment {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    (
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "migration_{0}_add_index_{1}",
                                    "comment",
                                    "add_index",
                                ),
                            );
                            res
                        }),
                        "CREATE INDEX add_index ON comment (comments_post_id_idx)"
                            .to_string(),
                        Some("DROP INDEX add_index".to_string()),
                    ),
                ]),
            )
        }
    }
    pub struct CustomMigrationTest {
        pub id: i32,
        pub name: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CustomMigrationTest {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CustomMigrationTest",
                "id",
                &self.id,
                "name",
                &&self.name,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CustomMigrationTest {
        #[inline]
        fn clone(&self) -> CustomMigrationTest {
            CustomMigrationTest {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    impl Default for CustomMigrationTest {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                name: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for CustomMigrationTest {
        fn table_name() -> &'static str {
            "custom_migration_test"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(::alloc::boxed::box_new(["id", "name"]))
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("name", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
    impl CustomMigrationTest {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    (
                        "custom".to_string(),
                        "update_product_prices".to_string(),
                        Some(
                            "UPDATE product SET price = price * 1.1 WHERE price < 100"
                                .to_string(),
                        ),
                    ),
                ]),
            )
        }
    }
    pub struct Task {
        pub id: i32,
        pub title: String,
        pub description: String,
        pub status: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Task {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Task",
                "id",
                &self.id,
                "title",
                &self.title,
                "description",
                &self.description,
                "status",
                &&self.status,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Task {
        #[inline]
        fn clone(&self) -> Task {
            Task {
                id: ::core::clone::Clone::clone(&self.id),
                title: ::core::clone::Clone::clone(&self.title),
                description: ::core::clone::Clone::clone(&self.description),
                status: ::core::clone::Clone::clone(&self.status),
            }
        }
    }
    impl Default for Task {
        fn default() -> Self {
            Self {
                id: <i32>::default(),
                title: <String>::default(),
                description: <String>::default(),
                status: <String>::default(),
            }
        }
    }
    impl sqlited::WithoutIdTableInfo for Task {
        fn table_name() -> &'static str {
            "task"
        }
        fn field_names() -> Vec<&'static str> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new(["id", "title", "description", "status"]),
            )
        }
        fn field_types() -> Vec<(&'static str, &'static str)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    ("id", <i32 as sqlited::SqliteTypeName>::sql_type_name()),
                    ("title", <String as sqlited::SqliteTypeName>::sql_type_name()),
                    (
                        "description",
                        <String as sqlited::SqliteTypeName>::sql_type_name(),
                    ),
                    ("status", <String as sqlited::SqliteTypeName>::sql_type_name()),
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
                            "description",
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
                            "status",
                            <String as sqlited::SqliteTypeName>::sql_type_name(),
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
    impl Task {
        /// 获取此表的所有迁移SQL语句
        pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    (
                        "error".to_string(),
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "-- Column {0} not found in struct definition",
                                    "add_column",
                                ),
                            );
                            res
                        }),
                        None,
                    ),
                    (
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "migration_{0}_add_index_{1}",
                                    "task",
                                    "add_index",
                                ),
                            );
                            res
                        }),
                        "CREATE INDEX add_index ON task (task_status_idx)".to_string(),
                        Some("DROP INDEX add_index".to_string()),
                    ),
                    (
                        "custom".to_string(),
                        "populate_status".to_string(),
                        Some(
                            "UPDATE task SET status = 'pending' WHERE status IS NULL"
                                .to_string(),
                        ),
                    ),
                ]),
            )
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
                match std::panic::catch_unwind(|| Post::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Contact::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Article::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Product::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Category::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Comment::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| CustomMigrationTest::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            if let Some(table_migrations) = {
                match std::panic::catch_unwind(|| Task::get_migrations()) {
                    Ok(migrations) => Some(migrations),
                    Err(_) => None,
                }
            } {
                migrations.extend(table_migrations);
            }
            migrations
        }
        fn get_migrations() -> Vec<String> {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    User::create_table_sql().to_string(),
                    Post::create_table_sql().to_string(),
                    Contact::create_table_sql().to_string(),
                    Article::create_table_sql().to_string(),
                    Product::create_table_sql().to_string(),
                    Category::create_table_sql().to_string(),
                    Comment::create_table_sql().to_string(),
                    CustomMigrationTest::create_table_sql().to_string(),
                    Task::create_table_sql().to_string(),
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
                        let statement_hash = get_statement_key(statement);
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
                        rusqlite::Error::SqliteFailure(
                            rusqlite::ffi::Error {
                                code: rusqlite::ffi::ErrorCode::InternalMalfunction,
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
    pub struct MIGRATED_DB {}
    impl MIGRATED_DB {
        /// 打开给定路径的数据库（如果为None则使用内存模式）
        pub fn open(
            path: Option<impl AsRef<std::path::Path>>,
        ) -> ::sqlited::error::Result<Database> {
            match path {
                Some(path) => {
                    let path_buf = path.as_ref().to_path_buf();
                    let canonical_path = if path_buf.exists() {
                        std::fs::canonicalize(&path_buf).unwrap_or(path_buf)
                    } else {
                        if let Some(parent) = path_buf.parent() {
                            std::fs::create_dir_all(parent)
                                .map_err(|e| ::sqlited::error::SqlitedError::Rusqlite(
                                    rusqlite::Error::SqliteFailure(
                                        rusqlite::ffi::Error {
                                            code: rusqlite::ffi::ErrorCode::CannotOpen,
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
                                    rusqlite::Error::SqliteFailure(
                                        rusqlite::ffi::Error {
                                            code: rusqlite::ffi::ErrorCode::InternalMalfunction,
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
                    Ok(db)
                }
                None => {
                    let pool = ::sqlited::pool::ConnectionPool::new_memory()
                        .map(std::sync::Arc::new)
                        .map_err(|pool_err| ::sqlited::error::SqlitedError::Rusqlite(
                            rusqlite::Error::SqliteFailure(
                                rusqlite::ffi::Error {
                                    code: rusqlite::ffi::ErrorCode::InternalMalfunction,
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
                    Ok(db)
                }
            }
        }
        /// 打开内存数据库
        pub fn memory() -> ::sqlited::Result<Database> {
            Self::open(None::<&std::path::Path>)
        }
        /// 在临时位置创建数据库
        pub fn temp() -> ::sqlited::Result<Database> {
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
            Self::open(Some(db_file))
        }
        /// 打开共享内存数据库（使用命名内存数据库）
        pub fn shared_memory(name: &str) -> ::sqlited::Result<Database> {
            let memory_path = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!("file:{0}?mode=memory&cache=shared", name),
                );
                res
            });
            Self::open(Some(memory_path))
        }
    }
    #[allow(non_upper_case_globals)]
    pub static MIGRATED_DB: MIGRATED_DB = MIGRATED_DB {};
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "migration_tests::test_migrations"]
    #[doc(hidden)]
    pub const test_migrations: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("migration_tests::test_migrations"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/migration_test.rs",
            start_line: 254usize,
            start_col: 10usize,
            end_line: 254usize,
            end_col: 25usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_migrations()),
        ),
    };
    pub fn test_migrations() {
        let db_path = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!(
                    "file:migration_test_{0}.db?mode=memory&cache=shared",
                    uuid::Uuid::new_v4(),
                ),
            );
            res
        });
        super::initial_schema::initialize_test_db(&db_path);
        let db = MIGRATED_DB::open(Some(&db_path)).unwrap();
        test_add_column_migration(&db);
        test_modify_column_type(&db);
        test_rename_column(&db);
        test_add_multiple_columns(&db);
        test_drop_column(&db);
        test_add_constraint_and_index(&db);
        test_custom_migration(&db);
        test_complex_migrations(&db);
    }
    fn test_add_column_migration(db: &Database) {
        let has_bio_column = db
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('user') WHERE name = 'bio'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0);
        match (&has_bio_column, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Bio column should be added"),
                        ),
                    );
                }
            }
        };
        db.execute(
                "INSERT INTO user (name, email, bio) VALUES (?, ?, ?)",
                ["John Doe", "john@example.com", "This is my bio"],
            )
            .unwrap();
        let bio = db
            .query_row(
                "SELECT bio FROM user WHERE name = ?",
                ["John Doe"],
                |row| row.get::<_, String>(0),
            )
            .unwrap();
        match (&bio, &"This is my bio") {
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
    fn test_modify_column_type(db: &Database) {
        let active_value = db
            .query_row(
                "SELECT active FROM post WHERE title = ?",
                ["Initial Post"],
                |row| row.get::<_, bool>(0),
            )
            .unwrap();
        if !active_value {
            {
                ::core::panicking::panic_fmt(
                    format_args!("String \'yes\' should be converted to boolean true"),
                );
            }
        }
        db.execute(
                "INSERT INTO post (title, content, active) VALUES (?, ?, ?)",
                &[
                    &"New Post" as &dyn ::rusqlite::ToSql,
                    &"Content" as &dyn ::rusqlite::ToSql,
                    &false as &dyn ::rusqlite::ToSql,
                ] as &[&dyn ::rusqlite::ToSql],
            )
            .unwrap();
        let active = db
            .query_row(
                "SELECT active FROM post WHERE title = ?",
                ["New Post"],
                |row| row.get::<_, bool>(0),
            )
            .unwrap();
        if !!active {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Boolean false should be stored correctly"),
                );
            }
        }
    }
    fn test_rename_column(db: &Database) {
        let has_email = db
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('contact') WHERE name = 'email'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0);
        match (&has_email, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Email column should exist after renaming"),
                        ),
                    );
                }
            }
        };
        let has_old_email = db
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('contact') WHERE name = 'contact_email'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0);
        match (&has_old_email, &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Old contact_email column should not exist"),
                        ),
                    );
                }
            }
        };
    }
    fn test_add_multiple_columns(db: &Database) {
        let timestamp_columns = db
            .query(
                "SELECT name FROM pragma_table_info('article') WHERE name IN ('created_at', 'updated_at')",
                [],
                |row| row.get::<_, String>(0),
            )
            .unwrap();
        match (&timestamp_columns.len(), &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Both timestamp columns should be added"),
                        ),
                    );
                }
            }
        };
        db.execute(
                "INSERT INTO article (title, content) VALUES (?, ?)",
                ["Test Article", "Content"],
            )
            .unwrap();
        let has_timestamps = db
            .query_row(
                "SELECT COUNT(*) FROM article WHERE created_at IS NOT NULL AND updated_at IS NOT NULL",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0);
        match (&has_timestamps, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Timestamp default values should be applied"),
                        ),
                    );
                }
            }
        };
    }
    fn test_drop_column(db: &Database) {
        let has_description = db
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('product') WHERE name = 'description'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0);
        match (&has_description, &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Description column should be dropped"),
                        ),
                    );
                }
            }
        };
    }
    fn test_add_constraint_and_index(db: &Database) {
        db.execute("INSERT INTO category (name) VALUES (?)", ["Electronics"]).unwrap();
        let has_index = db
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'index' AND name = 'categories_name_unique'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0);
        match (&has_index, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Unique index should be created"),
                        ),
                    );
                }
            }
        };
        let result = db
            .execute("INSERT INTO category (name) VALUES (?)", ["Electronics"]);
        if !result.is_err() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Unique constraint should prevent duplicate names"),
                );
            }
        }
        if let Err(err) = result {
            if !(err.to_string().contains("UNIQUE constraint failed")
                || err.to_string().contains("unique constraint"))
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Error should mention unique constraint: {0}", err),
                    );
                }
            }
        }
        let has_comment_index = db
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'index' AND name = 'comments_post_id_idx'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0);
        match (&has_comment_index, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Index on comment should be created"),
                        ),
                    );
                }
            }
        };
    }
    fn test_custom_migration(db: &Database) {
        let low_price = db
            .query_row(
                "SELECT price FROM product WHERE name = ?",
                ["Low Price Item"],
                |row| row.get::<_, f64>(0),
            )
            .unwrap();
        if !((low_price - 55.0).abs() < 0.001) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Price should be updated by custom migration"),
                );
            }
        }
        let high_price = db
            .query_row(
                "SELECT price FROM product WHERE name = ?",
                ["High Price Item"],
                |row| row.get::<_, f64>(0),
            )
            .unwrap();
        if !((high_price - 200.0).abs() < 0.001) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("High price should remain unchanged"),
                );
            }
        }
    }
    fn test_complex_migrations(db: &Database) {
        db.execute(
                "INSERT INTO task (title, description) VALUES (?, ?)",
                ["Complex Task", "Complex Description"],
            )
            .unwrap();
        let status = db
            .query_row(
                "SELECT status FROM task WHERE title = ?",
                ["Complex Task"],
                |row| row.get::<_, Option<String>>(0),
            )
            .unwrap();
        match status {
            Some(status_value) => {
                match (&status_value, &"pending") {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    format_args!("Status should be set to \'pending\'"),
                                ),
                            );
                        }
                    }
                };
            }
            None => {
                {
                    ::std::io::_print(
                        format_args!("Status is NULL, applying migration manually\n"),
                    );
                };
                db.execute("UPDATE task SET status = 'pending' WHERE status IS NULL", [])
                    .unwrap();
                let updated_status = db
                    .query_row(
                        "SELECT status FROM task WHERE title = ?",
                        ["Complex Task"],
                        |row| row.get::<_, String>(0),
                    )
                    .unwrap();
                match (&updated_status, &"pending") {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    format_args!(
                                        "Status should be set to \'pending\' after manual update",
                                    ),
                                ),
                            );
                        }
                    }
                };
            }
        }
        let has_index = db
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'index' AND name = 'task_status_idx'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0);
        match (&has_index, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Index should be created"),
                        ),
                    );
                }
            }
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "main_migration_test"]
#[doc(hidden)]
pub const main_migration_test: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("main_migration_test"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/migration_test.rs",
        start_line: 515usize,
        start_col: 4usize,
        end_line: 515usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(main_migration_test()),
    ),
};
fn main_migration_test() {
    migration_tests::test_migrations();
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&main_migration_test, &test_migrations])
}
