use std::path::Path;

use minio::s3::{
    client::{Client, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
    types::S3Api,
};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::error::{Error, Result};

pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub fn new(endpoint: &str, access_key: &str, secret_key: &str) -> Result<Self> {
        let base_url = endpoint.parse::<BaseUrl>().map_err(|e| {
            Error::ErrorMessage(Box::from(format!("failed to parse base url: {e}")))
        })?;
        let provider = StaticProvider::new(access_key, secret_key, None);

        let client = ClientBuilder::new(base_url)
            .provider(Some(Box::new(provider)))
            .build()
            .map_err(|e| {
                Error::ErrorMessage(Box::from(format!("failed to build S3 client: {e}")))
            })?;

        Ok(S3Client { client })
    }

    pub async fn read_text_file(&self, bucket: &str, file_path: &str) -> Result<String> {
        let get_object = self
            .client
            .get_object(bucket, file_path)
            .send()
            .await
            .map_err(|e| Error::ErrorMessage(Box::from(format!("failed to get object: {e}"))))?;

        // 从 ObjectContent 读取内容
        let content = get_object.content;

        // 使用 to_segmented_bytes() 方法读取所有内容
        let segmented_bytes = content.to_segmented_bytes().await?;

        // 将 SegmentedBytes 转换为 Vec<u8>
        let mut buffer = Vec::new();
        for chunk in segmented_bytes.into_iter() {
            buffer.extend_from_slice(&chunk);
        }

        // 转换为字符串（仅适用于文本文件）
        let text_content = String::from_utf8(buffer)?;

        // println!("📄 文件内容: {}", text_content);
        // println!("📊 文件大小: {} bytes", get_object.object_size);

        // if let Some(etag) = get_object.etag {
        //     println!("🏷️  ETag: {}", etag);
        // }

        Ok(text_content)
    }

    /// Download a file from S3 to local filesystem
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the S3 bucket
    /// * `object_key` - The key/path of the object in S3
    /// * `local_path` - The local path where the file should be saved
    ///
    /// # Returns
    ///
    /// Returns the number of bytes downloaded
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use toolcraft_s3_kit::S3Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = S3Client::new("http://localhost:9000", "access_key", "secret_key")?;
    /// let bytes_downloaded = client
    ///     .download_file("my-bucket", "path/to/file.pdf", "/tmp/local_file.pdf")
    ///     .await?;
    /// println!("Downloaded {} bytes", bytes_downloaded);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_file(
        &self,
        bucket: &str,
        object_key: &str,
        local_path: &str,
    ) -> Result<u64> {
        // Create parent directories if they don't exist
        if let Some(parent) = Path::new(local_path).parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                Error::ErrorMessage(Box::from(format!("failed to create directory: {e}")))
            })?;
        }

        // Get the object from S3
        let get_object = self
            .client
            .get_object(bucket, object_key)
            .send()
            .await
            .map_err(|e| Error::ErrorMessage(Box::from(format!("failed to get object: {e}"))))?;

        // Create the local file
        let mut file = File::create(local_path)
            .await
            .map_err(|e| Error::ErrorMessage(Box::from(format!("failed to create file: {e}"))))?;

        let object_size = get_object.object_size;
        let content = get_object.content;

        // Use to_segmented_bytes() method to read all content
        let segmented_bytes = content.to_segmented_bytes().await?;

        // Write all segments to file
        let mut bytes_written = 0u64;
        for chunk in segmented_bytes.into_iter() {
            file.write_all(&chunk).await.map_err(|e| {
                Error::ErrorMessage(Box::from(format!("failed to write to file: {e}")))
            })?;
            bytes_written += chunk.len() as u64;
        }

        // Ensure all data is written to disk
        file.flush()
            .await
            .map_err(|e| Error::ErrorMessage(Box::from(format!("failed to flush file: {e}"))))?;

        // Verify the downloaded size matches the expected size
        if bytes_written != object_size {
            return Err(Error::ErrorMessage(Box::from(format!(
                "download size mismatch: expected {object_size} bytes, got {bytes_written} bytes"
            ))));
        }

        Ok(bytes_written)
    }

    /// Download a file from S3 to memory as bytes
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the S3 bucket
    /// * `object_key` - The key/path of the object in S3
    ///
    /// # Returns
    ///
    /// Returns the file content as a Vec<u8>
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use toolcraft_s3_kit::S3Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = S3Client::new("http://localhost:9000", "access_key", "secret_key")?;
    /// let file_bytes = client
    ///     .download_to_bytes("my-bucket", "path/to/file.pdf")
    ///     .await?;
    /// println!("Downloaded {} bytes", file_bytes.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_to_bytes(&self, bucket: &str, object_key: &str) -> Result<Vec<u8>> {
        let get_object = self
            .client
            .get_object(bucket, object_key)
            .send()
            .await
            .map_err(|e| Error::ErrorMessage(Box::from(format!("failed to get object: {e}"))))?;

        let content = get_object.content;

        // Use to_segmented_bytes() method to read all content
        let segmented_bytes = content.to_segmented_bytes().await?;

        // Convert SegmentedBytes to Vec<u8>
        let mut buffer = Vec::new();
        for chunk in segmented_bytes.into_iter() {
            buffer.extend_from_slice(&chunk);
        }

        Ok(buffer)
    }
}
