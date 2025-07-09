use base64::{Engine as _, engine::general_purpose};
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use serde::Serialize;
use serde_json::json;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize)]
pub struct PostPolicyResponse {
    pub url: String,
    pub fields: PostPolicyFields,
}

#[derive(Debug, Serialize)]
pub struct PostPolicyFields {
    pub key: String,
    pub policy: String,
    #[serde(rename = "x-amz-algorithm")]
    pub x_amz_algorithm: String,
    #[serde(rename = "x-amz-credential")]
    pub x_amz_credential: String,
    #[serde(rename = "x-amz-date")]
    pub x_amz_date: String,
    #[serde(rename = "x-amz-signature")]
    pub x_amz_signature: String,
}

pub fn generate_post_policy(
    access_key: &str,
    secret_key: &str,
    bucket: &str,
    key_prefix: &str,
    region: &str,
    endpoint: &str,
    max_file_mb: u64,
) -> PostPolicyResponse {
    let now = Utc::now();
    let expiration = (now + Duration::minutes(10)).to_rfc3339();

    let date_stamp = now.format("%Y%m%d").to_string();
    let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();

    // Policy JSON
    let policy = json!({
        "expiration": expiration,
        "conditions": [
            {"bucket": bucket},
            ["starts-with", "$key", key_prefix],
            {"x-amz-algorithm": "AWS4-HMAC-SHA256"},
            {"x-amz-credential": format!("{}/{}/{}/s3/aws4_request", access_key, date_stamp, region)},
            {"x-amz-date": amz_date},
            ["content-length-range", 1, max_file_mb * 1024 * 1024]
        ]
    });

    let policy_str = policy.to_string();
    let policy_base64 = general_purpose::STANDARD.encode(policy_str.as_bytes());

    //  signature
    let date_key = hmac_sha256(
        format!("AWS4{}", secret_key).as_bytes(),
        date_stamp.as_bytes(),
    );
    let date_region_key = hmac_sha256(&date_key, region.as_bytes());
    let date_region_service_key = hmac_sha256(&date_region_key, b"s3");
    let signing_key = hmac_sha256(&date_region_service_key, b"aws4_request");
    let signature_bytes = hmac_sha256(&signing_key, policy_base64.as_bytes());
    let signature_hex = hex::encode(signature_bytes);

    PostPolicyResponse {
        url: format!("{}/{}", endpoint.trim_end_matches('/'), bucket),
        fields: PostPolicyFields {
            key: format!("{}${{filename}}", key_prefix),
            policy: policy_base64,
            x_amz_algorithm: "AWS4-HMAC-SHA256".to_string(),
            x_amz_credential: format!("{}/{}/{}/s3/aws4_request", access_key, date_stamp, region),
            x_amz_date: amz_date,
            x_amz_signature: signature_hex,
        },
    }
}

// HMAC-SHA256
fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}
