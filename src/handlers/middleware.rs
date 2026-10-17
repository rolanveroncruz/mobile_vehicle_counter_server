use axum::{
    extract::Request,
    middleware::Next,
    response::IntoResponse,
    http::{header::AUTHORIZATION, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    // 1. Extract the Authorization header
    let auth_header = req.headers().get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 2. Ensure it starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header["Bearer ".len()..];

    // 3. Fetch the secret
    let jwt_secret = env::var("JWT_SECRET").map_err(|error| {
        eprintln!("Failed to read JWT_SECRET: {}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 4. Decode and validate the token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|error| {
        eprintln!("JWT validation failed: {}", error);
        StatusCode::UNAUTHORIZED
    })?;

    // 5. (Optional but recommended) Inject the Claims into the request extensions.
    // If a specific handler ever DOES need to know the username, it can extract it later.
    req.extensions_mut().insert(token_data.claims);

    // 6. Token is valid! Pass the request to the next layer (the handler).
    Ok(next.run(req).await)
}