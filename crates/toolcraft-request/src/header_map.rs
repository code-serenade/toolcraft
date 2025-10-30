use crate::error::{Error, Result};

/// Wrapper for HTTP headers used in request construction.
#[derive(Debug, Clone)]
pub struct HeaderMap {
    headers: reqwest::header::HeaderMap,
}

impl HeaderMap {
    /// Create a new empty HeaderMap.
    pub fn new() -> Self {
        HeaderMap {
            headers: reqwest::header::HeaderMap::new(),
        }
    }

    /// Create HeaderMap with default headers for JSON requests.
    ///
    /// Sets:
    /// - `Content-Type: application/json`
    /// - `Accept: application/json`
    ///
    /// # Example
    /// ```
    /// use toolcraft_request::{Request, HeaderMap};
    /// use serde_json::json;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Request::new()?;
    ///
    /// // Use preset JSON headers
    /// let headers = HeaderMap::for_json()?;
    /// client.set_default_headers(headers);
    ///
    /// let body = json!({"key": "value"});
    /// let response = client.post("/api", &body, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn for_json() -> Result<Self> {
        let mut headers = Self::new();
        headers.insert("Content-Type", "application/json".to_string())?;
        headers.insert("Accept", "application/json".to_string())?;
        Ok(headers)
    }

    /// Create HeaderMap for form-data requests.
    ///
    /// Returns an empty HeaderMap because:
    /// - `Content-Type` is automatically set by `post_form()` with the correct boundary
    /// - Manual setting would break multipart/form-data encoding
    ///
    /// # Example
    /// ```
    /// use toolcraft_request::{FormField, HeaderMap, Request};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Request::new()?;
    ///
    /// // For form uploads, use empty headers or add custom ones
    /// let headers = HeaderMap::for_form();
    /// client.set_default_headers(headers);
    ///
    /// let fields = vec![FormField::text("name", "value")];
    /// let response = client.post_form("/upload", fields, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn for_form() -> Self {
        Self::new()
    }

    /// Insert a header key-value pair.
    /// If the key already exists, the old value is replaced.
    ///
    /// # Example
    /// ```
    /// use toolcraft_request::HeaderMap;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut headers = HeaderMap::new();
    /// headers.insert("Authorization", "Bearer token".to_string())?;
    ///
    /// // Dynamic header names are supported
    /// let header_name = "X-Custom-Header".to_string();
    /// headers.insert(header_name, "value".to_string())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn insert(&mut self, key: impl AsRef<str>, value: String) -> Result<()> {
        let header_name = reqwest::header::HeaderName::from_bytes(key.as_ref().as_bytes())
            .map_err(|_| Error::ErrorMessage("invalid header name".into()))?;
        let header_value = reqwest::header::HeaderValue::from_str(&value)
            .map_err(|_| Error::ErrorMessage("invalid header value".into()))?;
        self.headers.insert(header_name, header_value);
        Ok(())
    }

    /// Get the value of a header as String.
    ///
    /// # Example
    /// ```
    /// use toolcraft_request::HeaderMap;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut headers = HeaderMap::new();
    /// headers.insert("Content-Type", "application/json".to_string())?;
    ///
    /// assert_eq!(
    ///     headers.get("Content-Type"),
    ///     Some("application/json".to_string())
    /// );
    /// assert_eq!(headers.get("Missing"), None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self, key: impl AsRef<str>) -> Option<String> {
        let header_name = reqwest::header::HeaderName::from_bytes(key.as_ref().as_bytes()).ok()?;
        self.headers
            .get(&header_name)
            .map(|v| v.to_str().unwrap_or_default().to_string())
    }

    /// Get reference to the internal reqwest HeaderMap.
    pub fn inner(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    /// Get mutable reference to the internal reqwest HeaderMap.
    pub fn inner_mut(&mut self) -> &mut reqwest::header::HeaderMap {
        &mut self.headers
    }

    /// Remove a header by key and return its value if it existed.
    ///
    /// # Example
    /// ```
    /// use toolcraft_request::HeaderMap;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut headers = HeaderMap::new();
    /// headers.insert("Content-Type", "application/json".to_string())?;
    ///
    /// let removed = headers.remove("Content-Type");
    /// assert_eq!(removed, Some("application/json".to_string()));
    /// assert_eq!(headers.get("Content-Type"), None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<String> {
        let header_name = reqwest::header::HeaderName::from_bytes(key.as_ref().as_bytes()).ok()?;
        self.headers
            .remove(&header_name)
            .map(|v| v.to_str().unwrap_or_default().to_string())
    }

    /// Check if a header exists.
    ///
    /// # Example
    /// ```
    /// use toolcraft_request::HeaderMap;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut headers = HeaderMap::new();
    /// headers.insert("Authorization", "Bearer token".to_string())?;
    ///
    /// assert!(headers.contains("Authorization"));
    /// assert!(!headers.contains("Missing"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains(&self, key: impl AsRef<str>) -> bool {
        if let Ok(header_name) = reqwest::header::HeaderName::from_bytes(key.as_ref().as_bytes()) {
            self.headers.contains_key(&header_name)
        } else {
            false
        }
    }

    /// Merge another HeaderMap into this one.
    /// If a key exists in both, the value from `other` will overwrite.
    ///
    /// # Example
    /// ```
    /// use toolcraft_request::HeaderMap;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut default_headers = HeaderMap::new();
    /// default_headers.insert("User-Agent", "MyApp/1.0".to_string())?;
    ///
    /// let mut custom_headers = HeaderMap::new();
    /// custom_headers.insert("Authorization", "Bearer token".to_string())?;
    ///
    /// default_headers.merge(custom_headers);
    /// # Ok(())
    /// # }
    /// ```
    pub fn merge(&mut self, other: HeaderMap) {
        self.headers.extend(other.headers);
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        Self::new()
    }
}
