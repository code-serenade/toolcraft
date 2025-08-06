# toolcraft-config

A simple and flexible configuration management library for Rust applications.

[![Crates.io](https://img.shields.io/crates/v/toolcraft-config.svg)](https://crates.io/crates/toolcraft-config)
[![Documentation](https://docs.rs/toolcraft-config/badge.svg)](https://docs.rs/toolcraft-config)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 Simple API for loading configuration files
- 📄 Support for multiple configuration formats (TOML, JSON, YAML, etc.)
- 🎯 Type-safe configuration with serde
- 🔧 Built on top of the robust `config` crate
- ⚡ Zero boilerplate configuration loading
- 🎨 Custom error types for better error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toolcraft-config = "*"
```

Check the [crates.io page](https://crates.io/crates/toolcraft-config) for the latest version.

## Quick Start

```rust
use toolcraft_config::load_settings;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from a file
    let config: AppConfig = load_settings("config.toml")?;
    
    println!("Server: {}:{}", config.server.host, config.server.port);
    println!("Database: {}", config.database.url);
    
    Ok(())
}
```

## Configuration File Example

Create a `config.toml` file:

```toml
[server]
host = "localhost"
port = 8080

[database]
url = "postgresql://localhost/myapp"
max_connections = 10
```

## Supported Formats

The library supports all formats provided by the `config` crate:

- **TOML** (.toml)
- **JSON** (.json)
- **YAML** (.yaml, .yml)
- **INI** (.ini)
- **RON** (.ron)

The format is automatically detected based on the file extension.

## Advanced Usage

### Custom Configuration Structures

```rust
use toolcraft_config::load_settings;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyConfig {
    #[serde(default = "default_timeout")]
    timeout: u64,
    
    #[serde(rename = "max-retries")]
    max_retries: u32,
    
    features: Features,
}

#[derive(Debug, Deserialize)]
struct Features {
    #[serde(default)]
    enable_cache: bool,
    
    #[serde(default)]
    enable_logging: bool,
}

fn default_timeout() -> u64 {
    30
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: MyConfig = load_settings("my_app.toml")?;
    println!("Timeout: {} seconds", config.timeout);
    Ok(())
}
```

### Error Handling

```rust
use toolcraft_config::{load_settings, Result};

fn load_config() -> Result<AppConfig> {
    load_settings("config.toml")
}

fn main() {
    match load_config() {
        Ok(config) => {
            println!("Configuration loaded successfully");
        }
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    }
}
```

### Multiple Configuration Files

While the current API supports single file loading, you can easily extend it:

```rust
use toolcraft_config::load_settings;

fn load_with_defaults<T>() -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned + Default,
{
    match load_settings("config.toml") {
        Ok(config) => Ok(config),
        Err(_) => {
            // Fall back to defaults if config file doesn't exist
            Ok(T::default())
        }
    }
}
```

## API Reference

### Functions

- `load_settings<T>(config_path: &str) -> Result<T>` - Load configuration from a file
  - `T`: The type to deserialize into (must implement `DeserializeOwned`)
  - `config_path`: Path to the configuration file
  - Returns: `Result<T, Error>` with the parsed configuration or an error

### Types

- `Result<T>` - Type alias for `std::result::Result<T, Error>`
- `Error` - Custom error type for configuration loading failures

## Best Practices

1. **Use Strong Types**: Define specific structs for your configuration rather than using generic types
2. **Provide Defaults**: Use `#[serde(default)]` for optional configuration values
3. **Validate Early**: Validate configuration values after loading
4. **Environment Variables**: Consider combining with environment variables for sensitive data

## Example: Web Application Configuration

```rust
use toolcraft_config::load_settings;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WebAppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    redis: RedisConfig,
    jwt: JwtConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    pool_size: u32,
    timeout: u64,
}

#[derive(Debug, Deserialize)]
struct RedisConfig {
    url: String,
    #[serde(default = "default_redis_pool")]
    pool_size: u32,
}

#[derive(Debug, Deserialize)]
struct JwtConfig {
    secret: String,
    expiration: u64,
}

fn default_redis_pool() -> u32 {
    10
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: WebAppConfig = load_settings("webapp.toml")?;
    
    // Use the configuration to initialize your application
    println!("Starting server on {}:{}", config.server.host, config.server.port);
    
    Ok(())
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/code-serenade/toolcraft/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [Repository](https://github.com/code-serenade/toolcraft)
- [Documentation](https://docs.rs/toolcraft-config)
- [Crates.io](https://crates.io/crates/toolcraft-config)