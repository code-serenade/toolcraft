# toolcraft-jwt

A lightweight JWT (JSON Web Token) library for Rust with support for access and refresh tokens.

[![Crates.io](https://img.shields.io/crates/v/toolcraft-jwt.svg)](https://crates.io/crates/toolcraft-jwt)
[![Documentation](https://docs.rs/toolcraft-jwt/badge.svg)](https://docs.rs/toolcraft-jwt)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- 🔐 Access and refresh token generation
- ✅ Token validation with configurable rules
- ⏱️ Customizable token expiration times
- 🔄 Refresh token rotation support
- 🎯 Type-safe error handling
- 🚀 Simple and intuitive API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toolcraft-jwt = "*"
```

Check the [crates.io page](https://crates.io/crates/toolcraft-jwt) for the latest version.

## Quick Start

```rust
use toolcraft_jwt::{Jwt, JwtCfg};

fn main() {
    // Configure JWT settings (typically from config file + env)
    let config = JwtCfg {
        key_dir: Some("./keys".to_string()),
        access_private_key_pem: None,
        access_public_key_pem: None,
        refresh_private_key_pem: None,
        refresh_public_key_pem: None,
        issuer: "your-issuer".to_string(),
        audience: "your-app".to_string(),
        access_token_duration: 3600,  // 1 hour
        refresh_token_duration: 86400, // 24 hours
        access_key_validate_exp: true,
        refresh_key_validate_exp: true,
    };

    // Create JWT instance
    let jwt = Jwt::new(config);

    // Generate token pair
    let token_pair = jwt
        .generate_token_pair("user123".to_string(), None)
        .expect("Failed to generate tokens");

    println!("Access token: {}", token_pair.access_token);
    println!("Refresh token: {}", token_pair.refresh_token);

    // Validate access token
    match jwt.validate_access_token(&token_pair.access_token) {
        Ok(claims) => {
            println!("Valid token for user: {}", claims.sub);
            println!("Expires at: {}", claims.exp);
        }
        Err(e) => eprintln!("Invalid token: {}", e),
    }
}
```

## Advanced Usage

### Token Generation

```rust
// Generate token pair
let token_pair = jwt.generate_token_pair(
    "user123".to_string(),
    Some(serde_json::json!({"role":"admin"})),
)?;
```

### Token Validation

```rust
// Validate access token
let claims = jwt.validate_access_token(&token_pair.access_token)?;
println!("User ID: {}", claims.sub);
println!("Audience: {}", claims.aud);
println!("Issued at: {}", claims.iat);
println!("Expires at: {}", claims.exp);

// Validate refresh token
let claims = jwt.validate_refresh_token(&token_pair.refresh_token)?;
```

### Token Refresh

```rust
// Use refresh token to get new access token
let new_access_token = jwt.refresh_access_token(&token_pair.refresh_token)?;
```

### Custom Configuration

```rust
use toolcraft_jwt::JwtCfg;

let config = JwtCfg {
    key_dir: Some("./keys".to_string()),
    access_private_key_pem: None,
    access_public_key_pem: None,
    refresh_private_key_pem: None,
    refresh_public_key_pem: None,
    issuer: "my-issuer".to_string(),
    audience: "my-api".to_string(),
    access_token_duration: 900,    // 15 minutes
    refresh_token_duration: 604800, // 7 days
    access_key_validate_exp: true,  // Validate expiration
    refresh_key_validate_exp: true, // Validate expiration
};
```

### Configuration File Example

```toml
[jwt]
key_dir = "/etc/myapp/jwt"
issuer = "my-issuer"
audience = "my-api"
access_token_duration = 900
refresh_token_duration = 604800
access_key_validate_exp = true
refresh_key_validate_exp = true

# key_dir contains 4 files:
# access_private_key.pem
# access_public_key.pem
# refresh_private_key.pem
# refresh_public_key.pem
```

### Error Handling

```rust
use toolcraft_jwt::Error;

match jwt.validate_access_token(&token) {
    Ok(claims) => {
        // Token is valid
    }
    Err(Error::AuthError(msg)) => {
        // Authentication error (invalid token, expired, etc.)
        eprintln!("Auth error: {}", msg);
    }
    Err(e) => {
        // Other errors
        eprintln!("Error: {}", e);
    }
}
```

### Verify-Only (Public Key)

```rust
use toolcraft_jwt::{VerifyJwt, VerifyJwtCfg};

let verifier = VerifyJwt::new(
    std::env::var("JWT_PUBLIC_KEY_PEM").unwrap(),
    VerifyJwtCfg {
        issuer: "your-issuer".to_string(),
        audience: "your-audience".to_string(),
    },
)?;
let claims = verifier.validate_token(&token)?;
println!("sub={}", claims.sub);
```

## API Reference

### JwtCfg

Configuration struct for JWT settings:

- `access_private_key_pem`: Ed25519 private key PEM for access tokens
- `access_public_key_pem`: Ed25519 public key PEM for access tokens
- `refresh_private_key_pem`: Ed25519 private key PEM for refresh tokens
- `refresh_public_key_pem`: Ed25519 public key PEM for refresh tokens
- `key_dir`: Optional key directory, if set reads 4 fixed files:
  - `access_private_key.pem`
  - `access_public_key.pem`
  - `refresh_private_key.pem`
  - `refresh_public_key.pem`
- `issuer`: Expected issuer claim
- `audience`: Expected audience claim
- `access_token_duration`: Access token lifetime in seconds
- `refresh_token_duration`: Refresh token lifetime in seconds
- `access_key_validate_exp`: Whether to validate access token expiration
- `refresh_key_validate_exp`: Whether to validate refresh token expiration

### Jwt Methods

- `new(cfg: JwtCfg)` - Create a new JWT instance
- `generate_token_pair(sub: String, ext: Option<Value>) -> TokenPair` - Generate access and refresh tokens
- `validate_access_token(token: &str)` - Validate access token
- `validate_refresh_token(token: &str)` - Validate refresh token
- `refresh_access_token(refresh_token: &str)` - Generate new access token from refresh token
- `VerifyJwt::new(public_key_pem, cfg)` - Create verifier with fixed `iss/aud` validation config
- `VerifyJwtCfg` - Verifier config (`issuer` and `audience`)
- `VerifyJwt::validate_token(token: &str)` - Validate token using public key
- `AccessTokenVerifier` - verification trait implemented by both `Jwt` and `VerifyJwt`

### Claims

JWT claims structure:

- `iss`: Issuer
- `aud`: Audience
- `sub`: Subject (typically user ID)
- `exp`: Expiration time (Unix timestamp)
- `iat`: Issued at time (Unix timestamp)
- `ext`: Optional extension payload (`serde_json::Value`)

## Security Considerations

1. **Key Pairs**: Use dedicated Ed25519 key pairs for production
2. **Key Rotation**: Regularly rotate your secret keys
3. **HTTPS Only**: Always transmit tokens over HTTPS
4. **Storage**: Never store tokens in localStorage; use httpOnly cookies when possible
5. **Expiration**: Use short expiration times for access tokens

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/code-serenade/toolcraft/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [Repository](https://github.com/code-serenade/toolcraft)
- [Documentation](https://docs.rs/toolcraft-jwt)
- [Crates.io](https://crates.io/crates/toolcraft-jwt)
