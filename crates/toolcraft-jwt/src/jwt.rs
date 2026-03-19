use std::{fs, path::Path};

use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{AccessTokenVerifier, Result, error::Error};

/// Struct representing the JWT configuration parameters.
#[derive(Debug, Deserialize)]
pub struct JwtCfg {
    #[serde(default)]
    pub key_dir: Option<String>,
    #[serde(default)]
    pub access_private_key_pem: Option<String>,
    #[serde(default)]
    pub access_public_key_pem: Option<String>,
    #[serde(default)]
    pub refresh_private_key_pem: Option<String>,
    #[serde(default)]
    pub refresh_public_key_pem: Option<String>,
    pub issuer: String,
    pub audience: String,
    pub access_token_duration: usize,
    pub refresh_token_duration: usize,
    pub access_key_validate_exp: bool,
    pub refresh_key_validate_exp: bool,
}

/// Represents the JWT claims.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

impl Claims {
    /// Creates a new `Claims` instance.
    pub fn new(iss: String, aud: String, sub: String, exp: usize, iat: usize) -> Self {
        Self::new_with_ext(iss, aud, sub, exp, iat, None)
    }

    /// Creates a new `Claims` instance with custom extension payload.
    pub fn new_with_ext(
        iss: String,
        aud: String,
        sub: String,
        exp: usize,
        iat: usize,
        ext: Option<Value>,
    ) -> Self {
        Self {
            iss,
            aud,
            sub,
            exp,
            iat,
            ext,
        }
    }
}

/// Enum representing the type of token: ACCESS or REFRESH.
enum TokenKind {
    Access,
    Refesh,
}

/// Struct representing the JWT configuration and operations.
#[derive(Clone)]
pub struct Jwt {
    header: Header,
    encoding_access_key: EncodingKey,
    encoding_refresh_key: EncodingKey,
    decoding_access_key: DecodingKey,
    decoding_refresh_key: DecodingKey,
    validation_access_key: Validation,
    validation_refresh_key: Validation,
    iss: String,
    aud: String,
    access_token_duration: usize,
    refresh_token_duration: usize,
}

impl Jwt {
    /// Creates a new `Jwt` instance from the given configuration.
    pub fn new(cfg: JwtCfg) -> Self {
        Self::try_new(cfg).expect("invalid jwt config")
    }

    /// Creates a new `Jwt` instance from the given configuration.
    pub fn try_new(cfg: JwtCfg) -> Result<Self> {
        let (
            access_private_key_pem,
            access_public_key_pem,
            refresh_private_key_pem,
            refresh_public_key_pem,
        ) = resolve_key_material(&cfg)?;
        let encoding_access_key = EncodingKey::from_ed_pem(access_private_key_pem.as_bytes())?;
        let encoding_refresh_key = EncodingKey::from_ed_pem(refresh_private_key_pem.as_bytes())?;
        let decoding_access_key = DecodingKey::from_ed_pem(access_public_key_pem.as_bytes())?;
        let decoding_refresh_key = DecodingKey::from_ed_pem(refresh_public_key_pem.as_bytes())?;

        let header = Header::new(Algorithm::EdDSA);
        let mut validation_access_key = Validation::new(Algorithm::EdDSA);
        validation_access_key.set_issuer(std::slice::from_ref(&cfg.issuer));
        validation_access_key.set_audience(std::slice::from_ref(&cfg.audience));
        let mut validation_refresh_key = validation_access_key.clone();
        validation_access_key.validate_exp = cfg.access_key_validate_exp;
        validation_refresh_key.validate_exp = cfg.refresh_key_validate_exp;
        validation_refresh_key.required_spec_claims.clear();
        Ok(Self {
            header,
            encoding_access_key,
            encoding_refresh_key,
            decoding_access_key,
            decoding_refresh_key,
            validation_access_key,
            validation_refresh_key,
            iss: cfg.issuer,
            aud: cfg.audience,
            access_token_duration: cfg.access_token_duration,
            refresh_token_duration: cfg.refresh_token_duration,
        })
    }

    /// Generates a pair of access and refresh tokens.
    pub fn generate_token_pair(&self, sub: String, ext: Option<Value>) -> Result<TokenPair> {
        let access_token = self.generate_token(&TokenKind::Access, &sub, ext.clone())?;
        let refresh_token = self.generate_token(&TokenKind::Refesh, &sub, ext)?;
        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    /// Generates a pair of access and refresh tokens for subject only (`ext = None`).
    pub fn generate_token_pair_for_subject(&self, sub: String) -> Result<TokenPair> {
        self.generate_token_pair(sub, None)
    }

    /// Refreshes an access token using a refresh token.
    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String> {
        let claims = self.validate_refresh_token(refresh_token)?;
        self.generate_token(&TokenKind::Access, &claims.sub, claims.ext)
    }

    /// Validates an access token.
    pub fn validate_access_token(&self, token: &str) -> Result<Claims> {
        self.validate_token(&TokenKind::Access, token)
            .map(|data| data.claims)
    }

    /// Validates a refresh token.
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims> {
        self.validate_token(&TokenKind::Refesh, token)
            .map(|data| data.claims)
    }

    fn generate_token(&self, kind: &TokenKind, sub: &str, ext: Option<Value>) -> Result<String> {
        let duration = self.get_token_duration(kind);
        let (iat, exp) = self.generate_timestamps(duration);
        let key = self.select_encoding_key(kind);
        let claims = self.create_claims(sub, iat, exp, ext);
        encode(&self.header, &claims, key).map_err(|e| Error::AuthError(e.to_string().into()))
    }

    fn validate_token(&self, kind: &TokenKind, token: &str) -> Result<TokenData<Claims>> {
        let (key, validation) = self.select_decoding_key_and_validation(kind);
        decode::<Claims>(token, key, validation).map_err(|e| Error::AuthError(e.to_string().into()))
    }

    fn get_token_duration(&self, kind: &TokenKind) -> usize {
        match kind {
            TokenKind::Access => self.access_token_duration,
            TokenKind::Refesh => self.refresh_token_duration,
        }
    }

    fn generate_timestamps(&self, duration: usize) -> (usize, usize) {
        generate_expired_time(duration)
    }

    fn select_encoding_key(&self, kind: &TokenKind) -> &EncodingKey {
        match kind {
            TokenKind::Access => &self.encoding_access_key,
            TokenKind::Refesh => &self.encoding_refresh_key,
        }
    }

    fn create_claims(&self, sub: &str, iat: usize, exp: usize, ext: Option<Value>) -> Claims {
        Claims::new_with_ext(
            self.iss.clone(),
            self.aud.clone(),
            sub.to_string(),
            exp,
            iat,
            ext,
        )
    }

    fn select_decoding_key_and_validation(&self, kind: &TokenKind) -> (&DecodingKey, &Validation) {
        match kind {
            TokenKind::Access => (&self.decoding_access_key, &self.validation_access_key),
            TokenKind::Refesh => (&self.decoding_refresh_key, &self.validation_refresh_key),
        }
    }
}

impl AccessTokenVerifier for Jwt {
    fn validate_access_token(&self, token: &str) -> Result<Claims> {
        Jwt::validate_access_token(self, token)
    }
}

fn generate_expired_time(duration: usize) -> (usize, usize) {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now
        + Duration::try_seconds(i64::try_from(duration).expect("duration overflow"))
            .expect("duration out of range"))
    .timestamp() as usize;
    (iat, exp)
}

fn resolve_key_material(cfg: &JwtCfg) -> Result<(String, String, String, String)> {
    if let Some(dir) = cfg.key_dir.as_deref() {
        let dir = Path::new(dir);
        let access_private = read_key_file(dir, "access_private_key.pem")?;
        let access_public = read_key_file(dir, "access_public_key.pem")?;
        let refresh_private = read_key_file(dir, "refresh_private_key.pem")?;
        let refresh_public = read_key_file(dir, "refresh_public_key.pem")?;
        return Ok((
            access_private,
            access_public,
            refresh_private,
            refresh_public,
        ));
    }

    Ok((
        require_non_empty(
            cfg.access_private_key_pem.as_deref(),
            "access_private_key_pem",
        )?
        .to_string(),
        require_non_empty(
            cfg.access_public_key_pem.as_deref(),
            "access_public_key_pem",
        )?
        .to_string(),
        require_non_empty(
            cfg.refresh_private_key_pem.as_deref(),
            "refresh_private_key_pem",
        )?
        .to_string(),
        require_non_empty(
            cfg.refresh_public_key_pem.as_deref(),
            "refresh_public_key_pem",
        )?
        .to_string(),
    ))
}

fn read_key_file(dir: &Path, file_name: &str) -> Result<String> {
    let path = dir.join(file_name);
    fs::read_to_string(&path).map_err(|e| {
        Error::ErrorMessage(format!("failed to read key file {}: {e}", path.display()).into())
    })
}

fn require_non_empty<'a>(value: Option<&'a str>, field_name: &str) -> Result<&'a str> {
    value
        .filter(|s| !s.is_empty())
        .ok_or_else(|| Error::ErrorMessage(format!("missing required field: {field_name}").into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACCESS_PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIGrD/e7uKYqSY4twDEsRfMMuLSrODf14dpTiTK6K1YI0
-----END PRIVATE KEY-----";
    const ACCESS_PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA2+Jj2UvNCvQiUPNYRgSi0cJSPiJI6Rs6D0UTeEpQVj8=
-----END PUBLIC KEY-----";
    const REFRESH_PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIGrD/e7uKYqSY4twDEsRfMMuLSrODf14dpTiTK6K1YI0
-----END PRIVATE KEY-----";
    const REFRESH_PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA2+Jj2UvNCvQiUPNYRgSi0cJSPiJI6Rs6D0UTeEpQVj8=
-----END PUBLIC KEY-----";

    fn setup_jwt() -> Jwt {
        Jwt::new(JwtCfg {
            key_dir: None,
            access_private_key_pem: Some(ACCESS_PRIVATE_KEY_PEM.to_string()),
            access_public_key_pem: Some(ACCESS_PUBLIC_KEY_PEM.to_string()),
            refresh_private_key_pem: Some(REFRESH_PRIVATE_KEY_PEM.to_string()),
            refresh_public_key_pem: Some(REFRESH_PUBLIC_KEY_PEM.to_string()),
            issuer: "test_issuer".to_string(),
            audience: "test_audience".to_string(),
            access_token_duration: 3600,
            refresh_token_duration: 86400,
            access_key_validate_exp: true,
            refresh_key_validate_exp: true,
        })
    }

    #[test]
    fn test_generate_token_pair() {
        let jwt = setup_jwt();
        let token_pair = jwt
            .generate_token_pair("test_sub".to_string(), None)
            .unwrap();

        assert!(!token_pair.access_token.is_empty());
        assert!(!token_pair.refresh_token.is_empty());
    }

    #[test]
    fn test_validate_access_token() {
        let jwt = setup_jwt();
        let token_pair = jwt
            .generate_token_pair("test_sub".to_string(), None)
            .unwrap();
        let validation_result = jwt.validate_access_token(&token_pair.access_token);

        assert!(validation_result.is_ok());
        let claims = validation_result.unwrap();
        assert_eq!(claims.iss, "test_issuer");
        assert_eq!(claims.aud, "test_audience");
        assert_eq!(claims.sub, "test_sub");
    }

    #[test]
    fn test_validate_refresh_token() {
        let jwt = setup_jwt();
        let token_pair = jwt
            .generate_token_pair("test_sub".to_string(), None)
            .unwrap();
        let validation_result = jwt.validate_refresh_token(&token_pair.refresh_token);

        assert!(validation_result.is_ok());
        let claims = validation_result.unwrap();
        assert_eq!(claims.iss, "test_issuer");
        assert_eq!(claims.aud, "test_audience");
        assert_eq!(claims.sub, "test_sub");
    }

    #[test]
    fn test_key_dir_config() {
        use std::{
            fs,
            time::{SystemTime, UNIX_EPOCH},
        };

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("toolcraft_jwt_keys_{ts}"));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("access_private_key.pem"), ACCESS_PRIVATE_KEY_PEM).unwrap();
        fs::write(dir.join("access_public_key.pem"), ACCESS_PUBLIC_KEY_PEM).unwrap();
        fs::write(dir.join("refresh_private_key.pem"), REFRESH_PRIVATE_KEY_PEM).unwrap();
        fs::write(dir.join("refresh_public_key.pem"), REFRESH_PUBLIC_KEY_PEM).unwrap();

        let jwt = Jwt::new(JwtCfg {
            key_dir: Some(dir.to_string_lossy().to_string()),
            access_private_key_pem: None,
            access_public_key_pem: None,
            refresh_private_key_pem: None,
            refresh_public_key_pem: None,
            issuer: "test_issuer".to_string(),
            audience: "test_audience".to_string(),
            access_token_duration: 3600,
            refresh_token_duration: 86400,
            access_key_validate_exp: true,
            refresh_key_validate_exp: true,
        });

        let token_pair = jwt
            .generate_token_pair("test_sub".to_string(), None)
            .unwrap();
        let claims = jwt.validate_access_token(&token_pair.access_token).unwrap();
        assert_eq!(claims.sub, "test_sub");
    }

    #[test]
    fn test_refresh_access_token_keeps_ext() {
        let jwt = setup_jwt();
        let token_pair = jwt
            .generate_token_pair(
                "test_sub".to_string(),
                Some(serde_json::json!({"role":"admin"})),
            )
            .unwrap();
        let access_token = jwt.refresh_access_token(&token_pair.refresh_token).unwrap();
        let claims = jwt.validate_access_token(&access_token).unwrap();
        assert_eq!(claims.ext.unwrap()["role"], "admin");
    }
}
