use docx_rs::{DocumentChild, Docx};

// use serde_json::{Value, json};
use crate::error::Result;

#[derive(Debug)]
pub struct Node {
    pub level: usize,
    pub title: String,
    pub contents: Option<Vec<String>>,
    pub children: Vec<Node>,
}

fn insert_node(tree: &mut Vec<Node>, node: Node, path_stack: &mut Vec<*mut Node>) {
    while let Some(&last_ptr) = path_stack.last() {
        let last = unsafe { &mut *last_ptr };
        if node.level > last.level {
            last.children.push(node);
            let last_child = last.children.last_mut().unwrap();
            path_stack.push(last_child as *mut _);
            return;
        } else {
            path_stack.pop();
        }
    }

    tree.push(node);
    let last = tree.last_mut().unwrap();
    path_stack.push(last as *mut _);
}

// fn nodes_to_json(nodes: &[Node]) -> Value {
//     Value::Array(
//         nodes
//             .iter()
//             .map(|n| {
//                 json!({
//                     "title": n.title,
//                     "level": n.level,
//                     "children": nodes_to_json(&n.children)
//                 })
//             })
//             .collect(),
//     )
// }

pub fn extract_docx_headings(doc: Docx) -> Result<Vec<Node>> {
    let mut tree: Vec<Node> = Vec::new();
    let mut path_stack: Vec<*mut Node> = Vec::new();

    for para in &doc.document.children {
        if let DocumentChild::Paragraph(p) = para {
            if let Some(style) = &p.property.style {
                if let Ok(level) = style.val.parse::<usize>() {
                    let node = Node {
                        level,
                        title: p.raw_text(),
                        contents: None,
                        children: vec![],
                    };
                    insert_node(&mut tree, node, &mut path_stack);
                }
            } else {
                if let Some(&last_ptr) = path_stack.last() {
                    let last = unsafe { &mut *last_ptr };
                    if last.contents.is_none() {
                        last.contents = Some(Vec::new());
                    }
                    if let Some(ref mut contents) = last.contents {
                        contents.push(p.raw_text());
                    }
                } else {
                    // 没有任何标题节点，无法归属内容
                    println!("Orphan content (no parent node): {}", p.raw_text());
                }
            }
        }
    }

    Ok(tree)
}
