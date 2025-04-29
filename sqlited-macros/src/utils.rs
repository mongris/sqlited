/// 添加辅助函数：找到最相似的迁移类型（拼写检查）
pub(crate) fn find_closest_match<'a>(input: &str, valid_types: &'a [&'a str]) -> Option<&'a str> {
  valid_types.iter()
      .map(|valid| (*valid, levenshtein_distance(input, valid)))
      .min_by_key(|(_, distance)| *distance)
      .filter(|(_, distance)| *distance <= 3) // 最多允许3个字符的差异
      .map(|(valid, _)| valid)
}


/// 辅助函数：计算两个字符串的编辑距离
fn levenshtein_distance(a: &str, b: &str) -> usize {
  let a_len = a.chars().count();
  let b_len = b.chars().count();
  
  // 边界情况处理
  if a_len == 0 { return b_len; }
  if b_len == 0 { return a_len; }
  if a == b { return 0; }
  
  let a: Vec<char> = a.chars().collect();
  let b: Vec<char> = b.chars().collect();
  
  // 创建距离矩阵
  let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
  
  // 初始化第一行和第一列
  for i in 0..=a_len { matrix[i][0] = i; }
  for j in 0..=b_len { matrix[0][j] = j; }
  
  // 填充矩阵
  for i in 1..=a_len {
      for j in 1..=b_len {
          let cost = if a[i-1] == b[j-1] { 0 } else { 1 };
          
          matrix[i][j] = std::cmp::min(
              matrix[i-1][j] + 1,          // 删除
              std::cmp::min(
                  matrix[i][j-1] + 1,      // 插入
                  matrix[i-1][j-1] + cost  // 替换
              )
          );
      }
  }
  
  matrix[a_len][b_len]
}


/// 获取蛇形命名法
pub(crate) fn convert_to_snake_name(struct_name: &str) -> String {
    let struct_name_str = struct_name.to_string();

    // 将驼峰命名转换为蛇形命名
    let mut result = String::new();
    let chars: Vec<char> = struct_name_str.chars().collect();

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
}