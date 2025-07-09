use serde_json::{Map, Value, json};

use crate::jinjia2::parser::JinjaNode;

pub(crate) fn ast_to_json(ast: &[JinjaNode]) -> Value {
    let mut root = Map::new();

    for node in ast {
        merge_value(&mut root, node);
    }

    Value::Object(root)
}

fn merge_value(target: &mut Map<String, Value>, node: &JinjaNode) {
    match node {
        JinjaNode::Variable { path } => {
            insert_path(target, path);
        }
        JinjaNode::ForLoop { iterable, body, .. } => {
            let mut item_obj = Map::new();
            for item in body {
                merge_value(&mut item_obj, item);
            }
            target.insert(
                iterable.clone(),
                Value::Array(vec![Value::Object(item_obj)]),
            );
        }
    }
}

fn insert_path(target: &mut Map<String, Value>, path: &[String]) {
    if path.is_empty() {
        return;
    }

    let mut current = target;

    for (i, key) in path.iter().enumerate() {
        if i == path.len() - 1 {
            current.insert(key.clone(), json!(""));
        } else {
            current = current
                .entry(key)
                .or_insert_with(|| Value::Object(Map::new()))
                .as_object_mut()
                .unwrap();
        }
    }
}
