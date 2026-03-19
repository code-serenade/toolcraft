use std::sync::Arc;

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode, header},
    middleware::Next,
    response::Response,
};
use serde_json::Value;
use toolcraft_jwt::AccessTokenVerifier;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub ext: Option<Value>,
}

pub async fn auth<T>(mut req: Request, next: Next) -> Result<Response, StatusCode>
where
    T: AccessTokenVerifier + 'static,
{
    let headers = req.headers();
    let token = parse_token(headers)?;
    let jwt = req
        .extensions()
        .get::<Arc<T>>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let claims = jwt
        .validate_access_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let auth_user = AuthUser {
        user_id: claims.sub,
        ext: claims.ext,
    };
    req.extensions_mut().insert(auth_user);

    Ok(next.run(req).await)
}

fn parse_token(headers: &HeaderMap) -> Result<String, StatusCode> {
    let authorization = headers
        .get(header::AUTHORIZATION)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_str = authorization
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let mut parts = auth_str.split_whitespace();
    match parts.next() {
        Some(scheme) if scheme.eq_ignore_ascii_case("bearer") => {}
        _ => return Err(StatusCode::UNAUTHORIZED),
    }

    let token = parts.next().ok_or(StatusCode::UNAUTHORIZED)?.trim();
    if token.is_empty() || parts.next().is_some() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(token.to_string())
}
