pub mod docx;
pub mod extractor;
pub mod parser;
pub mod schema;

pub fn docx_to_json(docx_path: &str) -> crate::error::Result<serde_json::Value> {
    let text = docx::extract_text_from_docx(docx_path)?;
    let jinja2_tags = extractor::extract_jinja2_tags(&text);
    let ast = parser::parse_jinja2_ast(&jinja2_tags);
    Ok(schema::ast_to_json(&ast))
}
