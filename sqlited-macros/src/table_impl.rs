use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, Data, DeriveInput, Fields, LitStr, Meta, Token, parse_macro_input};

use crate::utils::{convert_to_snake_name, find_closest_match};

/// 表级属性
struct TableAttribute {
    attr_type: TableAttributeType,
    value: Vec<String>,
    migration_type: Option<MigrationType>,
}

/// 表级属性类型
enum TableAttributeType {
    Constraint,
    Index,
    UniqueIndex,
    Migration,
}

/// 迁移操作类型
enum MigrationType {
    AddColumn,
    RenameColumn,
    ModifyColumn,
    DropColumn,
    // AddConstraint,
    // DropConstraint,
    AddIndex,
    DropIndex,
    Custom, // 自定义SQL迁移
}

/// 字段属性
#[allow(dead_code)]
struct FieldAttribute {
    name: syn::Ident,
    ty: syn::Type,
    is_autoincrement: bool,
    is_primary_key: bool,
    is_unique: bool,
    is_not_null: bool,
    check_constraint: Option<String>,
    default: Option<String>,
    foreign_key: Option<(String, String, String, String)>, // (table, column, on_delete, on_update)
}

/// 解析表结构并生成完整的表实现
pub fn table(input: TokenStream) -> TokenStream {
    // 解析输入为 struct 定义
    let input = parse_macro_input!(input as DeriveInput);

    // println!("Processing table: {} attrs: {:?}", input.ident, input.attrs);

    // 获取结构体名称
    let struct_name = &input.ident;
    // let struct_name_str = struct_name.to_string();

    // 获取结构体字段
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            _ => {
                return syn::Error::new_spanned(&input, "表结构必须使用命名字段 (named fields)")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "table 宏只能用于结构体")
                .to_compile_error()
                .into();
        }
    };

    // 处理表级属性 (如 constraint, index)
    let table_attributes = process_table_attributes(&input.attrs);

    // 处理字段和它们的属性
    let field_attributes = fields
        .named
        .iter()
        .map(|field| process_field_attributes(field))
        .collect::<Vec<_>>();

    // 生成表信息实现
    generate_table_impl(
        struct_name,
        &fields.named,
        &table_attributes,
        &field_attributes,
    )
}

/// 解析表级属性，增加迁移类型拼写检查
fn process_table_attributes(attrs: &[Attribute]) -> Vec<TableAttribute> {
    let mut table_attrs = Vec::new();

    for attr in attrs {
        if let Some(attr_meta_name) = attr.path().get_ident() {
            // 打印调试信息
            // println!("Processing attribute: {}", attr_name);

            if attr_meta_name == "migration" {
                match &attr.meta {
                    Meta::List(meta_list) => {
                        let args: Vec<String> = meta_list
                            .parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                            .map(|p| p.iter().map(|lit| lit.value()).collect())
                            .unwrap_or_default();
                        if args.is_empty() {
                            emit_error!(
                                meta_list.span(),
                                "Migration requires at least one argument (migration type)"
                            );
                            continue;
                        }

                        // 检查迁移类型是否有效
                        let migration_type_str = &args[0];
                        let valid_migration_types = [
                            "add_column",
                            "rename_column",
                            "modify_column",
                            "drop_column",
                            "add_index",
                            "drop_index",
                            "custom",
                        ];

                        if !valid_migration_types.contains(&migration_type_str.as_str()) {
                            let suggestion =
                                find_closest_match(migration_type_str, &valid_migration_types);
                            let error_msg = if let Some(suggested) = suggestion {
                                format!(
                                    "Invalid migration type '{}'. Did you mean '{}'?",
                                    migration_type_str, suggested
                                )
                            } else {
                                format!(
                                    "Invalid migration type '{}'. Valid types are: {}",
                                    migration_type_str,
                                    valid_migration_types.join(", ")
                                )
                            };

                            emit_error!(meta_list.span(), "{}", error_msg);
                            continue;
                        }

                        // 解析迁移类型
                        let migration_type = match migration_type_str.as_str() {
                            "add_column" => MigrationType::AddColumn,
                            "rename_column" => MigrationType::RenameColumn,
                            "modify_column" => MigrationType::ModifyColumn,
                            "drop_column" => MigrationType::DropColumn,
                            "add_index" => MigrationType::AddIndex,
                            "drop_index" => MigrationType::DropIndex,
                            "custom" => MigrationType::Custom,
                            _ => unreachable!(),
                        };

                        // 检查参数数量是否正确
                        check_migration_args(&migration_type, &args, meta_list.span());

                        table_attrs.push(TableAttribute {
                            attr_type: TableAttributeType::Migration,
                            value: args[1..].to_vec(), // 迁移参数
                            migration_type: Some(migration_type),
                        });
                    }
                    _ => {
                        emit_error!(attr.span(), "Invalid attribute format for migration");
                    }
                }
            } else if attr_meta_name == "constraint" {
                // 处理表级约束
                match &attr.meta {
                    Meta::List(list) => {
                        if let Ok(lit) = list.parse_args::<LitStr>() {
                            // println!("Found constraint: {}", lit.value());
                            table_attrs.push(TableAttribute {
                                attr_type: TableAttributeType::Constraint,
                                value: vec![lit.value()],
                                migration_type: None,
                            });
                        }
                    }
                    _ => panic!("Incorrect format for using the `constraint` attribute."),
                }
            } else if attr_meta_name == "index" {
                // 处理普通索引
                match &attr.meta {
                    Meta::List(list) => {
                        if let Ok(args) =
                            list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                        {
                            if args.len() >= 2 {
                                let idx_name = args[0].value();
                                let idx_columns = args[1].value();
                                // println!("Found index: {} -> {}", idx_name, idx_columns);
                                table_attrs.push(TableAttribute {
                                    attr_type: TableAttributeType::Index,
                                    value: vec![idx_name, idx_columns],
                                    migration_type: None,
                                });
                            }
                        }
                    }
                    _ => panic!("Incorrect format for using the `index` attribute."),
                }
            } else if attr_meta_name == "unique_index" {
                // 处理唯一索引
                match &attr.meta {
                    Meta::List(list) => {
                        if let Ok(args) =
                            list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                        {
                            if args.len() >= 2 {
                                let idx_name = args[0].value();
                                let idx_columns = args[1].value();
                                // println!("Found unique index: {} -> {}", idx_name, idx_columns);
                                table_attrs.push(TableAttribute {
                                    attr_type: TableAttributeType::UniqueIndex,
                                    value: vec![idx_name, idx_columns],
                                    migration_type: None,
                                });
                            }
                        }
                    }
                    _ => panic!("Incorrect format for using the `unique_index` attribute."),
                }
            }
        }
    }

    table_attrs
}

/// 添加辅助函数：检查迁移参数是否完整
fn check_migration_args(migration_type: &MigrationType, args: &[String], span: proc_macro2::Span) {
    let required_args = match migration_type {
        MigrationType::AddColumn => 2,    // 类型 + 列名
        MigrationType::RenameColumn => 3, // 类型 + 旧列名 + 新列名
        MigrationType::ModifyColumn => 2, // 类型 + 列名
        MigrationType::DropColumn => 2,   // 类型 + 列名
        MigrationType::AddIndex => 3,     // 类型 + 索引名 + 列名(s)
        MigrationType::DropIndex => 2,    // 类型 + 索引名
        MigrationType::Custom => 3,       // 类型 + 迁移名 + 上升SQL
    };

    if args.len() < required_args {
        let err_msg = match migration_type {
            MigrationType::AddColumn => format!("'add_column' migration requires column name"),
            MigrationType::RenameColumn => {
                format!("'rename_column' migration requires old_name and new_name")
            }
            MigrationType::ModifyColumn => {
                format!("'modify_column' migration requires column name")
            }
            MigrationType::DropColumn => format!("'drop_column' migration requires column name"),
            MigrationType::AddIndex => {
                format!("'add_index' migration requires index_name and column(s)")
            }
            MigrationType::DropIndex => format!("'drop_index' migration requires index_name"),
            MigrationType::Custom => format!("'custom' migration requires name and SQL statement"),
        };

        emit_error!(span, "{}", err_msg);
    }
}

/// 处理字段属性
fn process_field_attributes(field: &syn::Field) -> FieldAttribute {
    let mut field_attr = FieldAttribute {
        name: field.ident.clone().unwrap(),
        ty: field.ty.clone(),
        is_autoincrement: false,
        is_primary_key: false,
        is_unique: false,
        is_not_null: false,
        check_constraint: None,
        default: None,
        foreign_key: None,
    };

    for attr in &field.attrs {
        if let Some(attr_meta_name) = attr.path().get_ident() {
            if attr_meta_name == "autoincrement" {
                field_attr.is_autoincrement = true;
                continue;
            } else if attr_meta_name == "primary_key" {
                field_attr.is_primary_key = true;
                continue;
            } else if attr_meta_name == "unique" {
                field_attr.is_unique = true;
                continue;
            } else if attr_meta_name == "not_null" {
                field_attr.is_not_null = true;
                continue;
            } else if attr_meta_name == "check" {
                match &attr.meta {
                    Meta::List(list) => {
                        let lit = list.parse_args::<LitStr>().unwrap();
                        field_attr.check_constraint = Some(lit.value());
                    }
                    _ => panic!("Incorrect format for using the `check` attribute."),
                }
            } else if attr_meta_name == "default" {
                match &attr.meta {
                    Meta::List(list) => {
                        let lit = list.parse_args::<LitStr>().unwrap();
                        field_attr.default = Some(lit.value());
                    }
                    _ => panic!("Incorrect format for using the `default` attribute."),
                }
            } else if attr_meta_name == "foreign_key" {
                match &attr.meta {
                    Meta::List(list) => {
                        let lits = list
                            .parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                            .unwrap();
                        let mut parts: Vec<String> = Vec::new();
                        for lit in lits {
                            parts.push(lit.value());
                        }
                        if parts.len() < 2 {
                            panic!("Incorrect format for using the `foreign_key` attribute.");
                        }
                        let ref_table = parts[0].clone();
                        let ref_column = parts[1].clone();
                        let on_delete = if parts.len() > 2 {
                            parts[2].clone()
                        } else {
                            "NO ACTION".to_string()
                        };
                        let on_update = if parts.len() > 3 {
                            parts[3].clone()
                        } else {
                            "NO ACTION".to_string()
                        };
                        field_attr.foreign_key =
                            Some((ref_table, ref_column, on_delete, on_update));
                    }
                    _ => panic!("Incorrect format for using the `foreign_key` attribute."),
                }
            }
        }
    }

    // 返回字段属性
    field_attr
}

/// 生成完整的表实现
fn generate_table_impl(
    struct_name: &syn::Ident,
    fields: &Punctuated<syn::Field, Comma>,
    table_attrs: &[TableAttribute],
    field_attrs: &[FieldAttribute],
) -> TokenStream {
    // 生成表名方法
    let table_name_impl = generate_table_name(struct_name);

    // 生成字段名称方法
    let field_names_impl = generate_field_names(fields);

    // 生成字段类型方法
    let field_types_impl = generate_field_types(fields);

    // 生成创建表 SQL 方法
    let create_table_sql_impl =
        generate_create_table_sql(struct_name, fields, table_attrs, field_attrs);

    // 生成 from_row 方法
    let from_row_impl = generate_from_row_method(struct_name, fields);

    let field_defs = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: #ty }
    });

    let field_defaults = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: <#ty>::default() }
    });

    // 生成迁移SQL
    let migration_impls = generate_migration_impls(struct_name, table_attrs, field_attrs);

    // 生成最终的实现
    quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            #(#field_defs),*
        }

        impl Default for #struct_name {
            fn default() -> Self {
                Self {
                    #(#field_defaults),*
                }
            }
        }

        #from_row_impl

        impl sqlited::WithoutIdTableInfo for #struct_name {
            #table_name_impl
            #field_names_impl
            #field_types_impl
            #create_table_sql_impl
        }

        #migration_impls
    }
    .into()
}

/// 生成表名实现代码
fn generate_table_name(struct_name: &syn::Ident) -> TokenStream2 {
    let snake_case_name = convert_to_snake_name(&struct_name.to_string());

    quote! {
        fn table_name() -> &'static str {
            // 直接使用编译时计算的表名
            #snake_case_name
        }
    }
}

fn generate_field_names(fields: &Punctuated<syn::Field, Comma>) -> TokenStream2 {
    let field_names = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        quote! { #field_name_str }
    });

    quote! {
        fn field_names() -> Vec<&'static str> {
            vec![
                #(#field_names),*
            ]
        }
    }
}

fn generate_field_types(fields: &Punctuated<syn::Field, Comma>) -> TokenStream2 {
    let field_types = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            (#field_name_str, <#field_type as sqlited::SqliteTypeName>::sql_type_name())
        }
    });

    quote! {
        fn field_types() -> Vec<(&'static str, &'static str)> {
            vec![
                #(#field_types),*
            ]
        }
    }
}

/// 生成 from_row 方法以从数据库行创建模型实例
fn generate_from_row_method(
    struct_name: &syn::Ident,
    fields: &Punctuated<syn::Field, Comma>,
) -> TokenStream2 {
    let field_extractions = fields.iter().enumerate().map(|(i, field)| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        quote! {
            #field_name: row.get::<_, #field_type>(#i)?
        }
    });

    quote! {
        impl #struct_name {
            /// Create a new instance from a database row
            pub fn from_row(row: &sqlited::rq::Row) -> sqlited::rq::Result<Self> {
                Ok(Self {
                    #(#field_extractions),*
                })
            }

            pub fn from_rows(rows: &[sqlited::rq::Row]) -> sqlited::rq::Result<Vec<Self>> {
                rows.iter().map(Self::from_row).collect()
            }
        }
    }
}

fn generate_migration_impls(
    struct_name: &syn::Ident,
    table_attrs: &[TableAttribute],
    field_attrs: &[FieldAttribute],
) -> TokenStream2 {
    let table_name = convert_to_snake_name(&struct_name.to_string());
    let migration_methods =
        generate_migration_methods(table_name.as_str(), table_attrs, field_attrs);

    quote! {
        impl #struct_name {
            /// 获取此表的所有迁移SQL语句
            pub fn get_migrations() -> Vec<(String, String, Option<String>)> {
                vec![
                    #(#migration_methods),*
                ]
            }
        }
    }
}

/// 生成每个迁移的方法
fn generate_migration_methods(
    table_name: &str,
    table_attrs: &[TableAttribute],
    field_attrs: &[FieldAttribute],
) -> Vec<TokenStream2> {
    let mut methods = Vec::new();

    for attr in table_attrs {
        if let TableAttributeType::Migration = attr.attr_type {
            if let Some(migration_type) = &attr.migration_type {
                let method = match migration_type {
                    MigrationType::AddColumn => {
                        generate_add_column_migration(table_name, &attr.value, field_attrs)
                    }
                    MigrationType::RenameColumn => {
                        generate_rename_column_migration(table_name, &attr.value)
                    }
                    MigrationType::ModifyColumn => {
                        generate_modify_column_migration(table_name, &attr.value, field_attrs)
                    }
                    MigrationType::DropColumn => {
                        generate_drop_column_migration(table_name, &attr.value)
                    }
                    // MigrationType::AddConstraint => {
                    //     generate_add_constraint_migration(table_name, &attr.value)
                    // },
                    // MigrationType::DropConstraint => {
                    //     generate_drop_constraint_migration(table_name, &attr.value)
                    // },
                    MigrationType::AddIndex => {
                        generate_add_index_migration(table_name, &attr.value)
                    }
                    MigrationType::DropIndex => {
                        generate_drop_index_migration(table_name, &attr.value)
                    }
                    MigrationType::Custom => generate_custom_migration(&attr.value),
                };

                methods.push(method);
            }
        }
    }

    methods
}

/// 生成添加列的迁移
fn generate_add_column_migration(
    table_name: &str,
    args: &[String],
    field_attrs: &[FieldAttribute],
) -> TokenStream2 {
    if args.is_empty() {
        return quote! {
            (
                "error".to_string(),
                "-- Migration add_column requires at least one argument".to_string(),
                None
            )
        };
    }

    let column_name = &args[0];

    // 查找字段定义
    if let Some(field) = field_attrs
        .iter()
        .find(|f| f.name.to_string() == *column_name)
    {
        // 获取字段类型和约束
        let field_type = get_sql_type(&field.ty);
        let mut constraints = Vec::new();

        // 添加各种约束
        if field.is_not_null {
            constraints.push("NOT NULL".to_string());
        }

        if field.is_unique {
            constraints.push("UNIQUE".to_string());
        }

        if let Some(check) = &field.check_constraint {
            constraints.push(format!("CHECK ({})", check));
        }

        if let Some(default) = &field.default {
            constraints.push(format!("DEFAULT {}", default));
        }

        let constraints_str = constraints.join(" ");

        // 生成ALTER TABLE语句
        let alter_sql = format!(
            "ALTER TABLE {} ADD COLUMN {} {} {}",
            table_name, column_name, field_type, constraints_str
        );

        // 生成可能的回滚语句
        let down_sql = if let Some(sqlite_version) = get_sqlite_version() {
            // SQLite 3.35.0+ 支持DROP COLUMN
            if sqlite_version >= (3, 35, 0) {
                quote! {
                    Some(format!("ALTER TABLE {} DROP COLUMN {}", #table_name, #column_name).to_string())
                }
            } else {
                quote! { None }
            }
        } else {
            quote! { None }
        };

        quote! {
            (
                format!("migration_{}_add_{}", #table_name, #column_name),
                #alter_sql.to_string(),
                #down_sql
            )
        }
    } else {
        quote! {
            (
                "error".to_string(),
                format!("-- Column {} not found in struct definition", #column_name),
                None
            )
        }
    }
}

/// 生成重命名列的迁移
fn generate_rename_column_migration(table_name: &str, args: &[String]) -> TokenStream2 {
    if args.len() < 2 {
        return quote! {
            (
                "error".to_string(),
                "-- Migration rename_column requires old_name and new_name".to_string(),
                None
            )
        };
    }

    let old_name = &args[0];
    let new_name = &args[1];

    // SQLite 3.25.0+ 支持 RENAME COLUMN
    let alter_sql = format!(
        "ALTER TABLE {} RENAME COLUMN {} TO {}",
        table_name, old_name, new_name
    );

    let down_sql = format!(
        "ALTER TABLE {} RENAME COLUMN {} TO {}",
        table_name, new_name, old_name
    );

    quote! {
        (
            format!("migration_{}_rename_{}_to_{}", #table_name, #old_name, #new_name),
            #alter_sql.to_string(),
            Some(#down_sql.to_string())
        )
    }
}

/// 生成修改列类型的迁移（使用SQLite 3.35.0+特性）
fn generate_modify_column_migration(
    table_name: &str,
    args: &[String],
    field_attrs: &[FieldAttribute],
) -> TokenStream2 {
    if args.is_empty() {
        return quote! {
            (
                "error".to_string(),
                "-- Migration modify_column requires column name".to_string(),
                None
            )
        };
    }

    let column_name = &args[0];

    // 查找字段定义
    if let Some(field) = field_attrs
        .iter()
        .find(|f| f.name.to_string() == *column_name)
    {
        // 获取字段类型和约束
        let field_type = get_sql_type(&field.ty);
        let mut constraints = Vec::new();

        // 添加各种约束
        if field.is_not_null {
            constraints.push("NOT NULL".to_string());
        }

        if field.is_unique {
            constraints.push("UNIQUE".to_string());
        }

        if let Some(check) = &field.check_constraint {
            constraints.push(format!("CHECK ({})", check));
        }

        if let Some(default) = &field.default {
            constraints.push(format!("DEFAULT {}", default));
        }

        let constraints_str = constraints.join(" ");

        // 使用临时列名
        let temp_column = format!("{}_new", column_name);

        // 检查目标类型是否为布尔
        let is_bool_type = field_type.to_lowercase() == "integer" && is_bool_type(&field.ty);

        // 根据类型使用不同的转换逻辑
        let conversion_expr = if is_bool_type {
            format!(
                "CASE WHEN LOWER({}) IN ('1', 'true', 'yes', 'on', 't', 'y') THEN 1 ELSE 0 END",
                column_name
            )
        } else {
            format!("CAST({} AS {})", column_name, field_type)
        };

        // 构建四步迁移过程（适用于SQLite 3.35.0+）
        let expr = format!(
            "-- SQLite 3.35.0+ column type modification using ADD+DROP+RENAME\n\
             PRAGMA foreign_keys=off;\n\
             \n\
             -- Step 1: Add a new column with the desired type\n\
             ALTER TABLE {} ADD COLUMN {} {} {};\n\
             \n\
             -- Step 2: Copy data with type conversion\n\
             UPDATE {} SET {} = {};\n\
             \n\
             -- Step 3: Drop the old column\n\
             ALTER TABLE {} DROP COLUMN {};\n\
             \n\
             -- Step 4: Rename the new column to the original name\n\
             ALTER TABLE {} RENAME COLUMN {} TO {};\n\
             \n\
             PRAGMA foreign_keys=on;",
            table_name,
            temp_column,
            field_type,
            constraints_str,
            table_name,
            temp_column,
            conversion_expr,
            table_name,
            column_name,
            table_name,
            temp_column,
            column_name
        );

        quote! {
            (
                format!("migration_{}_modify_{}", #table_name, #column_name),
                #expr.to_string(),
                None // 复杂迁移无法提供回滚
            )
        }
    } else {
        quote! {
            (
                "error".to_string(),
                format!("-- Column {} not found in struct definition", #column_name),
                None
            )
        }
    }
}

// 辅助函数：检查类型是否为布尔类型
fn is_bool_type(ty: &syn::Type) -> bool {
    let type_str = quote! { #ty }.to_string().replace(" ", "");
    type_str.contains("bool")
}

/// 生成删除列的迁移
fn generate_drop_column_migration(table_name: &str, args: &[String]) -> TokenStream2 {
    if args.is_empty() {
        return quote! {
            (
                "error".to_string(),
                "-- Migration drop_column requires column name".to_string(),
                None
            )
        };
    }

    let column_name = &args[0];

    // SQLite 3.35.0+ 支持 DROP COLUMN
    let alter_sql = format!("ALTER TABLE {} DROP COLUMN {}", table_name, column_name);

    quote! {
        (
            format!("migration_{}_drop_{}", #table_name, #column_name),
            #alter_sql.to_string(),
            None // 删除的列无法恢复，不提供回滚
        )
    }
}

/// 生成添加约束的迁移
// fn generate_add_constraint_migration(
//     table_name: &str,
//     args: &[String],
// ) -> TokenStream2 {
//     if args.len() < 2 {
//         return quote! {
//             (
//                 "error".to_string(),
//                 "-- Migration add_constraint requires constraint name and definition".to_string(),
//                 None
//             )
//         };
//     }

//     let constraint_name = &args[0];
//     let constraint_def = &args[1];

//     let alter_sql = format!(
//         "ALTER TABLE {} ADD CONSTRAINT {} {}",
//         table_name, constraint_name, constraint_def
//     );

//     let down_sql = format!(
//         "ALTER TABLE {} DROP CONSTRAINT {}",
//         table_name, constraint_name
//     );

//     quote! {
//         (
//             format!("migration_{}_add_constraint_{}", #table_name, #constraint_name),
//             #alter_sql.to_string(),
//             Some(#down_sql.to_string())
//         )
//     }
// }

/// 生成删除约束的迁移
// fn generate_drop_constraint_migration(
//     table_name: &str,
//     args: &[String],
// ) -> TokenStream2 {
//     if args.is_empty() {
//         return quote! {
//             (
//                 "error".to_string(),
//                 "-- Migration drop_constraint requires constraint name".to_string(),
//                 None
//             )
//         };
//     }

//     let constraint_name = &args[0];

//     let alter_sql = format!("ALTER TABLE {} DROP CONSTRAINT {}", table_name, constraint_name);

//     quote! {
//         (
//             format!("migration_{}_drop_constraint_{}", #table_name, #constraint_name),
//             #alter_sql.to_string(),
//             None // 约束定义未保存，不提供回滚
//         )
//     }
// }

/// 生成添加索引的迁移
fn generate_add_index_migration(table_name: &str, args: &[String]) -> TokenStream2 {
    if args.len() < 2 {
        return quote! {
            (
                "error".to_string(),
                "-- Migration add_index requires index name and column(s)".to_string(),
                None
            )
        };
    }

    let index_name = &args[0];
    let columns = &args[1];
    let is_unique = args.len() >= 3 && args[2].to_uppercase() == "UNIQUE";

    let create_sql = if is_unique {
        format!(
            "CREATE UNIQUE INDEX {} ON {} ({})",
            index_name, table_name, columns
        )
    } else {
        format!(
            "CREATE INDEX {} ON {} ({})",
            index_name, table_name, columns
        )
    };

    let down_sql = format!("DROP INDEX {}", index_name);

    quote! {
        (
            format!("migration_{}_add_index_{}", #table_name, #index_name),
            #create_sql.to_string(),
            Some(#down_sql.to_string())
        )
    }
}

/// 生成删除索引的迁移
fn generate_drop_index_migration(table_name: &str, args: &[String]) -> TokenStream2 {
    if args.is_empty() {
        return quote! {
            (
                "error".to_string(),
                "-- Migration drop_index requires index name".to_string(),
                None
            )
        };
    }

    let index_name = &args[0];

    let drop_sql = format!("DROP INDEX {}", index_name);

    quote! {
        (
            format!("migration_{}_drop_index_{}", #table_name, #index_name),
            #drop_sql.to_string(),
            None // 索引定义未保存，不提供回滚
        )
    }
}

/// 生成自定义迁移
fn generate_custom_migration(args: &[String]) -> TokenStream2 {
    if args.len() < 2 {
        return quote! {
            (
                "error".to_string(),
                "-- Migration custom requires name and SQL statement".to_string(),
                None
            )
        };
    }

    let name = &args[0];
    let sql = &args[1];
    let down_sql = if args.len() >= 3 {
        let args2 = &args[2];
        quote! { Some(#args2.to_string()) }
    } else {
        quote! { None }
    };

    quote! {
        (
            #name.to_string(),
            #sql.to_string(),
            #down_sql
        )
    }
}

/// 获取SQLite版本（编译时）
fn get_sqlite_version() -> Option<(u32, u32, u32)> {
    // 这里我们返回一个假设的版本，实际实现可能需要更复杂的逻辑
    // 例如从环境变量或编译标志中获取
    Some((3, 35, 0))
}

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// 从 Rust 类型获取 SQLite 类型
fn get_sql_type(ty: &syn::Type) -> String {
    let type_str = quote! { #ty }.to_string();

    // 移除空格和可能的包装类型
    let type_str = type_str.replace(" ", "");

    // 对常见类型进行映射
    if type_str.contains("String") || type_str.contains("str") {
        "TEXT".to_string()
    } else if type_str.contains("i32")
        || type_str.contains("i16")
        || type_str.contains("i8")
        || type_str.contains("u32")
        || type_str.contains("u16")
        || type_str.contains("u8")
    {
        "INTEGER".to_string()
    } else if type_str.contains("i64") || type_str.contains("u64") {
        "INTEGER".to_string()
    } else if type_str.contains("f32") || type_str.contains("f64") {
        "REAL".to_string()
    } else if type_str.contains("bool") {
        "INTEGER".to_string() // SQLite没有布尔类型，使用INTEGER
    } else if type_str.contains("Vec<u8>") || type_str.contains("&[u8]") {
        "BLOB".to_string()
    } else if type_str.contains("UtcDateTime") {
        "TEXT".to_string() // 日期时间类型存储为TEXT
    } else if type_str.contains("Timestamp") {
        "INTEGER".to_string() // 时间戳存储为INTEGER
    } else if type_str.contains("Option<") {
        // 处理Option类型，递归获取内部类型
        if let syn::Type::Path(type_path) = ty {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Option" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(arg) = args.args.first() {
                            if let syn::GenericArgument::Type(inner_type) = arg {
                                return get_sql_type(inner_type);
                            }
                        }
                    }
                }
            }
        }
        // 默认处理为TEXT
        "TEXT".to_string()
    } else {
        // 对于自定义类型，默认处理为TEXT
        // 也可以考虑在这里添加更多的自定义类型映射
        "TEXT".to_string()
    }
}

fn generate_create_table_sql(
    _struct_name: &syn::Ident,
    fields: &Punctuated<syn::Field, Comma>,
    table_attrs: &[TableAttribute],
    field_attrs: &[FieldAttribute],
) -> TokenStream2 {
    let field_definitions = fields.iter().enumerate().map(|(i, field)| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_type = &field.ty;
        let field_attr = &field_attrs[i];

        let mut constraints = Vec::new();

        // 处理自增主键
        if field_attr.is_autoincrement {
            constraints.push(quote! { " PRIMARY KEY AUTOINCREMENT" });
        } else if field_attr.is_primary_key {
            constraints.push(quote! { " PRIMARY KEY" });
        }

        // 处理唯一约束
        if field_attr.is_unique {
            constraints.push(quote! { " UNIQUE" });
        }
        // 添加 NOT NULL 约束
        if !field_attr.is_autoincrement && !field_attr.is_primary_key {
            // Check if the type is Option<T>
            let is_option = is_option_type(field_type);
            
            // 处理非空约束
            if field_attr.is_not_null || !is_option {
                constraints.push(quote! { " NOT NULL" });
            } else {
                // 获取 SQLite 类型名称
                let sqlite_type = quote! {
                    <#field_type as sqlited::SqliteTypeName>::sql_type_name()
                }
                .to_string();

                // 检查是否为 Option 或 BLOB
                if is_option && sqlite_type != "\"BLOB\"" {
                    constraints.push(quote! { " NULL" });
                }
            }
        }

        // 处理默认值
        if let Some(default) = &field_attr.default {
            let default_val = default.clone();
            let type_str = quote! { #field_type }.to_string();

            // 特殊处理 "now" 默认值
            if default_val == "now" {
                if type_str.contains("UtcDateTime") {
                    constraints.push(quote! { " DEFAULT CURRENT_TIMESTAMP" });
                } else if type_str.contains("Timestamp") {
                    constraints.push(quote! { " DEFAULT (strftime('%s','now'))" });
                } else {
                    constraints.push(quote! { " DEFAULT CURRENT_TIMESTAMP" });
                }
            }
            // 特殊处理布尔值
            else if (default_val == "1" || default_val == "true") && type_str.contains("bool") {
                constraints.push(quote! { " DEFAULT 1" });
            } else if (default_val == "0" || default_val == "false") && type_str.contains("bool") {
                constraints.push(quote! { " DEFAULT 0" });
            }
            // 其他情况
            else {
                constraints.push(quote! {
                    format!(" DEFAULT {}", #default_val)
                });
            }
        }

        // 处理检查约束
        if let Some(check) = &field_attr.check_constraint {
            let check_expr = check.clone();
            constraints.push(quote! { format!(" CHECK({})", #check_expr) });
        }

        // 处理外键约束
        if let Some((ref_table, ref_column, on_delete, on_update)) = &field_attr.foreign_key {
            constraints.push(quote! {
                format!(" REFERENCES {}({}) ON DELETE {} ON UPDATE {}",
                    #ref_table, #ref_column, #on_delete, #on_update)
            });
        }

        // 组合所有约束
        quote! {
            let mut field_constraints = String::new();
            #(field_constraints.push_str(&#constraints);)*

            sql.push_str(&format!("    {} {}{},\n",
                #field_name_str,
                <#field_type as sqlited::SqliteTypeName>::sql_type_name(),
                field_constraints
            ));
        }
    });

    // 处理表级约束
    let table_constraints = table_attrs.iter().filter_map(|attr| match attr.attr_type {
        TableAttributeType::Constraint => {
            let constraint = &attr.value[0];
            Some(quote! {
                sql.push_str(&format!("    {},\n", #constraint));
            })
        }
        _ => None,
    });

    // 处理索引
    let indices = table_attrs.iter().filter_map(|attr| {
        match attr.attr_type {
            TableAttributeType::Index => {
                if attr.value.len() >= 2 {
                    let idx_name = &attr.value[0];
                    let idx_columns = &attr.value[1];
                    Some(quote! {
                        // 使用直接的字符串拼接而不是格式化
                        indexes.push_str("CREATE INDEX IF NOT EXISTS ");
                        indexes.push_str(#idx_name);
                        indexes.push_str(" ON ");
                        indexes.push_str(Self::table_name());
                        indexes.push_str(" (");
                        indexes.push_str(#idx_columns);
                        indexes.push_str(");\n");
                    })
                } else {
                    println!("Index attribute has insufficient values: {:?}", attr.value);
                    None
                }
            }
            TableAttributeType::UniqueIndex => {
                if attr.value.len() >= 2 {
                    let idx_name = &attr.value[0];
                    let idx_columns = &attr.value[1];
                    Some(quote! {
                        indexes.push_str(&format!(
                            "CREATE UNIQUE INDEX IF NOT EXISTS {} ON {} ({});\n",
                            #idx_name,
                            Self::table_name(),
                            #idx_columns
                        ));
                    })
                } else {
                    println!(
                        "Unique index attribute has insufficient values: {:?}",
                        attr.value
                    );
                    None
                }
            }
            _ => None,
        }
    });

    quote! {
        fn create_table_sql() -> String {
            let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (\n", Self::table_name());

            // 添加字段定义
            #(#field_definitions)*

            // 添加表级约束
            #(#table_constraints)*

            // 移除最后的逗号和换行符
            if sql.ends_with(",\n") {
                sql.pop();
                sql.pop();
                sql.push_str("\n");
            }

            sql.push_str(")");

            // 处理索引
            let mut indexes = String::new();
            #(#indices)*

            if !indexes.is_empty() {
                sql.push_str(";\n");
                sql.push_str(&indexes);
            }

            sql
        }
    }
}
