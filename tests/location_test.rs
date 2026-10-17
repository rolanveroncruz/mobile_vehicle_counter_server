use axum::{
    body::Body,
    http::{Request, header, StatusCode},
};
use tower::ServiceExt;
use serde_json::{json, Value};
use sea_orm::Database;
use std::env;
use dotenvy::dotenv;

// Import from your library crate
use mobile_vehicle_counter_server::handlers::app_state::AppState;
use mobile_vehicle_counter_server::server::build_app;

#[tokio::test]
async fn test_full_location_lifecycle() {
    // 1. Setup environment and DB
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn = Database::connect(&db_url).await.unwrap();
    let state = AppState { db_conn };

    // 2. Build the app using your production router configuration
    let app = build_app(state.clone()).with_state(state);

    // ==========================================
    // STEP 1: LOGIN TO GET TOKEN
    // ==========================================
    let login_req = json!({
        "username": "admin",
        "password": "asdf"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/api/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&login_req).unwrap()))
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let auth_response: Value = serde_json::from_slice(&body_bytes).unwrap();
    let token = auth_response["token"].as_str().unwrap().to_string();
    let auth_header = format!("Bearer {}", token);

    // ==========================================
    // STEP 2: CREATE A LOCATION
    // ==========================================
    let create_req = json!({
        "name": "Integration Test Camera",
        "description": "Test camera for automated testing"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/api/locations")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, &auth_header)
        .body(Body::from(serde_json::to_vec(&create_req).unwrap()))
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let created_location: Value = serde_json::from_slice(&body_bytes).unwrap();
    let location_id = created_location["id"].as_str().unwrap().to_string();

    // ==========================================
    // STEP 3: GET LOCATIONS & VERIFY EXISTENCE
    // ==========================================
    let req = Request::builder()
        .method("GET")
        .uri("/api/locations")
        .header(header::AUTHORIZATION, &auth_header)
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let locations: Vec<Value> = serde_json::from_slice(&body_bytes).unwrap();

    let exists = locations.iter().any(|loc| loc["id"] == location_id);
    assert!(exists, "Newly created location was not found in the GET array");

    // ==========================================
    // STEP 4: DELETE THE LOCATION
    // ==========================================
    let req = Request::builder()
        .method("DELETE")
        // Note: Your router is configured as /api/locations/{id}[cite: 6]
        // Ensure Axum 0.8 correctly maps this, otherwise change it to /api/locations/:id
        .uri(format!("/api/locations/{}", location_id))
        .header(header::AUTHORIZATION, &auth_header)
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // ==========================================
    // STEP 5: GET LOCATIONS & VERIFY DELETION
    // ==========================================
    let req = Request::builder()
        .method("GET")
        .uri("/api/locations")
        .header(header::AUTHORIZATION, &auth_header)
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let locations_after: Vec<Value> = serde_json::from_slice(&body_bytes).unwrap();

    let still_exists = locations_after.iter().any(|loc| loc["id"] == location_id);
    assert!(!still_exists, "Location was found in GET array after deletion");
}