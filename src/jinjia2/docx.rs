use std::{fs::File, io::Read, path::Path};

use quick_xml::{Reader, events::Event};
use zip::ZipArchive;

use crate::error::Result;

pub(crate) fn extract_text_from_docx<P: AsRef<Path>>(docx_path: P) -> Result<String> {
    let file = File::open(docx_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut xml = String::new();

    archive
        .by_name("word/document.xml")?
        .read_to_string(&mut xml)?;

    let mut text = String::new();
    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    while let Ok(evt) = reader.read_event_into(&mut buf) {
        match evt {
            Event::Text(e) => text.push_str(&e.decode().unwrap().to_string()),
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }

    Ok(text)
}
