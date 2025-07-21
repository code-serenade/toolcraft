use docx_rs::*;
use toolcraft::docx::scan::extract_docx_headings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 打开 docx 文件
    let buf = std::fs::read("./data/sample.docx")?;

    // 读取 docx 文档
    let doc = read_docx(&buf)?;

    let headings = extract_docx_headings(doc)?;
    println!("{:#?}", headings);

    Ok(())
}
