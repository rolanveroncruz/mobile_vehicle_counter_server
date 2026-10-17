use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize };
use uuid::Uuid;
use crate::entities::location; // Adjust if your entity name is different
use crate::handlers::app_state::AppState;

#[derive(Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    // Add other fields here (e.g., description, coordinates) if your DB table requires them
}

/// GET /api/locations
pub async fn get_all_locations(
    State(state): State<AppState>,
) -> Result<Json<Vec<location::Model>>, StatusCode> {
    let all_locations = location::Entity::find()
        .all(&state.db_conn)
        .await
        .map_err(|error| {
            eprintln!("Failed to fetch locations: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(all_locations))
}

/// POST /api/locations
pub async fn create_location(
    State(state): State<AppState>,
    Json(payload): Json<CreateLocationRequest>,
) -> Result<(StatusCode, Json<location::Model>), StatusCode> {
    // Generate a new UUID string for the primary key
    let new_uuid = Uuid::new_v4().to_string();

    // Construct the active model for insertion
    let new_location = location::ActiveModel {
        id: Set(new_uuid),
        name: Set(payload.name),
        // Add other Set() mappings here if needed
        ..Default::default()
    };

    // Insert and immediately return the created model
    let created = new_location.insert(&state.db_conn).await.map_err(|error| {
        eprintln!("Failed to create location: {}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(created)))
}

/// DELETE /api/locations/:id
pub async fn delete_location(
    State(state): State<AppState>,
    Path(id): Path<String>, // Ensure this matches the ID type in your SeaORM Model (i64 or i32)
) -> Result<StatusCode, StatusCode> {
    let result = location::Entity::delete_by_id(id)
        .exec(&state.db_conn)
        .await
        .map_err(|error| {
            eprintln!("Failed to delete location: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND); // Return 404 if the ID didn't exist
    }

    Ok(StatusCode::NO_CONTENT) // Return 204 No Content on successful deletion
}