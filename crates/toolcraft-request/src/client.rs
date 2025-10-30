use futures_util::StreamExt;
use reqwest::{Client, multipart};
use url::Url;

use crate::{
    error::{Error, Result},
    header_map::HeaderMap,
    response::{ByteStream, Response},
};

/// An HTTP request builder and executor with base URL and default headers.
#[derive(Debug)]
pub struct Request {
    client: Client,
    base_url: Option<Url>,
    default_headers: HeaderMap,
}

impl Request {
    /// Create a new Request client.
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .build()
            .map_err(|e| Error::ErrorMessage(e.to_string().into()))?;
        Ok(Request {
            client,
            base_url: None,
            default_headers: HeaderMap::new(),
        })
    }

    pub fn with_timeout(timeout_sec: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_sec))
            .build()
            .map_err(|e| Error::ErrorMessage(e.to_string().into()))?;
        Ok(Request {
            client,
            base_url: None,
            default_headers: HeaderMap::new(),
        })
    }

    /// Set the base URL for all requests.
    pub fn set_base_url(&mut self, base_url: &str) -> Result<()> {
        let mut url_str = base_url.to_string();
        if !url_str.ends_with('/') {
            url_str.push('/');
        }
        let url = Url::parse(&url_str)?;
        self.base_url = Some(url);
        Ok(())
    }

    /// Set default headers to be applied on all requests.
    pub fn set_default_headers(&mut self, headers: HeaderMap) {
        self.default_headers = headers;
    }

    /// Send a GET request.
    pub async fn get(
        &self,
        endpoint: &str,
        query: Option<Vec<(String, String)>>,
        headers: Option<HeaderMap>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint, query)?;
        let mut request = self.client.get(url.as_str());

        let mut combined_headers = self.default_headers.clone();
        if let Some(custom_headers) = headers {
            combined_headers.merge(custom_headers);
        }
        request = request.headers(combined_headers.inner().clone());

        let response = request.send().await?;
        Ok(response.into())
    }

    /// Send a POST request with JSON body.
    pub async fn post(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
        headers: Option<HeaderMap>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint, None)?;
        let mut request = self.client.post(url).json(body);

        let mut combined_headers = self.default_headers.clone();
        if let Some(custom_headers) = headers {
            combined_headers.merge(custom_headers);
        }
        request = request.headers(combined_headers.inner().clone());

        let response = request.send().await?;
        Ok(response.into())
    }

    /// Send a PUT request with JSON body.
    pub async fn put(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
        headers: Option<HeaderMap>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint, None)?;
        let mut request = self.client.put(url).json(body);

        let mut combined_headers = self.default_headers.clone();
        if let Some(custom_headers) = headers {
            combined_headers.merge(custom_headers);
        }
        request = request.headers(combined_headers.inner().clone());

        let response = request.send().await?;
        Ok(response.into())
    }

    /// Send a DELETE request.
    pub async fn delete(
        &self,
        endpoint: &str,
        headers: Option<HeaderMap>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint, None)?;
        let mut request = self.client.delete(url);

        let mut combined_headers = self.default_headers.clone();
        if let Some(custom_headers) = headers {
            combined_headers.merge(custom_headers);
        }
        request = request.headers(combined_headers.inner().clone());

        let response = request.send().await?;
        Ok(response.into())
    }

    /// Send a POST request with multipart/form-data.
    ///
    /// # Arguments
    /// * `endpoint` - The URL endpoint
    /// * `form_fields` - Vector of form fields (text or file)
    /// * `headers` - Optional custom headers
    ///
    /// # Important
    /// The `Content-Type` header will be automatically removed from default and custom headers
    /// to allow reqwest to set the correct `multipart/form-data` with boundary.
    ///
    /// # Example
    /// ```no_run
    /// use toolcraft_request::{FormField, Request};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Request::new()?;
    /// let fields = vec![
    ///     FormField::text("name", "John"),
    ///     FormField::file("avatar", "/path/to/image.jpg").await?,
    /// ];
    /// let response = client.post_form("/upload", fields, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_form(
        &self,
        endpoint: &str,
        form_fields: Vec<FormField>,
        headers: Option<HeaderMap>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint, None)?;

        let mut form = multipart::Form::new();
        for field in form_fields {
            match field {
                FormField::Text { name, value } => {
                    form = form.text(name, value);
                }
                FormField::File {
                    name,
                    filename,
                    content,
                } => {
                    let part = multipart::Part::bytes(content).file_name(filename);
                    form = form.part(name, part);
                }
            }
        }

        let mut combined_headers = self.default_headers.clone();
        if let Some(custom_headers) = headers {
            combined_headers.merge(custom_headers);
        }

        // Remove Content-Type to let reqwest set the correct multipart/form-data with boundary
        combined_headers.remove("Content-Type");
        combined_headers.remove("content-type");

        let mut request = self.client.post(url).multipart(form);
        request = request.headers(combined_headers.inner().clone());

        let response = request.send().await?;
        Ok(response.into())
    }

    /// Send a streaming POST request and return the response stream.
    pub async fn post_stream(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
        headers: Option<HeaderMap>,
    ) -> Result<ByteStream> {
        let url = self.build_url(endpoint, None)?;
        let mut request = self.client.post(url).json(body);

        let mut combined_headers = self.default_headers.clone();
        if let Some(custom_headers) = headers {
            combined_headers.merge(custom_headers);
        }
        request = request.headers(combined_headers.inner().clone());

        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(Error::ErrorMessage(
                format!("Unexpected status: {}", response.status()).into(),
            ));
        }

        let stream = response
            .bytes_stream()
            .map(|chunk_result| chunk_result.map_err(Error::from));
        Ok(Box::pin(stream))
    }

    /// Build a full URL by combining base URL, endpoint, and optional query parameters.
    fn build_url(&self, endpoint: &str, query: Option<Vec<(String, String)>>) -> Result<Url> {
        let mut url = if let Some(base_url) = &self.base_url {
            base_url.join(endpoint)?
        } else {
            Url::parse(endpoint)?
        };

        if let Some(query_params) = query {
            let query_pairs: Vec<(String, String)> = query_params.into_iter().collect();
            url.query_pairs_mut().extend_pairs(query_pairs);
        }

        Ok(url)
    }
}

/// Parse a full URL with optional query parameters.
pub fn parse_url(url: &str, query: Option<Vec<(String, String)>>) -> Result<Url> {
    let mut url = Url::parse(url)?;
    if let Some(query_params) = query {
        let query_pairs: Vec<(String, String)> = query_params.into_iter().collect();
        url.query_pairs_mut().extend_pairs(query_pairs);
    }
    Ok(url)
}

/// Represents a field in a multipart/form-data request.
#[derive(Debug, Clone)]
pub enum FormField {
    /// A text field.
    Text { name: String, value: String },
    /// A file field.
    File {
        name: String,
        filename: String,
        content: Vec<u8>,
    },
}

impl FormField {
    /// Create a text field.
    ///
    /// # Example
    /// ```
    /// use toolcraft_request::FormField;
    /// let field = FormField::text("username", "john_doe");
    /// ```
    pub fn text(name: impl Into<String>, value: impl Into<String>) -> Self {
        FormField::Text {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Create a file field from bytes.
    ///
    /// # Example
    /// ```
    /// use toolcraft_request::FormField;
    /// let data = b"file content".to_vec();
    /// let field = FormField::file_from_bytes("avatar", "photo.jpg", data);
    /// ```
    pub fn file_from_bytes(
        name: impl Into<String>,
        filename: impl Into<String>,
        content: Vec<u8>,
    ) -> Self {
        FormField::File {
            name: name.into(),
            filename: filename.into(),
            content,
        }
    }

    /// Create a file field by reading from a file path.
    ///
    /// # Example
    /// ```no_run
    /// use toolcraft_request::FormField;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let field = FormField::file("avatar", "/path/to/image.jpg").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn file(name: impl Into<String>, path: impl AsRef<std::path::Path>) -> Result<Self> {
        let path = path.as_ref();
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::ErrorMessage("Invalid file path".into()))?
            .to_string();

        let content = tokio::fs::read(path)
            .await
            .map_err(|e| Error::ErrorMessage(format!("Failed to read file: {}", e).into()))?;

        Ok(FormField::File {
            name: name.into(),
            filename,
            content,
        })
    }
}
