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

    /// Insert a header key-value pair.
    pub fn insert(&mut self, key: &'static str, value: String) -> Result<()> {
        let header_value = reqwest::header::HeaderValue::from_str(&value)
            .map_err(|_| Error::ErrorMessage("invalid headerValue".into()))?;
        self.headers.insert(key, header_value);
        Ok(())
    }

    /// Get the value of a header as String.
    pub fn get(&self, key: &'static str) -> Option<String> {
        self.headers
            .get(key)
            .map(|v| v.to_str().unwrap_or_default().to_string())
    }

    /// Get reference to the internal reqwest HeaderMap.
    pub fn inner(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        Self::new()
    }
}
