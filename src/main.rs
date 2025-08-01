use axum::{
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE, HeaderName},
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
mod macros;
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

async fn serve_robots() -> impl IntoResponse {
    ([(CONTENT_TYPE, "text/plain")], include_str!("../static/robots.txt"))
}

async fn serve_sitemap() -> impl IntoResponse {
    ([(CONTENT_TYPE, "application/xml")], include_str!("../static/sitemap.xml"))
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

    // Build our application with routes and security headers
    let app = Router::new()
        .route(Route::Home.path(), get(controllers::home::handler))
        .route(Route::About.path(), get(controllers::about::handler))
        .route(Route::Experience.path(), get(controllers::experience::handler))
        .route(Route::Projects.path(), get(controllers::projects::handler))
        .route(Route::Contact.path(), get(controllers::contact::handler))
        .route(Route::Contact.path(), post(controllers::contact::contact_submit))
        .route("/api/json-ld/website", get(json_ld_website))
        .route("/api/json-ld/person", get(json_ld_person))
        .route("/robots.txt", get(serve_robots))
        .route("/sitemap.xml", get(serve_sitemap))
        .nest_service("/static", static_service)
        .layer(ServiceBuilder::new()
            // Security headers
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("x-frame-options"),
                HeaderValue::from_static("DENY"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("x-content-type-options"),
                HeaderValue::from_static("nosniff"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("x-xss-protection"),
                HeaderValue::from_static("1; mode=block"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("referrer-policy"),
                HeaderValue::from_static("strict-origin-when-cross-origin"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("permissions-policy"),
                HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
            ))
        )
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
