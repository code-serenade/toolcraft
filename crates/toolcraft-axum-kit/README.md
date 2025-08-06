# toolcraft-axum-kit

A comprehensive toolkit for building Axum web services with built-in middleware, error handling, and response utilities.

[![Crates.io](https://img.shields.io/crates/v/toolcraft-axum-kit.svg)](https://crates.io/crates/toolcraft-axum-kit)
[![Documentation](https://docs.rs/toolcraft-axum-kit/badge.svg)](https://docs.rs/toolcraft-axum-kit)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 Quick HTTP server setup with sensible defaults
- 🔐 JWT authentication middleware (optional)
- 🌐 CORS middleware support
- 📋 Standardized response format
- 🎯 Type-safe error handling
- 🔧 OpenAPI/Swagger integration ready

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toolcraft-axum-kit = "*"
```

Or with specific features:

```toml
[dependencies]
toolcraft-axum-kit = { version = "*", features = ["jwt"] }
```

Check the [crates.io page](https://crates.io/crates/toolcraft-axum-kit) for the latest version.

## Quick Start

```rust
use axum::{routing::get, Router};
use toolcraft_axum_kit::{start, CommonOk};

#[tokio::main]
async fn main() {
    // Create your router
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health));

    // Start the server
    start("0.0.0.0:3000", app).await;
}

async fn handler() -> CommonOk<String> {
    CommonOk("Hello, World!".to_string())
}

async fn health() -> CommonOk<&'static str> {
    CommonOk("OK")
}
```

## Advanced Usage

### Standardized Responses

The toolkit provides a standardized response format for consistency across your API:

```rust
use toolcraft_axum_kit::{CommonOk, CommonError, CommonResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

// Success response
async fn get_user() -> CommonOk<User> {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
    };
    CommonOk(user)
}

// Error response
async fn not_found() -> CommonError {
    CommonError::not_found("User not found")
}

// Using Result type
async fn get_user_by_id(id: u64) -> Result<CommonOk<User>, CommonError> {
    if id == 0 {
        return Err(CommonError::bad_request("Invalid user ID"));
    }
    
    Ok(CommonOk(User {
        id,
        name: "Bob".to_string(),
    }))
}
```

### CORS Middleware

```rust
use axum::Router;
use toolcraft_axum_kit::middleware::cors::cors_layer;

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(cors_layer());
```

### JWT Authentication Middleware

When the `jwt` feature is enabled:

```rust
use axum::{Router, routing::get};
use toolcraft_axum_kit::middleware::auth_mw::{auth_layer, Claims};
use axum::Extension;

// Configure your JWT settings
let jwt_config = toolcraft_jwt::JwtCfg {
    access_secret: "your-secret".to_string(),
    refresh_secret: "your-refresh-secret".to_string(),
    audience: "your-app".to_string(),
    access_token_duration: 3600,
    refresh_token_duration: 86400,
    access_key_validate_exp: true,
    refresh_key_validate_exp: true,
};

let jwt = toolcraft_jwt::Jwt::new(jwt_config);

// Protected routes
let protected_routes = Router::new()
    .route("/profile", get(get_profile))
    .layer(auth_layer(jwt.clone()));

// Handler with authentication
async fn get_profile(Extension(claims): Extension<Claims>) -> CommonOk<String> {
    CommonOk(format!("Hello, user {}", claims.sub))
}

let app = Router::new()
    .nest("/api", protected_routes)
    .route("/login", post(login));
```

### Error Handling

The toolkit provides comprehensive error handling:

```rust
use toolcraft_axum_kit::{CommonError, error::ErrorCode};

// Using predefined error methods
async fn handler() -> Result<CommonOk<String>, CommonError> {
    // Bad request (400)
    return Err(CommonError::bad_request("Invalid input"));
    
    // Unauthorized (401)
    return Err(CommonError::unauthorized("Please login"));
    
    // Forbidden (403)
    return Err(CommonError::forbidden("Access denied"));
    
    // Not found (404)
    return Err(CommonError::not_found("Resource not found"));
    
    // Internal server error (500)
    return Err(CommonError::internal_server_error("Something went wrong"));
}

// Custom error codes
async fn custom_error() -> CommonError {
    CommonError::new(ErrorCode::Custom(422), "Validation failed")
}
```

### Response Types

```rust
use toolcraft_axum_kit::{CommonResponse, IntoCommonResponse, Empty};

// Different response types
async fn success_with_data() -> CommonResponse {
    CommonOk("Success").into_common_response()
}

async fn success_empty() -> CommonResponse {
    CommonOk(Empty).into_common_response()
}

async fn error_response() -> CommonResponse {
    CommonError::bad_request("Invalid request").into_common_response()
}

// Using Result with ResponseResult
use toolcraft_axum_kit::{Result, ResponseResult};

async fn flexible_handler(id: u64) -> ResponseResult<User> {
    if id == 0 {
        return Err(CommonError::bad_request("Invalid ID"));
    }
    
    Ok(CommonOk(User {
        id,
        name: "Alice".to_string(),
    }))
}
```

## API Reference

### Server Functions

- `start(addr: &str, app: Router)` - Start the HTTP server

### Response Types

- `CommonOk<T>` - Success response wrapper
- `CommonError` - Error response wrapper
- `CommonResponse` - Generic response type
- `ResponseResult<T>` - Result type alias for handlers
- `Empty` - Empty response body

### Error Codes

- `ErrorCode::BadRequest` - 400
- `ErrorCode::Unauthorized` - 401
- `ErrorCode::Forbidden` - 403
- `ErrorCode::NotFound` - 404
- `ErrorCode::InternalServerError` - 500
- `ErrorCode::Custom(u16)` - Custom status code

### Middleware

- `cors_layer()` - CORS middleware layer
- `auth_layer(jwt: Jwt)` - JWT authentication middleware (requires `jwt` feature)

## Features

- `jwt` - Enable JWT authentication middleware (enabled by default)

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/code-serenade/toolcraft/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [Repository](https://github.com/code-serenade/toolcraft)
- [Documentation](https://docs.rs/toolcraft-axum-kit)
- [Crates.io](https://crates.io/crates/toolcraft-axum-kit)