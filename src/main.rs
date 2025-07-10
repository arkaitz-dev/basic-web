use axum::{
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        HeaderValue,
    },
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_htmx::AutoVaryLayer;
use maud::Markup;
use std::fs;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};

mod controllers;
mod routes;
mod views;
use routes::Route;

// Helper function to read JSON-LD files
fn read_json_ld_file(filename: &str) -> String {
    // Try multiple possible paths for JSON-LD files
    let possible_paths = [
        format!("static/data/{}", filename),   // Local development
        format!("/static/data/{}", filename),  // Docker container
        format!("./static/data/{}", filename), // Alternative local
    ];

    for path in &possible_paths {
        if let Ok(content) = fs::read_to_string(path) {
            return content;
        }
    }

    "{}".to_string()
}

// Public function to get structured data
pub fn get_website_json_ld() -> String {
    read_json_ld_file("website.json")
}

pub fn get_person_json_ld() -> String {
    read_json_ld_file("person.json")
}

// Helper function to convert Maud Markup to Html response
fn into_html_response(markup: Markup) -> Html<String> {
    Html(markup.into_string())
}

async fn json_ld_website() -> impl IntoResponse {
    ([(CONTENT_TYPE, "application/json")], get_website_json_ld())
}

async fn json_ld_person() -> impl IntoResponse {
    ([(CONTENT_TYPE, "application/json")], get_person_json_ld())
}

async fn not_found() -> impl IntoResponse {
    into_html_response(views::error_view::not_found())
}

#[tokio::main]
async fn main() {
    // Create static file service with cache headers
    let static_service = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        ))
        .service(ServeDir::new("static"));

    // Build our application with routes
    let app = Router::new()
        .route(Route::Home.path(), get(controllers::home_handler))
        .route("/about", get(controllers::about_handler))
        .route("/experience", get(controllers::experience_handler))
        .route("/projects", get(controllers::projects_handler))
        .route("/contact", get(controllers::contact_handler))
        .route("/contact", post(controllers::contact_submit))
        .route("/api/json-ld/website", get(json_ld_website))
        .route("/api/json-ld/person", get(json_ld_person))
        .nest_service("/static", static_service)
        .layer(AutoVaryLayer)
        .fallback(not_found);

    // Configure bind address based on build mode
    let bind_addr = if cfg!(debug_assertions) {
        "127.0.0.1:3000" // Development: localhost only
    } else {
        "0.0.0.0:3000" // Production: all interfaces
    };

    // Run the server
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();

    println!("Server running on http://{}", bind_addr);
    axum::serve(listener, app).await.unwrap();
}
