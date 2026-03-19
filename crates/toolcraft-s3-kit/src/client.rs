use reqwest::Client;
use toolcraft_utils::{DEFAULT_REGION, sign_request};
use url::Url;

use crate::{
    error::{Error, Result},
    util::{check_status, parse_bucket_names},
};

// ── Types ─────────────────────────────────────────────────────────────────────

pub struct S3Client {
    pub(crate) access_key: String,
    pub(crate) secret_key: String,
    pub(crate) base_url: Url,
    pub(crate) region: String,
    pub(crate) http: Client,
}

// ── Init ──────────────────────────────────────────────────────────────────────

impl S3Client {
    pub fn new(
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
        region: Option<&str>,
    ) -> Result<Self> {
        let base_url = Url::parse(endpoint)?;
        let http = Client::builder()
            .build()
            .map_err(|e| Error::Message(e.to_string().into()))?;
        Ok(Self {
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url,
            region: region.unwrap_or(DEFAULT_REGION).to_string(),
            http,
        })
    }
}

// ── Bucket management ─────────────────────────────────────────────────────────

impl S3Client {
    pub async fn create_bucket(&self, bucket: &str) -> Result<()> {
        let path = format!("/{bucket}");
        let auth = sign_request(
            "PUT",
            &self.access_key,
            &self.secret_key,
            &self.host(),
            &path,
            "",
            Some(&self.region),
        );

        let body = if self.region != "us-east-1" {
            format!(
                "<CreateBucketConfiguration><LocationConstraint>{}</LocationConstraint></\
                 CreateBucketConfiguration>",
                self.region,
            )
        } else {
            String::new()
        };

        let resp = self
            .http
            .put(self.url(&path))
            .header("host", self.host())
            .header("x-amz-date", &auth.x_amz_date)
            .header("x-amz-content-sha256", &auth.x_amz_content_sha256)
            .header("authorization", &auth.authorization)
            .body(body)
            .send()
            .await?;

        check_status(resp).await.map(|_| ())
    }

    pub async fn delete_bucket(&self, bucket: &str) -> Result<()> {
        let path = format!("/{bucket}");
        let auth = sign_request(
            "DELETE",
            &self.access_key,
            &self.secret_key,
            &self.host(),
            &path,
            "",
            Some(&self.region),
        );

        let resp = self
            .http
            .delete(self.url(&path))
            .header("host", self.host())
            .header("x-amz-date", &auth.x_amz_date)
            .header("x-amz-content-sha256", &auth.x_amz_content_sha256)
            .header("authorization", &auth.authorization)
            .send()
            .await?;

        check_status(resp).await.map(|_| ())
    }

    pub async fn list_buckets(&self) -> Result<Vec<String>> {
        let auth = sign_request(
            "GET",
            &self.access_key,
            &self.secret_key,
            &self.host(),
            "/",
            "",
            Some(&self.region),
        );

        let resp = self
            .http
            .get(self.url("/"))
            .header("host", self.host())
            .header("x-amz-date", &auth.x_amz_date)
            .header("x-amz-content-sha256", &auth.x_amz_content_sha256)
            .header("authorization", &auth.authorization)
            .send()
            .await?;

        let xml = check_status(resp).await?.text().await?;
        parse_bucket_names(&xml)
    }
}

// ── Private helpers ───────────────────────────────────────────────────────────

impl S3Client {
    pub(crate) fn host(&self) -> String {
        let host = self.base_url.host_str().unwrap_or_default();
        match self.base_url.port() {
            Some(port) => format!("{host}:{port}"),
            None => host.to_string(),
        }
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}://{}{}", self.base_url.scheme(), self.host(), path)
    }
}
