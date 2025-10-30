# toolcraft-request

A lightweight, ergonomic HTTP client wrapper around `reqwest` with support for base URLs, default headers, and streaming responses.

[![Crates.io](https://img.shields.io/crates/v/toolcraft-request.svg)](https://crates.io/crates/toolcraft-request)
[![Documentation](https://docs.rs/toolcraft-request/badge.svg)](https://docs.rs/toolcraft-request)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 Simple and intuitive API
- 🔧 Base URL configuration for API clients
- 📋 Default headers management
- ⏱️ Configurable timeouts
- 🌊 Stream response support
- 📤 Multipart/form-data file upload
- 🎯 Type-safe error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toolcraft-request = "*"
```

Check the [crates.io page](https://crates.io/crates/toolcraft-request) for the latest version.

## Quick Start

```rust
use toolcraft_request::Request;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new request client
    let mut client = Request::new()?;
    client.set_base_url("https://api.example.com")?;

    // Make a simple GET request
    let response = client.get("/users", None, None).await?;
    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);

    Ok(())
}
```

## Advanced Usage

### Setting Default Headers

```rust
use toolcraft_request::{Request, HeaderMap};

let mut client = Request::new()?;
client.set_base_url("https://api.example.com")?;

// Method 1: Build headers manually
let mut headers = HeaderMap::new();
headers.insert("Authorization", "Bearer token123".to_string())?;
headers.insert("User-Agent", "MyApp/1.0".to_string())?;
client.set_default_headers(headers);

// Method 2: Use preset for JSON APIs
let headers = HeaderMap::for_json()?;
client.set_default_headers(headers);
// Sets: Content-Type: application/json
//       Accept: application/json

// Method 3: Use preset for form uploads
let headers = HeaderMap::for_form();
client.set_default_headers(headers);
// Returns empty HeaderMap (Content-Type handled by post_form)
```

### GET Request with Query Parameters

```rust
let query = vec![
    ("page".to_string(), "1".to_string()),
    ("limit".to_string(), "10".to_string()),
];

let response = client.get("/users", Some(query), None).await?;
```

### POST Request with JSON

```rust
use serde_json::json;

let body = json!({
    "name": "John Doe",
    "email": "john@example.com"
});

let response = client.post("/users", &body, None).await?;
```

### POST with Custom Headers (Dynamic Values)

```rust
use toolcraft_request::HeaderMap;
use serde_json::json;

// Support dynamic header values
let token = get_auth_token().await?;
let mut headers = HeaderMap::new();
headers.insert("Authorization", format!("Bearer {}", token))?;
headers.insert("Content-Type", "application/json".to_string())?;

let body = json!({"data": "value"});
let response = client.post("/api/data", &body, Some(headers)).await?;
```

### File Upload with FormData

```rust
use toolcraft_request::{Request, FormField};

let mut client = Request::new()?;
client.set_base_url("https://api.example.com")?;

// Method 1: Upload file from path
let fields = vec![
    FormField::text("username", "john_doe"),
    FormField::text("description", "My avatar"),
    FormField::file("avatar", "/path/to/image.jpg").await?,
];

let response = client.post_form("/upload", fields, None).await?;

// Method 2: Upload from bytes
let file_data = std::fs::read("/path/to/file.pdf")?;
let fields = vec![
    FormField::text("title", "Document"),
    FormField::file_from_bytes("document", "file.pdf", file_data),
];

let response = client.post_form("/documents", fields, None).await?;
```

**Important**: `post_form()` automatically removes `Content-Type` header to let reqwest set the correct `multipart/form-data` with boundary.

### Managing Headers

```rust
use toolcraft_request::HeaderMap;

// Create with presets
let mut headers = HeaderMap::for_json()?;  // JSON API preset
// Or: let headers = HeaderMap::for_form();  // Form upload preset
// Or: let mut headers = HeaderMap::new();   // Empty

// Insert headers (supports dynamic strings)
headers.insert("Authorization", "Bearer token".to_string())?;
let custom_header = "X-Request-ID".to_string();
headers.insert(custom_header, "12345".to_string())?;

// Check if header exists
if headers.contains("Authorization") {
    println!("Auth header present");
}

// Get header value
if let Some(value) = headers.get("Authorization") {
    println!("Auth: {}", value);
}

// Remove header
let removed = headers.remove("Authorization");
assert_eq!(removed, Some("Bearer token".to_string()));

// Merge headers
let mut other_headers = HeaderMap::new();
other_headers.insert("User-Agent", "MyApp/1.0".to_string())?;
headers.merge(other_headers);
```

### Streaming Response

```rust
use futures_util::StreamExt;
use serde_json::json;

let body = json!({
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "Hello"}],
    "stream": true
});

let mut stream = client.post_stream("/chat", &body, None).await?;

while let Some(chunk) = stream.next().await {
    let bytes = chunk?;
    println!("Received {} bytes", bytes.len());
}
```

### Error Handling

```rust
match client.get("/api/data", None, None).await {
    Ok(response) => println!("Success: {}", response.status()),
    Err(e) => eprintln!("Error: {}", e),
}
```

## API Reference

### Creating a Client

- `Request::new()` - Create a new Request client
- `Request::with_timeout(timeout_sec: u64)` - Create client with timeout

### Configuration Methods

- `set_base_url(&mut self, base_url: &str)` - Set base URL for all requests
- `set_default_headers(&mut self, headers: HeaderMap)` - Set default headers

### HTTP Methods

- `get(endpoint, query, headers)` - Send GET request
  - `endpoint: &str` - API endpoint
  - `query: Option<Vec<(String, String)>>` - Query parameters
  - `headers: Option<HeaderMap>` - Custom headers

- `post(endpoint, body, headers)` - Send POST request with JSON
  - `endpoint: &str` - API endpoint
  - `body: &serde_json::Value` - JSON body
  - `headers: Option<HeaderMap>` - Custom headers

- `put(endpoint, body, headers)` - Send PUT request with JSON
- `delete(endpoint, headers)` - Send DELETE request

- `post_form(endpoint, form_fields, headers)` - Send POST with multipart/form-data
  - `endpoint: &str` - API endpoint
  - `form_fields: Vec<FormField>` - Form fields (text and files)
  - `headers: Option<HeaderMap>` - Custom headers

- `post_stream(endpoint, body, headers)` - Send POST and return byte stream

### FormField Methods

- `FormField::text(name, value)` - Create text field
- `FormField::file(name, path)` - Create file field from path (async)
- `FormField::file_from_bytes(name, filename, content)` - Create file field from bytes

### HeaderMap Methods

**Factory Methods:**
- `new()` - Create new empty HeaderMap
- `for_json()` - Create with JSON preset headers (Content-Type + Accept)
- `for_form()` - Create for form uploads (empty, Content-Type auto-handled)

**Management Methods:**
- `insert(key, value)` - Insert header (supports dynamic strings, overwrites if exists)
- `get(key)` - Get header value as String
- `remove(key)` - Remove header and return its value
- `contains(key)` - Check if header exists
- `merge(other)` - Merge another HeaderMap (overwrites on conflict)

### Response Methods

- `status()` - Get HTTP status code
- `headers()` - Get response headers
- `text()` - Get response as text (async)
- `json<T>()` - Parse response as JSON (async)
- `bytes()` - Get response as bytes (async)

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/code-serenade/toolcraft/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [Repository](https://github.com/code-serenade/toolcraft)
- [Documentation](https://docs.rs/toolcraft-request)
- [Crates.io](https://crates.io/crates/toolcraft-request)