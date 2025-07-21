use std::{fs, fs::File, io::BufWriter};

use docx_rs::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct TableStyle {
    font_size: Option<u32>,
    bold_headers: Option<bool>,
}

#[derive(Deserialize)]
struct TableData {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    style: Option<TableStyle>,
}

#[derive(Deserialize)]
struct Input {
    table: TableData,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string("data/input.json")?;
    let input: Input = serde_json::from_str(&json_str)?;

    let headers = input.table.headers;
    let rows = input.table.rows;
    let style = input.table.style.unwrap_or(TableStyle {
        font_size: Some(22),
        bold_headers: Some(true),
    });
    let font_size = style.font_size.unwrap_or(22);
    let bold_headers = style.bold_headers.unwrap_or(true);

    let mut table = Table::new(vec![])
        .set_grid(vec![2000, 2000, 2000])
        .indent(1000);

    // 表头（加粗）
    let header_row = TableRow::new(
        headers
            .into_iter()
            .map(|h| {
                let mut run = Run::new().add_text(h).size(font_size as usize);
                if bold_headers {
                    run = run.bold();
                }
                TableCell::new().add_paragraph(Paragraph::new().add_run(run))
            })
            .collect(),
    );
    table = table.add_row(header_row);

    // 表体
    for row in rows {
        let table_row = TableRow::new(
            row.into_iter()
                .map(|cell| {
                    TableCell::new().add_paragraph(
                        Paragraph::new()
                            .add_run(Run::new().add_text(cell).size(font_size as usize)),
                    )
                })
                .collect(),
        );
        table = table.add_row(table_row);
    }

    // 构建文档
    let doc = Docx::new().add_table(table);

    // 输出 Word 文件
    let file = File::create("./output/test.docx")?;
    let writer = BufWriter::new(file);
    doc.build().pack(writer)?;

    println!("✅ Word 表格已生成: output.docx");
    Ok(())
}
