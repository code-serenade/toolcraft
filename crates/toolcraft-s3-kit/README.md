# toolcraft-s3-kit

S3-compatible object storage utilities for the toolcraft ecosystem.

[![Crates.io](https://img.shields.io/crates/v/toolcraft-s3-kit.svg)](https://crates.io/crates/toolcraft-s3-kit)
[![Documentation](https://docs.rs/toolcraft-s3-kit/badge.svg)](https://docs.rs/toolcraft-s3-kit)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 S3-compatible object storage client
- 📦 Support for MinIO and other S3-compatible services
- 🔐 Secure credential management
- ⚡ Async/await support with Tokio
- 🧩 Easy integration with the toolcraft ecosystem

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toolcraft-s3-kit = "*"
```

Check the [crates.io page](https://crates.io/crates/toolcraft-s3-kit) for the latest version.

## Quick Start

```rust
use toolcraft_s3_kit::S3Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create S3 client
    let client = S3Client::new(
        "http://localhost:9000",
        "minioadmin",
        "minioadmin",
        None, // Optional region
    )?;
    
    // List buckets
    let buckets = client.list_buckets().await?;
    for bucket in buckets {
        println!("Bucket: {}", bucket.name);
    }
    
    Ok(())
}
```

## Core Features

### S3 Client

The `S3Client` provides a high-level interface for interacting with S3-compatible storage:

- **Bucket Operations**: Create, list, and delete buckets
- **Object Operations**: Upload, download, list, and delete objects
- **Presigned URLs**: Generate time-limited URLs for object access
- **Streaming**: Support for streaming large files
- **Error Handling**: Comprehensive error types for all operations

### Example Usage

```rust
use toolcraft_s3_kit::S3Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = S3Client::new(
        "http://localhost:9000",
        "minioadmin",
        "minioadmin",
        None,
    )?;
    
    // Create a bucket
    client.create_bucket("my-bucket").await?;
    
    // Upload an object
    let data = b"Hello, world!";
    client.put_object("my-bucket", "hello.txt", data.to_vec()).await?;
    
    // Download an object
    let downloaded = client.get_object("my-bucket", "hello.txt").await?;
    println!("Downloaded: {}", String::from_utf8_lossy(&downloaded));
    
    // Generate presigned URL
    let url = client.presigned_get_object("my-bucket", "hello.txt", 3600).await?;
    println!("Presigned URL: {}", url);
    
    Ok(())
}
```

## Advanced Features

### Custom Configuration

```rust
use toolcraft_s3_kit::S3Client;

let client = S3Client::new(
    "https://s3.amazonaws.com",
    "your-access-key",
    "your-secret-key",
    Some("us-east-1"), // Specify region
)?;
```

### Error Handling

The crate provides detailed error types for different failure scenarios:

```rust
use toolcraft_s3_kit::{S3Client, error::S3Error};

match client.get_object("bucket", "key").await {
    Ok(data) => println!("Success!"),
    Err(S3Error::NotFound) => println!("Object not found"),
    Err(S3Error::AccessDenied) => println!("Access denied"),
    Err(e) => println!("Other error: {}", e),
}
```

## Supported S3 Operations

- **Bucket Operations**
  - `create_bucket()`
  - `list_buckets()`
  - `delete_bucket()`
  - `bucket_exists()`

- **Object Operations**
  - `put_object()`
  - `get_object()`
  - `delete_object()`
  - `list_objects()`
  - `object_exists()`

- **Presigned URLs**
  - `presigned_get_object()`
  - `presigned_put_object()`

## Integration with toolcraft

This crate is designed to work seamlessly with other toolcraft components:

```rust
// Use with toolcraft-config for configuration management
use toolcraft_config::Config;
use toolcraft_s3_kit::S3Client;

let config = Config::from_file("config.toml")?;
let s3_endpoint = config.get_string("s3.endpoint")?;
let s3_access_key = config.get_string("s3.access_key")?;
let s3_secret_key = config.get_string("s3.secret_key")?;

let client = S3Client::new(&s3_endpoint, &s3_access_key, &s3_secret_key, None)?;
```

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/code-serenade/toolcraft/blob/main/LICENSE) file for details.

## Links

- [Repository](https://github.com/code-serenade/toolcraft)
- [Documentation](https://docs.rs/toolcraft-s3-kit)
- [Crates.io](https://crates.io/crates/toolcraft-s3-kit)