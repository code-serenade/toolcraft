use std::{path::Path, sync::Arc};

use bytes::Bytes;
use toolcraft_utils::{presign_get_object, presign_put_object, sign_request};

use crate::{
    client::S3Client,
    error::Result,
    util::{ObjectInfo, check_status, parse_object_list, url_encode},
};

// ── Types ─────────────────────────────────────────────────────────────────────

/// Object operations scoped to a specific bucket.
///
/// Constructed from a shared [`S3Client`]:
/// ```rust,ignore
/// let client = Arc::new(S3Client::new(endpoint, ak, sk, None)?);
/// let bucket = BucketClient::new(Arc::clone(&client), "my-bucket");
/// ```
#[derive(Clone)]
pub struct BucketClient {
    inner: Arc<S3Client>,
    bucket: String,
}

// ── Init ──────────────────────────────────────────────────────────────────────

impl BucketClient {
    pub fn new(client: Arc<S3Client>, bucket: impl Into<String>) -> Self {
        Self {
            inner: client,
            bucket: bucket.into(),
        }
    }
}

// ── Object operations ─────────────────────────────────────────────────────────

impl BucketClient {
    pub async fn list_objects(&self, prefix: Option<&str>) -> Result<Vec<ObjectInfo>> {
        let c = &self.inner;
        let path = format!("/{}", self.bucket);
        let query = match prefix {
            Some(p) => format!("list-type=2&prefix={}", url_encode(p)),
            None => "list-type=2".to_string(),
        };
        let auth = sign_request(
            "GET",
            &c.access_key,
            &c.secret_key,
            &c.host(),
            &path,
            &query,
            Some(&c.region),
        );

        let resp = c
            .http
            .get(format!("{}?{}", c.url(&path), query))
            .header("host", c.host())
            .header("x-amz-date", &auth.x_amz_date)
            .header("x-amz-content-sha256", &auth.x_amz_content_sha256)
            .header("authorization", &auth.authorization)
            .send()
            .await?;

        let xml = check_status(resp).await?.text().await?;
        parse_object_list(&xml)
    }

    /// Upload raw bytes as an object.
    pub async fn upload_bytes(
        &self,
        key: &str,
        data: Bytes,
        content_type: Option<&str>,
    ) -> Result<()> {
        let c = &self.inner;
        let url = presign_put_object(
            &c.access_key,
            &c.secret_key,
            &self.bucket,
            key,
            Some(&c.region),
            c.base_url.as_str(),
            None,
        );

        let mut req = c.http.put(&url).body(data);
        if let Some(ct) = content_type {
            req = req.header("content-type", ct);
        }
        check_status(req.send().await?).await.map(|_| ())
    }

    /// Upload a local file to S3, returning uploaded bytes length.
    pub async fn upload_local_file<P: AsRef<Path>>(
        &self,
        key: &str,
        local_path: P,
        content_type: Option<&str>,
    ) -> Result<u64> {
        let bytes = tokio::fs::read(local_path.as_ref()).await?;
        let size = bytes.len() as u64;
        self.upload_bytes(key, Bytes::from(bytes), content_type)
            .await?;
        Ok(size)
    }

    /// Backward-compatible alias. Prefer [`BucketClient::upload_bytes`].
    pub async fn upload_file(
        &self,
        key: &str,
        data: Bytes,
        content_type: Option<&str>,
    ) -> Result<()> {
        self.upload_bytes(key, data, content_type).await
    }

    pub async fn download_object(&self, key: &str) -> Result<Bytes> {
        let c = &self.inner;
        let url = presign_get_object(
            &c.access_key,
            &c.secret_key,
            &self.bucket,
            key,
            Some(&c.region),
            c.base_url.as_str(),
            None,
        );

        let resp = check_status(c.http.get(&url).send().await?).await?;
        Ok(resp.bytes().await?)
    }

    pub async fn delete_object(&self, key: &str) -> Result<()> {
        let c = &self.inner;
        let path = format!("/{}/{}", self.bucket, key.trim_start_matches('/'));
        let auth = sign_request(
            "DELETE",
            &c.access_key,
            &c.secret_key,
            &c.host(),
            &path,
            "",
            Some(&c.region),
        );

        let resp = c
            .http
            .delete(c.url(&path))
            .header("host", c.host())
            .header("x-amz-date", &auth.x_amz_date)
            .header("x-amz-content-sha256", &auth.x_amz_content_sha256)
            .header("authorization", &auth.authorization)
            .send()
            .await?;

        check_status(resp).await.map(|_| ())
    }

    /// Generate a presigned PUT URL for direct client-side upload.
    pub fn presign_upload(&self, key: &str, expires_secs: Option<u64>) -> String {
        let c = &self.inner;
        presign_put_object(
            &c.access_key,
            &c.secret_key,
            &self.bucket,
            key,
            Some(&c.region),
            c.base_url.as_str(),
            expires_secs,
        )
    }
}
