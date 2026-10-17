#![allow(dead_code,unused)]
use dotenvy::dotenv;
use std::env;

use axum::{
    body::Bytes,
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::http::Method;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use sea_orm::{Database, Iden};

use uuid::Uuid;
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::app_state::AppState;
use crate::handlers::login::login_handler;
#[derive(Debug, Deserialize)]
struct AlprPayload {
    camid: String,
    date: String,
    plate: String,
    plate_image: String,
    image: String,
}




/// Start the alarm server.
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // ------------------------
    // 1. Configure the CORS layer
    // ------------------------
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    // ------------------------
    // 2. Read the DATABASE_URL and connect
    // ------------------------
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let db_conn = Database::connect(&db_url).await?;
    let state = AppState {
        db_conn,
    };

    // -----------------------
    // 3. Setup axum server
    // -----------------------
    let app = Router::new()
        .route( "/api/login", post(login_handler))
        .layer(cors)
        .with_state(state);

    println!("Alarm server listening on http://127.0.0.1:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}



/// Receive an ALPR event from the camera.
async fn alpr_webhook_handler(
    State(state): State<AppState>,
    _headers: header::HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
}

fn decode_and_save_images_to_files(state:&AppState,
                                   event_id: &str,
                                   payload_image: &str,
                                   plate_image:  &str,
)->Result<(String, String), std::io::Error>{
    // ------------------------
    // 1. Decode the scene image.
    // -------------------------
/*
    let image_bytes = match STANDARD.decode(payload_image) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("Failed to decode scene image: {}", error);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, error));

        }
    };

    // ------------------------
    // 2.Decode the plate image.
    // ------------------------
    let plate_image_bytes = match STANDARD.decode(plate_image) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("Failed to decode plate image: {}", error);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
        }
    };

    //------------------
    // 3. Set the relevant variables.
    //-------------------
    let image_filename = format!("event_{}.jpg", event_id);
    let plate_image_filename = format!("event_{}_plate.jpg", event_id);

    let image_path = state.out_dir.join(&image_filename);
    let plate_image_path = state.out_dir.join(&plate_image_filename);

    //------------------
    // 4. Save scene image.
    //-------------------
    if let Err(error) = std::fs::write(&image_path, image_bytes) {
        eprintln!("Failed to save scene image: {}", error);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
    }

    // -----------------
    // 5. Save plate image.
    //------------------
    if let Err(error) = std::fs::write(&plate_image_path, plate_image_bytes) {
        eprintln!("Failed to save plate image: {}", error);

        // ✅ Don't leave an orphaned scene image.
        let _ = std::fs::remove_file(&image_path);

        return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
    }
    Ok((image_filename, plate_image_filename))
*/
    Ok(("hello".to_string(), "world".to_string()))
}

#[derive(Debug, Serialize)]
struct AlprRecord {
    id: String,
    camid: String,
    date: String,
    plate: String,
    image: String,
    plate_image: String,
}
/// Return all ALPR events as a JSON array.
async fn get_alpr_events(
    State(state): State<AppState>,
) -> Result<Json<Vec<AlprRecord>>, StatusCode> {
    Ok(Json(vec![]))
}