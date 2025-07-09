use regex::Regex;

pub(crate) fn extract_jinja2_tags(input: &str) -> Vec<String> {
    let tag_re = Regex::new(r"(?s)\{\{.*?\}\}|\{%-?.*?-?%\}").unwrap();

    tag_re
        .find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect()
}
