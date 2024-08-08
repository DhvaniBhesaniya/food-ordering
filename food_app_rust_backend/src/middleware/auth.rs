use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};
use serde::Deserialize;
use crate::config::env::Config;

#[derive(Debug, Deserialize,Clone)]
struct Claims {
    id: String,
}

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    // Extract JWT token from headers
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.trim_start_matches("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Read the secret from config
    let config = Config::from_env();
    let jwt_secret = &config.jwt_secret;

    // Verify and decode token
    let token_data: TokenData<Claims> = decode(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Attach claims to the request if needed
    req.extensions_mut().insert(token_data.claims);

    // Proceed to the next middleware or handler
    Ok(next.run(req).await)
}
