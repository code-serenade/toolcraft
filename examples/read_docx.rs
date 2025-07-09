use toolcraft::{
    error::Result,
    jinjia2::{
        docx::extract_text_from_docx, extractor::extract_jinja2_tags, parser::parse_jinja2_ast,
        schema::ast_to_json,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let text = extract_text_from_docx("data/tt.docx")?;
    // println!("Extracted text: {}", text);

    let jinja2_tags = extract_jinja2_tags(&text);
    // println!("Found Jinja2 tags: {:?}", jinja2_tags);

    let ast = parse_jinja2_ast(&jinja2_tags);
    // println!("Parsed Jinja2 AST: {:#?}", ast);

    let json = ast_to_json(&ast);
    println!("Generated JSON: {}", json);

    Ok(())
}
