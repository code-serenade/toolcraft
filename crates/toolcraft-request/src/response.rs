use std::pin::Pin;

use bytes::Bytes;
use futures_util::{Stream, StreamExt};

use crate::error::{Error, Result};

pub type ByteStream = Pin<Box<dyn Stream<Item = crate::error::Result<Bytes>> + Send>>;
pub struct Response {
    response: reqwest::Response,
}

impl From<reqwest::Response> for Response {
    fn from(response: reqwest::Response) -> Self {
        Response { response }
    }
}

impl Response {
    /// Create a new Response wrapper.
    pub fn new(response: reqwest::Response) -> Self {
        Response { response }
    }

    /// Get the underlying reqwest Response.
    pub fn inner(&self) -> &reqwest::Response {
        &self.response
    }

    /// Get the status code of the response.
    pub fn status(&self) -> reqwest::StatusCode {
        self.inner().status()
    }

    /// Get the response body as a string.
    pub async fn text(self) -> Result<String> {
        self.response
            .text()
            .await
            .map_err(Error::from)
            .map(|s| s.to_string())
    }

    /// Get the response headers.
    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        self.response.headers()
    }

    /// Get the response body as JSON.
    pub async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        self.response.json::<T>().await.map_err(Error::from)
    }

    /// Get the response body as bytes.
    pub async fn bytes(self) -> Result<Bytes> {
        self.response.bytes().await.map_err(Error::from)
    }

    /// Get the response body as a stream of bytes.
    pub fn bytes_stream(self) -> ByteStream {
        let stream = self
            .response
            .bytes_stream()
            .map(|chunk_result| chunk_result.map_err(Error::from));
        Box::pin(stream)
    }
}
