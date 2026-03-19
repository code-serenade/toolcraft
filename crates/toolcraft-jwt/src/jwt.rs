use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};

use crate::{AccessTokenVerifier, Result, error::Error};

/// Struct representing the JWT configuration parameters.
#[derive(Debug, Deserialize)]
pub struct JwtCfg {
    pub access_private_key_pem: String,
    pub access_public_key_pem: String,
    pub refresh_private_key_pem: String,
    pub refresh_public_key_pem: String,
    pub audience: String,
    pub access_token_duration: usize,
    pub refresh_token_duration: usize,
    pub access_key_validate_exp: bool,
    pub refresh_key_validate_exp: bool,
}

/// Represents the JWT claims.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    /// Creates a new `Claims` instance.
    pub fn new(aud: String, sub: String, exp: usize, iat: usize) -> Self {
        Self { aud, sub, exp, iat }
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
        let encoding_access_key = EncodingKey::from_ed_pem(cfg.access_private_key_pem.as_bytes())?;
        let encoding_refresh_key =
            EncodingKey::from_ed_pem(cfg.refresh_private_key_pem.as_bytes())?;
        let decoding_access_key = DecodingKey::from_ed_pem(cfg.access_public_key_pem.as_bytes())?;
        let decoding_refresh_key = DecodingKey::from_ed_pem(cfg.refresh_public_key_pem.as_bytes())?;

        let header = Header::new(Algorithm::EdDSA);
        let mut validation_access_key = Validation::new(Algorithm::EdDSA);
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
            aud: cfg.audience,
            access_token_duration: cfg.access_token_duration,
            refresh_token_duration: cfg.refresh_token_duration,
        })
    }

    /// Generates a pair of access and refresh tokens.
    pub fn generate_token_pair(&self, sub: String) -> Result<(String, String)> {
        let access_token = self.generate_token(&TokenKind::Access, &sub)?;
        let refresh_token = self.generate_token(&TokenKind::Refesh, &sub)?;
        Ok((access_token, refresh_token))
    }

    /// Generates an access token.
    pub fn generate_access_token(&self, sub: String) -> Result<String> {
        self.generate_token(&TokenKind::Access, &sub)
    }

    /// Refreshes an access token using a refresh token.
    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String> {
        let claims = self.validate_refresh_token(refresh_token)?;
        self.generate_access_token(claims.sub)
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

    fn generate_token(&self, kind: &TokenKind, sub: &str) -> Result<String> {
        let duration = self.get_token_duration(kind);
        let (iat, exp) = self.generate_timestamps(duration);
        let key = self.select_encoding_key(kind);
        let claims = self.create_claims(sub, iat, exp);
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

    fn create_claims(&self, sub: &str, iat: usize, exp: usize) -> Claims {
        Claims::new(self.aud.clone(), sub.to_string(), exp, iat)
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
            access_private_key_pem: ACCESS_PRIVATE_KEY_PEM.to_string(),
            access_public_key_pem: ACCESS_PUBLIC_KEY_PEM.to_string(),
            refresh_private_key_pem: REFRESH_PRIVATE_KEY_PEM.to_string(),
            refresh_public_key_pem: REFRESH_PUBLIC_KEY_PEM.to_string(),
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
        let (access_token, refresh_token) =
            jwt.generate_token_pair("test_sub".to_string()).unwrap();

        assert!(!access_token.is_empty());
        assert!(!refresh_token.is_empty());
    }

    #[test]
    fn test_generate_access_token() {
        let jwt = setup_jwt();
        let access_token = jwt.generate_access_token("test_sub".to_string()).unwrap();

        assert!(!access_token.is_empty());
    }

    #[test]
    fn test_validate_access_token() {
        let jwt = setup_jwt();
        let access_token = jwt.generate_access_token("test_sub".to_string()).unwrap();
        let validation_result = jwt.validate_access_token(&access_token);

        assert!(validation_result.is_ok());
        let claims = validation_result.unwrap();
        assert_eq!(claims.aud, "test_audience");
        assert_eq!(claims.sub, "test_sub");
    }

    #[test]
    fn test_validate_refresh_token() {
        let jwt = setup_jwt();
        let (_, refresh_token) = jwt.generate_token_pair("test_sub".to_string()).unwrap();
        let validation_result = jwt.validate_refresh_token(&refresh_token);

        assert!(validation_result.is_ok());
        let claims = validation_result.unwrap();
        assert_eq!(claims.aud, "test_audience");
        assert_eq!(claims.sub, "test_sub");
    }
}
