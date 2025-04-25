use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parenthesized, parse_macro_input, Attribute, Data, DeriveInput, Fields, LitStr, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// 表级属性
struct TableAttribute {
    attr_type: TableAttributeType,
    value: Vec<String>,
}

enum TableAttributeType {
    Constraint,
    Index,
    UniqueIndex,
}

/// 字段属性
struct FieldAttribute {
    name: syn::Ident,
    ty: syn::Type,
    is_autoincrement: bool,
    is_primary_key: bool,
    is_unique: bool,
    is_not_null: bool,
    check_constraint: Option<String>,
    default_value: Option<String>,
    foreign_key: Option<(String, String, String, String)>, // (table, column, on_delete, on_update)
}


/// 解析表结构并生成完整的表实现
pub fn table(input: TokenStream) -> TokenStream {
    // 解析输入为 struct 定义
    let input = parse_macro_input!(input as DeriveInput);

    // println!("Processing table: {} attrs: {:?}", input.ident, input.attrs);
    
    // 获取结构体名称
    let struct_name = &input.ident;
    let struct_name_str = struct_name.to_string();
    
    // 获取结构体字段
    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => fields,
                _ => {
                    return syn::Error::new_spanned(
                        &input,
                        "表结构必须使用命名字段 (named fields)"
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(
                &input,
                "table 宏只能用于结构体"
            )
            .to_compile_error()
            .into();
        }
    };
    
    // 处理表级属性 (如 constraint, index)
    let table_attributes = process_table_attributes(&input.attrs);
    
    // 处理字段和它们的属性
    let field_attributes = fields.named.iter().map(|field| {
        process_field_attributes(field)
    }).collect::<Vec<_>>();
    
    // 生成表信息实现
    generate_table_impl(struct_name, &fields.named, &table_attributes, &field_attributes)
}

/// 处理表级属性
fn process_table_attributes(attrs: &[Attribute]) -> Vec<TableAttribute> {
    let mut table_attrs = Vec::new();
  
    // 1. 直接检查每个属性的路径
    for attr in attrs {
        if let Some(attr_meta_name) = attr.path().get_ident() {

            let attr_name = attr_meta_name.to_string();
            
            // 打印调试信息
            // println!("Processing attribute: {}", attr_name);


            if attr_name == "constraint" {
                // 处理表级约束
                match &attr.meta {
                    Meta::List(list) => {
                        if let Ok(lit) = list.parse_args::<LitStr>() {
                            // println!("Found constraint: {}", lit.value());
                            table_attrs.push(TableAttribute {
                                attr_type: TableAttributeType::Constraint,
                                value: vec![lit.value()],
                            });
                        }
                    },
                    _ => panic!("Incorrect format for using the `constraint` attribute."),
                }
            } else if attr_name == "index" {
                // 处理普通索引
                match &attr.meta {
                    Meta::List(list) => {
                        if let Ok(args) = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated) {
                            if args.len() >= 2 {
                                let idx_name = args[0].value();
                                let idx_columns = args[1].value();
                                // println!("Found index: {} -> {}", idx_name, idx_columns);
                                table_attrs.push(TableAttribute {
                                    attr_type: TableAttributeType::Index,
                                    value: vec![idx_name, idx_columns],
                                });
                            }
                        }
                    },
                    _ => panic!("Incorrect format for using the `index` attribute."),
                }
            } else if attr_meta_name == "unique_index" {
                // 处理唯一索引
                match &attr.meta {
                    Meta::List(list) => {
                        if let Ok(args) = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated) {
                            if args.len() >= 2 {
                                let idx_name = args[0].value();
                                let idx_columns = args[1].value();
                                // println!("Found unique index: {} -> {}", idx_name, idx_columns);
                                table_attrs.push(TableAttribute {
                                    attr_type: TableAttributeType::UniqueIndex,
                                    value: vec![idx_name, idx_columns],
                                });
                            }
                        }
                    },
                    _ => panic!("Incorrect format for using the `unique_index` attribute."),
                }
            }
        }
    }

    // 打印找到的所有表级属性
    // println!("Found {} table attributes", table_attrs.len());
    // for (i, attr) in table_attrs.iter().enumerate() {
    //     let type_str = match attr.attr_type {
    //         TableAttributeType::Constraint => "Constraint",
    //         TableAttributeType::Index => "Index",
    //         TableAttributeType::UniqueIndex => "UniqueIndex",
    //     };
    //     println!("Attribute {}: {:?} with values: {:?}", i, type_str, attr.value);
    // }
  
    table_attrs
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
        default_value: None,
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
                    },
                    _ => panic!("Incorrect format for using the `check` attribute."),
                }
            } else if attr_meta_name == "default_value" {
                match &attr.meta {
                  Meta::List(list) => {
                      let lit = list.parse_args::<LitStr>().unwrap();
                      field_attr.default_value = Some(lit.value());
                  },
                    _ => panic!("Incorrect format for using the `default_value` attribute."),
                }
            } else if attr_meta_name == "foreign_key" {
                match &attr.meta {
                    Meta::List(list) => {
                        let lits = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated).unwrap();
                        let mut parts: Vec<String> = Vec::new();
                        for lit in lits {
                            parts.push(lit.value());
                        }
                        if parts.len() < 2 {
                            panic!("Incorrect format for using the `foreign_key` attribute.");
                        }
                        let ref_table = parts[0].clone();
                        let ref_column = parts[1].clone();
                        let on_delete = if parts.len() > 2 { parts[2].clone() } else { "NO ACTION".to_string() };
                        let on_update = if parts.len() > 3 { parts[3].clone() } else { "NO ACTION".to_string() };
                        field_attr.foreign_key = Some((ref_table, ref_column, on_delete, on_update));
                    },
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
    field_attrs: &[FieldAttribute]
) -> TokenStream {
    // 生成表名方法
    let table_name_impl = generate_table_name(struct_name);
    
    // 生成字段名称方法
    let field_names_impl = generate_field_names(fields);
    
    // 生成字段类型方法
    let field_types_impl = generate_field_types(fields);
    
    // 生成创建表 SQL 方法
    let create_table_sql_impl = generate_create_table_sql(struct_name, fields, table_attrs, field_attrs);

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
        
        impl sqlited::WithoutIdTableInfo for #struct_name {
            #table_name_impl
            #field_names_impl
            #field_types_impl
            #create_table_sql_impl
        }
    }.into()
}

// 添加以下函数

fn generate_table_name(struct_name: &syn::Ident) -> TokenStream2 {
    let struct_name_str = struct_name.to_string();
    quote! {
        fn table_name() -> &'static str {
            static TABLE_NAME: &str = #struct_name_str;
            use std::sync::LazyLock;
            static SNAKE_NAME: LazyLock<String> = LazyLock::new(|| {
                // 将驼峰命名法转换为蛇形命名法
                let mut result = String::new();
                let chars: Vec<char> = TABLE_NAME.chars().collect();
                
                for (i, &c) in chars.iter().enumerate() {
                    if c.is_uppercase() {
                        // 不是首字母且是大写，添加下划线
                        if i > 0 {
                            result.push('_');
                        }
                        result.push(c.to_lowercase().next().unwrap());
                    } else {
                        result.push(c);
                    }
                }
                
                result.to_lowercase()
            });
            
            // 泄露一次字符串，使其具有 'static 生命周期
            Box::leak(SNAKE_NAME.clone().into_boxed_str())
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

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn generate_create_table_sql(
  struct_name: &syn::Ident,
  fields: &Punctuated<syn::Field, Comma>,
  table_attrs: &[TableAttribute],
  field_attrs: &[FieldAttribute]
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
          // 处理非空约束
          if field_attr.is_not_null {
              constraints.push(quote! { " NOT NULL" });
          } else {
              // 获取 SQLite 类型名称
              let sqlite_type = quote! {
                  <#field_type as sqlited::SqliteTypeName>::sql_type_name()
              }.to_string();

              // 检查是否为 Option 或 BLOB
              if is_option_type(field_type) && sqlite_type != "\"BLOB\"" {
                  constraints.push(quote! { " NULL" });
              }
          }
      }
      
      // 处理默认值
      if let Some(default_value) = &field_attr.default_value {
          let default_val = default_value.clone();
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
          }
          else if (default_val == "0" || default_val == "false") && type_str.contains("bool") {
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
  let table_constraints = table_attrs.iter().filter_map(|attr| {
      match attr.attr_type {
          TableAttributeType::Constraint => {
              let constraint = &attr.value[0];
              Some(quote! {
                  sql.push_str(&format!("    {},\n", #constraint));
              })
          },
          _ => None,
      }
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
            },
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
                    println!("Unique index attribute has insufficient values: {:?}", attr.value);
                    None
                }
            },
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