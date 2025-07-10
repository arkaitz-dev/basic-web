use axum::{
    extract::Form,
    http::{HeaderValue, header::{CACHE_CONTROL, CONTENT_TYPE}},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_htmx::{HxRequest, AutoVaryLayer};
use maud::Markup;
use serde::Deserialize;
use std::fs;
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    set_header::SetResponseHeaderLayer,
};

mod views;

// Helper function to read JSON-LD files
fn read_json_ld_file(filename: &str) -> String {
    // Try multiple possible paths for JSON-LD files
    let possible_paths = [
        format!("static/data/{}", filename),  // Local development
        format!("/static/data/{}", filename), // Docker container
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

async fn home_handler(HxRequest(is_htmx): HxRequest) -> impl IntoResponse {
    if is_htmx {
        into_html_response(views::home_view::render())
    } else {
        into_html_response(views::layout::render_page_with_content(
            "home",
            views::home_view::render(),
        ))
    }
}

async fn about_handler(HxRequest(is_htmx): HxRequest) -> impl IntoResponse {
    if is_htmx {
        into_html_response(views::about_view::render())
    } else {
        into_html_response(views::layout::render_page_with_content(
            "about",
            views::about_view::render(),
        ))
    }
}

async fn experience_handler(HxRequest(is_htmx): HxRequest) -> impl IntoResponse {
    if is_htmx {
        into_html_response(views::experience_view::render())
    } else {
        into_html_response(views::layout::render_page_with_content(
            "experience",
            views::experience_view::render(),
        ))
    }
}

async fn projects_handler(HxRequest(is_htmx): HxRequest) -> impl IntoResponse {
    if is_htmx {
        into_html_response(views::projects_view::render())
    } else {
        into_html_response(views::layout::render_page_with_content(
            "projects",
            views::projects_view::render(),
        ))
    }
}

async fn contact_handler(HxRequest(is_htmx): HxRequest) -> impl IntoResponse {
    if is_htmx {
        into_html_response(views::contact_view::render())
    } else {
        into_html_response(views::layout::render_page_with_content(
            "contact",
            views::contact_view::render(),
        ))
    }
}

#[derive(Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

async fn json_ld_website() -> impl IntoResponse {
    ([(CONTENT_TYPE, "application/json")], get_website_json_ld())
}

async fn json_ld_person() -> impl IntoResponse {
    ([(CONTENT_TYPE, "application/json")], get_person_json_ld())
}

async fn contact_post_handler(
    HxRequest(is_htmx): HxRequest,
    Form(form): Form<ContactForm>
) -> impl IntoResponse {
    let name = form.name.trim();
    let email = form.email.trim();
    let message = form.message.trim();

    if name.is_empty() || email.is_empty() || message.is_empty() {
        let error_content = views::contact_view::render_error("Todos los campos son obligatorios");
        return if is_htmx {
            into_html_response(error_content)
        } else {
            into_html_response(views::layout::render_page_with_content(
                "contact",
                error_content,
            ))
        };
    }

    if !email.contains('@') {
        let error_content = views::contact_view::render_error("Por favor ingresa un email válido");
        return if is_htmx {
            into_html_response(error_content)
        } else {
            into_html_response(views::layout::render_page_with_content(
                "contact",
                error_content,
            ))
        };
    }

    if message.len() < 10 {
        let error_content =
            views::contact_view::render_error("El mensaje debe tener al menos 10 caracteres");
        return if is_htmx {
            into_html_response(error_content)
        } else {
            into_html_response(views::layout::render_page_with_content(
                "contact",
                error_content,
            ))
        };
    }

    let success_content = views::contact_view::render_success(&name);
    if is_htmx {
        into_html_response(success_content)
    } else {
        into_html_response(views::layout::render_page_with_content("contact", success_content))
    }
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
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/experience", get(experience_handler))
        .route("/projects", get(projects_handler))
        .route("/contact", get(contact_handler))
        .route("/contact", post(contact_post_handler))
        .route("/api/json-ld/website", get(json_ld_website))
        .route("/api/json-ld/person", get(json_ld_person))
        .nest_service("/static", static_service)
        .layer(AutoVaryLayer)
        .fallback(not_found);

    // Configure bind address based on build mode
    let bind_addr = if cfg!(debug_assertions) {
        "127.0.0.1:3000"  // Development: localhost only
    } else {
        "0.0.0.0:3000"    // Production: all interfaces
    };
    
    // Run the server
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .unwrap();
    
    println!("Server running on http://{}", bind_addr);
    axum::serve(listener, app).await.unwrap();
}
