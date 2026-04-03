use std::{env, sync::Arc};

use crate::{
    bucket_client::BucketClient,
    client::S3Client,
    error::{Error, Result},
};

/// S3 configuration for building [`S3Client`] and [`BucketClient`].
///
/// Supported environment variables:
/// - `TOOLCRAFT_S3_ENDPOINT` or `S3_ENDPOINT`
/// - `TOOLCRAFT_S3_ACCESS_KEY` or `S3_ACCESS_KEY`
/// - `TOOLCRAFT_S3_SECRET_KEY` or `S3_SECRET_KEY`
/// - `TOOLCRAFT_S3_BUCKET` or `S3_BUCKET`
/// - `TOOLCRAFT_S3_REGION` or `S3_REGION` (optional)
#[derive(Debug, Clone)]
pub struct S3BucketConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub region: Option<String>,
}

impl S3BucketConfig {
    pub fn new(
        endpoint: impl Into<String>,
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
        bucket: impl Into<String>,
        region: Option<String>,
    ) -> Self {
        Self {
            endpoint: endpoint.into(),
            access_key: access_key.into(),
            secret_key: secret_key.into(),
            bucket: bucket.into(),
            region,
        }
    }

    /// Load config from environment variables.
    pub fn from_env() -> Result<Self> {
        let endpoint = read_required(&["TOOLCRAFT_S3_ENDPOINT", "S3_ENDPOINT"])?;
        let access_key = read_required(&["TOOLCRAFT_S3_ACCESS_KEY", "S3_ACCESS_KEY"])?;
        let secret_key = read_required(&["TOOLCRAFT_S3_SECRET_KEY", "S3_SECRET_KEY"])?;
        let bucket = read_required(&["TOOLCRAFT_S3_BUCKET", "S3_BUCKET"])?;
        let region = read_optional(&["TOOLCRAFT_S3_REGION", "S3_REGION"]);

        Ok(Self {
            endpoint,
            access_key,
            secret_key,
            bucket,
            region,
        })
    }

    pub fn build_s3_client(&self) -> Result<S3Client> {
        S3Client::new(
            &self.endpoint,
            &self.access_key,
            &self.secret_key,
            self.region.as_deref(),
        )
    }

    pub fn build_bucket_client(&self) -> Result<BucketClient> {
        let s3_client = Arc::new(self.build_s3_client()?);
        Ok(BucketClient::new(s3_client, self.bucket.clone()))
    }
}

fn read_required(names: &[&str]) -> Result<String> {
    for name in names {
        if let Ok(value) = env::var(name) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.to_string());
            }
        }
    }

    let joined = names.join(" / ");
    Err(Error::Message(
        format!("missing required environment variable: {joined}").into(),
    ))
}

fn read_optional(names: &[&str]) -> Option<String> {
    for name in names {
        if let Ok(value) = env::var(name) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}
