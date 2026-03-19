use base64::{Engine as _, engine::general_purpose};
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use serde_json::{Value, json};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Default presigned URL expiry: 15 minutes (matches AWS SDK v2/v3 and boto3 defaults).
pub const PRESIGN_DEFAULT_EXPIRES_SECS: u64 = 900;

/// Default region for S3-compatible services that have no real region (MinIO, RustFS, SeaweedFS,
/// etc.).
pub const DEFAULT_REGION: &str = "us-east-1";

// ── Public API ───────────────────────────────────────────────────────────────

/// SigV4 Authorization headers for a standard (non-presigned) S3 request.
pub struct S3AuthHeaders {
    pub authorization: String,
    pub x_amz_date: String,
    pub x_amz_content_sha256: String,
}

/// Build SigV4 Authorization headers for standard S3 API calls (create/delete bucket, list, etc.).
///
/// `query` must be the canonical query string: keys sorted by ASCII order, already percent-encoded.
#[must_use]
pub fn sign_request(
    method: &str,
    access_key: &str,
    secret_key: &str,
    host: &str,
    path: &str,
    query: &str,
    region: Option<&str>,
) -> S3AuthHeaders {
    let region = region.unwrap_or(DEFAULT_REGION);
    let now = Utc::now();
    let date_stamp = now.format("%Y%m%d").to_string();
    let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();

    // Canonical headers must be sorted alphabetically by header name
    let canonical_headers =
        format!("host:{host}\nx-amz-content-sha256:UNSIGNED-PAYLOAD\nx-amz-date:{amz_date}\n");
    let signed_headers = "host;x-amz-content-sha256;x-amz-date";

    let canonical_request = format!(
        "{method}\n{path}\n{query}\n{canonical_headers}\n{signed_headers}\nUNSIGNED-PAYLOAD"
    );

    let credential_scope = format!("{date_stamp}/{region}/s3/aws4_request");
    let string_to_sign = format!(
        "AWS4-HMAC-SHA256\n{amz_date}\n{credential_scope}\n{}",
        sha256_hex(canonical_request.as_bytes())
    );

    let signing_key = derive_signing_key(secret_key, &date_stamp, region);
    let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

    S3AuthHeaders {
        authorization: format!(
            "AWS4-HMAC-SHA256 Credential={access_key}/{credential_scope}, \
             SignedHeaders={signed_headers}, Signature={signature}"
        ),
        x_amz_date: amz_date,
        x_amz_content_sha256: "UNSIGNED-PAYLOAD".to_string(),
    }
}

#[must_use]
pub fn generate_s3_post_policy(
    access_key: &str,
    secret_key: &str,
    bucket: &str,
    key_prefix: &str,
    region: Option<&str>,
    endpoint: &str,
    max_file_mb: u64,
) -> Value {
    let region = region.unwrap_or(DEFAULT_REGION);
    let now = Utc::now();
    let expiration = (now + Duration::minutes(10)).to_rfc3339();
    let date_stamp = now.format("%Y%m%d").to_string();
    let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
    let credential = format!("{access_key}/{date_stamp}/{region}/s3/aws4_request");

    let policy = json!({
        "expiration": expiration,
        "conditions": [
            {"bucket": bucket},
            ["starts-with", "$key", key_prefix],
            {"x-amz-algorithm": "AWS4-HMAC-SHA256"},
            {"x-amz-credential": credential},
            {"x-amz-date": amz_date},
            ["content-length-range", 1, max_file_mb * 1024 * 1024]
        ]
    });

    let policy_base64 = general_purpose::STANDARD.encode(policy.to_string().as_bytes());
    let signing_key = derive_signing_key(secret_key, &date_stamp, region);
    let signature = hex::encode(hmac_sha256(&signing_key, policy_base64.as_bytes()));

    json!({
        "url": format!("{}/{bucket}", endpoint.trim_end_matches('/')),
        "fields": {
            "key": format!("{key_prefix}${{filename}}"),
            "policy": policy_base64,
            "x-amz-algorithm": "AWS4-HMAC-SHA256",
            "x-amz-credential": credential,
            "x-amz-date": amz_date,
            "x-amz-signature": signature
        }
    })
}

#[must_use]
pub fn presign_get_object(
    access_key: &str,
    secret_key: &str,
    bucket: &str,
    key: &str,
    region: Option<&str>,
    endpoint: &str,
    expires_secs: Option<u64>,
) -> String {
    presign_object(
        "GET",
        access_key,
        secret_key,
        bucket,
        key,
        region.unwrap_or(DEFAULT_REGION),
        endpoint,
        expires_secs.unwrap_or(PRESIGN_DEFAULT_EXPIRES_SECS),
    )
}

#[must_use]
pub fn presign_put_object(
    access_key: &str,
    secret_key: &str,
    bucket: &str,
    key: &str,
    region: Option<&str>,
    endpoint: &str,
    expires_secs: Option<u64>,
) -> String {
    presign_object(
        "PUT",
        access_key,
        secret_key,
        bucket,
        key,
        region.unwrap_or(DEFAULT_REGION),
        endpoint,
        expires_secs.unwrap_or(PRESIGN_DEFAULT_EXPIRES_SECS),
    )
}

// ── Internal ─────────────────────────────────────────────────────────────────

// Core presign implementation shared by GET and PUT
fn presign_object(
    method: &str,
    access_key: &str,
    secret_key: &str,
    bucket: &str,
    key: &str,
    region: &str,
    endpoint: &str,
    expires_secs: u64,
) -> String {
    let now = Utc::now();
    let date_stamp = now.format("%Y%m%d").to_string();
    let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();

    let host = extract_host(endpoint);
    let canonical_uri = format!("/{bucket}/{}", key.trim_start_matches('/'));
    let credential = format!("{access_key}/{date_stamp}/{region}/s3/aws4_request");

    // Query params sorted by ASCII order (required by SigV4)
    let mut query_params = vec![
        ("X-Amz-Algorithm", "AWS4-HMAC-SHA256".to_string()),
        ("X-Amz-Credential", credential),
        ("X-Amz-Date", amz_date.clone()),
        ("X-Amz-Expires", expires_secs.to_string()),
        ("X-Amz-SignedHeaders", "host".to_string()),
    ];
    query_params.sort_by_key(|(k, _)| *k);

    let canonical_query_string = query_params
        .iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let canonical_request = format!(
        "{method}\n{canonical_uri}\n{canonical_query_string}\nhost:{host}\n\nhost\\
         nUNSIGNED-PAYLOAD"
    );

    let string_to_sign = format!(
        "AWS4-HMAC-SHA256\n{amz_date}\n{date_stamp}/{region}/s3/aws4_request\n{}",
        sha256_hex(canonical_request.as_bytes())
    );

    let signing_key = derive_signing_key(secret_key, &date_stamp, region);
    let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

    format!(
        "{}{canonical_uri}?{canonical_query_string}&X-Amz-Signature={signature}",
        endpoint.trim_end_matches('/')
    )
}

// Derive signing key: AWS4+secret → date → region → s3 → aws4_request
fn derive_signing_key(secret_key: &str, date_stamp: &str, region: &str) -> Vec<u8> {
    let date_key = hmac_sha256(
        format!("AWS4{secret_key}").as_bytes(),
        date_stamp.as_bytes(),
    );
    let date_region_key = hmac_sha256(&date_key, region.as_bytes());
    let date_region_service_key = hmac_sha256(&date_region_key, b"s3");
    hmac_sha256(&date_region_service_key, b"aws4_request")
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

fn sha256_hex(data: &[u8]) -> String {
    use sha2::Digest;
    hex::encode(sha2::Sha256::digest(data))
}

// Strip scheme and trailing slash to extract bare host
fn extract_host(endpoint: &str) -> String {
    endpoint
        .trim_end_matches('/')
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .to_string()
}

// Percent-encode per AWS SigV4 spec (unreserved chars pass through)
fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A' ..= b'Z' | b'a' ..= b'z' | b'0' ..= b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}
