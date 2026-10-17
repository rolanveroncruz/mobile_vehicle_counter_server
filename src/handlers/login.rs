use jsonwebtoken::{encode, EncodingKey, Header};
use argon2::{Argon2, password_hash::{ PasswordVerifier}};
use argon2::password_hash::phc::PasswordHash;
use chrono::{Utc, Duration};
use axum::{extract::State, Json, http::{ StatusCode}};
use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use crate::handlers::app_state::AppState;
use crate::entities::users;
use std::env;
#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // The subject (username or user ID)
    exp: usize,  // Expiration time
}
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // 1. Fetch the user using SeaORM
    let user = users::Entity::find()
        .filter(users::Column::UserName.eq(&payload.username))
        .filter(users::Column::IsActive.eq(true))
        .one(&state.db_conn)
        .await
        .map_err(|error| {
            eprintln!("Database Error fetching user: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let user_model = match user{
        Some(model)=>model,
        None => return Err(StatusCode::UNAUTHORIZED)
    };

    // 2. Verify the provided password against the stored Argon2 hash
    let parsed_hash = PasswordHash::new(&user_model.encrypted_password)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err(){
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 3. Generate the JWT Claims (Expires in 24 hours)
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: payload.username.clone(),
        exp: expiration,
    };

    // 5. Encode the token
    let jwt_secret = env::var("JWT_SECRET").map_err(|error|{
        eprintln!("Failed to read JWT_SECRET:{}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).map_err(|error| {
        eprintln!("JWT Encoding Error: {}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 6. Return the Bearer token
    Ok(Json(AuthResponse { token }))
}