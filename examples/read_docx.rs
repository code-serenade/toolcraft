use toolcraft::{self, error::Result, jinjia2::docx_to_json};

#[tokio::main]
async fn main() -> Result<()> {
    let json = docx_to_json("data/tt.docx")?;
    println!("Generated JSON: {}", json);

    Ok(())
}
