//! Hello World example for RustAPI
//!
//! Run with: cargo run -p hello-world
//!
//! Then visit: http://127.0.0.1:8080

use rustapi_rs::prelude::*;

// ============================================
// Response types
// ============================================

#[derive(Serialize, Schema)]
struct HelloResponse {
    message: String,
}

#[derive(Serialize, Schema)]
struct UserResponse {
    id: i64,
    name: String,
    email: String,
}

/// Request body with validation
#[derive(Deserialize, Validate, Schema)]
struct CreateUser {
    #[validate(length(min = 1, max = 100))]
    name: String,

    #[validate(email)]
    email: String,
}

#[derive(Deserialize, IntoParams)]
#[allow(dead_code)]
struct SearchParams {
    /// Search query
    pub q: String,
    #[param(minimum = 1)]
    pub page: Option<usize>,
}

// ============================================
// Handlers using attribute macros
// ============================================

/// Hello World endpoint
#[rustapi_rs::get("/")]
#[rustapi_rs::tag("General")]
#[rustapi_rs::summary("Hello World")]
async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, World!".to_string(),
    })
}

/// Health check endpoint
#[rustapi_rs::get("/health")]
#[rustapi_rs::tag("General")]
#[rustapi_rs::summary("Health Check")]
#[rustapi_rs::description("Returns 'OK' if the server is healthy")]
async fn health() -> &'static str {
    "OK"
}

/// Get user by ID
#[rustapi_rs::get("/users/{id}")]
#[rustapi_rs::tag("Users")]
#[rustapi_rs::summary("Get User")]
#[rustapi_rs::description("Retrieves a user by their unique ID")]
async fn get_user(Path(id): Path<i64>) -> Json<UserResponse> {
    Json(UserResponse {
        id,
        name: format!("User {}", id),
        email: format!("user{}@example.com", id),
    })
}

/// Create a new user with validation
#[rustapi_rs::post("/users")]
#[rustapi_rs::tag("Users")]
#[rustapi_rs::summary("Create User")]
#[rustapi_rs::description("Creates a new user. Validates name (1-100 chars) and email format. Returns 422 on validation failure.")]
async fn create_user(ValidatedJson(body): ValidatedJson<CreateUser>) -> Json<UserResponse> {
    Json(UserResponse {
        id: 1,
        name: body.name,
        email: body.email,
    })
}

/// Search users
#[rustapi_rs::get("/search")]
#[rustapi_rs::tag("Users")]
#[rustapi_rs::summary("Search users")]
async fn search_users(Query(_params): Query<SearchParams>) -> Json<UserResponse> {
    Json(UserResponse {
        id: 0,
        name: "Search Result".to_string(),
        email: "search@example.com".to_string(),
    })
}

// ============================================
// Main entry point
// ============================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 RustAPI Example Server");
    println!("Routes:");
    println!("  GET  /          - Hello World");
    println!("  GET  /health    - Health check");
    println!("  GET  /users/:id - Get user by ID");
    println!("  POST /users     - Create user (validates name & email)");
    println!("  GET  /docs      - Swagger UI");
    println!();

    // Phase 6 / zero-config: routes + schemas are auto-registered via macros.
    // Swagger UI is enabled at /docs by default (when built with swagger-ui feature).
    RustApi::auto().run("127.0.0.1:8080").await
}
