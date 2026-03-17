use crate::error::{Error, Result};

pub struct ObjectInfo {
    pub key: String,
    pub size: u64,
    pub last_modified: String,
}

pub(crate) async fn check_status(resp: reqwest::Response) -> Result<reqwest::Response> {
    let status = resp.status();
    if status.is_success() || status == reqwest::StatusCode::NO_CONTENT {
        return Ok(resp);
    }
    let code = status.as_u16();
    let body = resp.text().await.unwrap_or_default();
    let message = extract_tag_values(&body, "Message")
        .into_iter()
        .next()
        .unwrap_or(body);
    Err(Error::S3 { status: code, message })
}

pub(crate) fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

pub(crate) fn extract_tag_values(xml: &str, tag: &str) -> Vec<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut values = Vec::new();
    let mut pos = 0;
    while let Some(start) = xml[pos..].find(&open) {
        let content_start = pos + start + open.len();
        if let Some(end) = xml[content_start..].find(&close) {
            values.push(xml[content_start..content_start + end].to_string());
            pos = content_start + end + close.len();
        } else {
            break;
        }
    }
    values
}

pub(crate) fn parse_bucket_names(xml: &str) -> Result<Vec<String>> {
    let buckets = extract_tag_values(xml, "Bucket");
    Ok(buckets
        .into_iter()
        .filter_map(|b| extract_tag_values(&b, "Name").into_iter().next())
        .collect())
}

pub(crate) fn parse_object_list(xml: &str) -> Result<Vec<ObjectInfo>> {
    let contents = extract_tag_values(xml, "Contents");
    Ok(contents
        .into_iter()
        .map(|block| ObjectInfo {
            key: extract_tag_values(&block, "Key")
                .into_iter()
                .next()
                .unwrap_or_default(),
            size: extract_tag_values(&block, "Size")
                .into_iter()
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            last_modified: extract_tag_values(&block, "LastModified")
                .into_iter()
                .next()
                .unwrap_or_default(),
        })
        .collect())
}
