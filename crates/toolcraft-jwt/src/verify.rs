use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

use crate::{AccessTokenVerifier, Claims, Result, error::Error};

#[derive(Debug, Clone)]
pub struct VerifyJwtCfg {
    pub issuer: String,
    pub audience: String,
}

/// Minimal verifier for asymmetric Ed25519 JWT.
///
/// Only public key is required.
pub struct VerifyJwt {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl VerifyJwt {
    /// Create an Ed25519 verifier with public key PEM and claim validation config.
    pub fn new(public_key_pem: impl AsRef<[u8]>, cfg: VerifyJwtCfg) -> Result<Self> {
        let decoding_key = DecodingKey::from_ed_pem(public_key_pem.as_ref())?;
        let mut validation = Validation::new(Algorithm::EdDSA);
        if cfg.issuer.is_empty() {
            return Err(Error::ErrorMessage("issuer must not be empty".into()));
        }
        if cfg.audience.is_empty() {
            return Err(Error::ErrorMessage("audience must not be empty".into()));
        }
        validation.set_issuer(&[cfg.issuer]);
        validation.set_audience(&[cfg.audience]);
        validation.validate_aud = true;
        Ok(Self {
            decoding_key,
            validation,
        })
    }

    /// Validate token signature and standard claims based on default validation.
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map(|data| data.claims)
            .map_err(|e| Error::AuthError(e.to_string().into()))
    }
}

impl AccessTokenVerifier for VerifyJwt {
    fn validate_access_token(&self, token: &str) -> Result<Claims> {
        self.validate_token(token)
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{EncodingKey, Header, encode};

    use super::*;

    const PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIGrD/e7uKYqSY4twDEsRfMMuLSrODf14dpTiTK6K1YI0
-----END PRIVATE KEY-----";
    const PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA2+Jj2UvNCvQiUPNYRgSi0cJSPiJI6Rs6D0UTeEpQVj8=
-----END PUBLIC KEY-----";

    #[test]
    fn test_verify_with_public_key_only() {
        let claims = Claims::new(
            "test_issuer".to_string(),
            "test_audience".to_string(),
            "test_sub".to_string(),
            (chrono::Utc::now().timestamp() as usize) + 3600,
            chrono::Utc::now().timestamp() as usize,
        );
        let token = encode(
            &Header::new(Algorithm::EdDSA),
            &claims,
            &EncodingKey::from_ed_pem(PRIVATE_KEY_PEM.as_bytes()).unwrap(),
        )
        .unwrap();

        let verifier = VerifyJwt::new(
            PUBLIC_KEY_PEM,
            VerifyJwtCfg {
                issuer: "test_issuer".to_string(),
                audience: "test_audience".to_string(),
            },
        )
        .unwrap();
        let decoded = verifier.validate_token(&token).unwrap();
        assert_eq!(decoded.sub, "test_sub");
    }
}
