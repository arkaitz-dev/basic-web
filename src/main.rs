#[macro_use]
extern crate rocket;

use maud::Markup;
use rocket::fs::FileServer;
use rocket::http::{Header, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::{form::Form, Config, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use std::fs;

mod views;

// Helper function to read JSON-LD files
fn read_json_ld_file(filename: &str) -> String {
    let path = format!("static/data/{}", filename);
    fs::read_to_string(&path).unwrap_or_else(|_| {
        eprintln!("Warning: Could not read JSON-LD file: {}", path);
        "{}".to_string()
    })
}

// Public function to get structured data
pub fn get_website_json_ld() -> String {
    read_json_ld_file("website.json")
}

pub fn get_person_json_ld() -> String {
    read_json_ld_file("person.json")
}

struct HtmxRequest(bool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HtmxRequest {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let is_htmx = req.headers().get_one("HX-Request").is_some();
        Outcome::Success(HtmxRequest(is_htmx))
    }
}

#[get("/")]
fn home_handler(htmx: HtmxRequest) -> Result<Markup, Status> {
    if htmx.0 {
        Ok(views::layout::render_home_content())
    } else {
        Ok(views::layout::render_page("home"))
    }
}

#[get("/about")]
fn about_handler(htmx: HtmxRequest) -> Result<Markup, Status> {
    if htmx.0 {
        Ok(views::about_view::render())
    } else {
        Ok(views::layout::render_page_with_content(
            "about",
            views::about_view::render(),
        ))
    }
}

#[get("/experience")]
fn experience_handler(htmx: HtmxRequest) -> Result<Markup, Status> {
    if htmx.0 {
        Ok(views::experience_view::render())
    } else {
        Ok(views::layout::render_page_with_content(
            "experience",
            views::experience_view::render(),
        ))
    }
}

#[get("/projects")]
fn projects_handler(htmx: HtmxRequest) -> Result<Markup, Status> {
    if htmx.0 {
        Ok(views::projects_view::render())
    } else {
        Ok(views::layout::render_page_with_content(
            "projects",
            views::projects_view::render(),
        ))
    }
}

#[get("/contact")]
fn contact_handler(htmx: HtmxRequest) -> Result<Markup, Status> {
    if htmx.0 {
        Ok(views::contact_view::render())
    } else {
        Ok(views::layout::render_page_with_content(
            "contact",
            views::contact_view::render(),
        ))
    }
}

#[derive(FromForm)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

#[get("/api/json-ld/website")]
fn json_ld_website() -> (rocket::http::ContentType, String) {
    (rocket::http::ContentType::JSON, get_website_json_ld())
}

#[get("/api/json-ld/person")]
fn json_ld_person() -> (rocket::http::ContentType, String) {
    (rocket::http::ContentType::JSON, get_person_json_ld())
}

#[post("/contact", data = "<form>")]
fn contact_post_handler(form: Form<ContactForm>, htmx: HtmxRequest) -> Result<Markup, Status> {
    let name = form.name.trim();
    let email = form.email.trim();
    let message = form.message.trim();

    if name.is_empty() || email.is_empty() || message.is_empty() {
        let error_content = views::contact_view::render_error("Todos los campos son obligatorios");
        return if htmx.0 {
            Ok(error_content)
        } else {
            Ok(views::layout::render_page_with_content(
                "contact",
                error_content,
            ))
        };
    }

    if !email.contains('@') {
        let error_content = views::contact_view::render_error("Por favor ingresa un email válido");
        return if htmx.0 {
            Ok(error_content)
        } else {
            Ok(views::layout::render_page_with_content(
                "contact",
                error_content,
            ))
        };
    }

    if message.len() < 10 {
        let error_content =
            views::contact_view::render_error("El mensaje debe tener al menos 10 caracteres");
        return if htmx.0 {
            Ok(error_content)
        } else {
            Ok(views::layout::render_page_with_content(
                "contact",
                error_content,
            ))
        };
    }

    let success_content = views::contact_view::render_success(&name);
    Ok(if htmx.0 {
        success_content
    } else {
        views::layout::render_page_with_content("contact", success_content)
    })
}

#[catch(404)]
fn not_found() -> Markup {
    views::error_view::not_found()
}

#[catch(500)]
fn internal_error() -> Markup {
    views::error_view::internal_error()
}

// Fairing for Cache-Control headers on static assets
pub struct StaticAssetCacheFairing;

#[rocket::async_trait]
impl Fairing for StaticAssetCacheFairing {
    fn info(&self) -> Info {
        Info {
            name: "Static Asset Cache Headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.uri().path().starts_with("/static") {
            response.set_header(Header::new(
                "Cache-Control",
                "public, max-age=31536000, immutable",
            ));
        }
    }
}

#[launch]
fn rocket() -> _ {
    use rocket::shield::{Permission, Shield};

    let figment = Config::figment()
        .merge(("address", "127.0.0.1"))
        .merge(("port", 3000));

    // Create Shield without the deprecated interest-cohort policy
    // Shield::default() already handles disabling interest-cohort.
    // We will add CSP to the default configuration.
    let shield = Shield::default().disable::<Permission>();

    rocket::custom(figment)
        .attach(shield)
        .attach(StaticAssetCacheFairing) // Add cache headers for static files
        .mount("/static", FileServer::from("static")) // Serve static files
        .mount(
            "/",
            routes![
                home_handler,
                about_handler,
                experience_handler,
                projects_handler,
                contact_handler,
                contact_post_handler,
                json_ld_website,
                json_ld_person
            ],
        )
        .register("/", catchers![not_found, internal_error])
}
