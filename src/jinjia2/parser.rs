#[derive(Debug)]
pub enum JinjaNode {
    Variable {
        path: Vec<String>,
    },
    ForLoop {
        loop_var: String,
        iterable: String,
        body: Vec<JinjaNode>,
    },
}

pub(crate) fn parse_jinja2_ast(tags: &[String]) -> Vec<JinjaNode> {
    let mut ast = Vec::new();
    let mut stack: Vec<JinjaNode> = Vec::new();

    for tag in tags {
        let tag = tag.trim();

        if tag.starts_with("{{") && tag.ends_with("}}") {
            let content = tag.trim_start_matches("{{").trim_end_matches("}}").trim();
            let path = content.split('.').map(|s| s.trim().to_string()).collect();
            let node = JinjaNode::Variable { path };

            if let Some(JinjaNode::ForLoop { body, .. }) = stack.last_mut() {
                body.push(node);
            } else {
                ast.push(node);
            }
        } else if tag.starts_with("{%") && tag.contains("for") && tag.contains("in") {
            println!("Found for loop tag: {}", tag);
            let content = tag.trim_start_matches("{%").trim_end_matches("%}").trim();
            let parts: Vec<&str> = content.split_whitespace().collect();
            if let Some(for_index) = parts.iter().position(|&s| s == "for") {
                if for_index + 2 < parts.len() && parts[for_index + 2] == "in" {
                    let loop_var = parts
                        .get(for_index + 1)
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    let iterable = parts.get(for_index + 3).to_string_or_empty();
                    stack.push(JinjaNode::ForLoop {
                        loop_var,
                        iterable,
                        body: Vec::new(),
                    });
                }
            }
        } else if tag.contains("endfor") {
            if let Some(for_node) = stack.pop() {
                if let Some(JinjaNode::ForLoop { body, .. }) = stack.last_mut() {
                    body.push(for_node);
                } else {
                    ast.push(for_node);
                }
            }
        }
    }

    ast
}

// Helper trait for safe string extraction
trait StringVecExt {
    fn to_string_or_empty(&self) -> String;
}
impl StringVecExt for Option<&&str> {
    fn to_string_or_empty(&self) -> String {
        self.map(|s| s.to_string()).unwrap_or_default()
    }
}
