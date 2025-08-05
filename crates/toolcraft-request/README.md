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
- 🎯 Type-safe error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toolcraft-request = "0.2.1"
```

## Quick Start

```rust
use toolcraft_request::Request;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new request client
    let client = Request::new("https://api.example.com")?
        .set_timeout(Duration::from_secs(30));

    // Make a simple GET request
    let response = client.get("/users")
        .header("User-Agent", "MyApp/1.0")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);

    Ok(())
}
```

## Advanced Usage

### Setting Default Headers

```rust
use toolcraft_request::{Request, HeaderMap};
use reqwest::header::{AUTHORIZATION, USER_AGENT};

let mut headers = HeaderMap::new();
headers.insert(AUTHORIZATION, "Bearer token123".parse()?);
headers.insert(USER_AGENT, "MyApp/1.0".parse()?);

let client = Request::new("https://api.example.com")?
    .set_default_headers(headers);
```

### POST Request with JSON

```rust
use serde_json::json;

let response = client.post("/users")
    .json(&json!({
        "name": "John Doe",
        "email": "john@example.com"
    }))
    .send()
    .await?;
```

### Streaming Response

```rust
use futures_util::StreamExt;

let mut stream = client.get("/large-file")
    .send()
    .await?
    .bytes_stream();

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    println!("Received {} bytes", chunk.len());
}
```

### Error Handling

```rust
use toolcraft_request::RequestError;

match client.get("/api/data").send().await {
    Ok(response) => println!("Success: {}", response.status()),
    Err(RequestError::Network(e)) => eprintln!("Network error: {}", e),
    Err(RequestError::Timeout) => eprintln!("Request timed out"),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## API Reference

### Request Methods

- `new(base_url: &str)` - Create a new Request client
- `get(path: &str)` - Create a GET request
- `post(path: &str)` - Create a POST request
- `put(path: &str)` - Create a PUT request
- `patch(path: &str)` - Create a PATCH request
- `delete(path: &str)` - Create a DELETE request

### Configuration Methods

- `set_timeout(duration: Duration)` - Set request timeout
- `set_default_headers(headers: HeaderMap)` - Set default headers for all requests

### Request Builder Methods

- `header(key: K, value: V)` - Add a header to the request
- `headers(headers: HeaderMap)` - Add multiple headers
- `query<T: Serialize>(query: &T)` - Add query parameters
- `json<T: Serialize>(json: &T)` - Set JSON body
- `body<T: Into<Body>>(body: T)` - Set request body
- `send()` - Execute the request

### Response Methods

- `status()` - Get response status code
- `headers()` - Get response headers
- `text()` - Get response body as text
- `json<T: DeserializeOwned>()` - Parse response as JSON
- `bytes()` - Get response body as bytes
- `bytes_stream()` - Get response as a byte stream

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/code-serenade/toolcraft/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [Repository](https://github.com/code-serenade/toolcraft)
- [Documentation](https://docs.rs/toolcraft-request)
- [Crates.io](https://crates.io/crates/toolcraft-request)