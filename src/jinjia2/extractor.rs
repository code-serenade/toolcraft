use regex::Regex;

/// 提取 Jinja2 标签（按顺序，支持 {{ }} 和 {% ... %} 含非标准 tag）
pub fn extract_jinja2_tags(input: &str) -> Vec<String> {
    let tag_re = Regex::new(r"(?s)\{\{.*?\}\}|\{%-?.*?-?%\}").unwrap();

    tag_re
        .find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect()
}
