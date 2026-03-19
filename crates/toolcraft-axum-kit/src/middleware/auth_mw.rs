use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::Next,
    response::Response,
};
use toolcraft_jwt::AccessTokenVerifier;

#[derive(Debug, Clone)]
pub struct UserId(pub String);

pub async fn auth<T>(
    State(jwt): State<Arc<T>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode>
where
    T: AccessTokenVerifier,
{
    let headers = req.headers();
    let token = parse_token(headers)?;
    let claims = jwt
        .validate_access_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let user_id = UserId(claims.sub);
    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}

fn parse_token(headers: &HeaderMap) -> Result<String, StatusCode> {
    let authorization = headers
        .get(header::AUTHORIZATION)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_str = authorization
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let mut parts = auth_str.splitn(2, ' ');
    match parts.next() {
        Some("Bearer") => {}
        _ => return Err(StatusCode::UNAUTHORIZED),
    }

    let token = parts.next().ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(token.to_string())
}
