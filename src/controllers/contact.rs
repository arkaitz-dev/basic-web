use crate::generate_page_handler;
use crate::views;
use axum::{extract::Form, response::{Html, IntoResponse}};
use axum_htmx::HxRequest;
use maud::Markup;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}

fn into_html_response(markup: Markup) -> Html<String> {
    Html(markup.into_string())
}

generate_page_handler!(handler, "contact", contact_view);

pub async fn contact_submit(
    HxRequest(is_htmx): HxRequest,
    Form(form): Form<ContactForm>,
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
        into_html_response(views::layout::render_page_with_content(
            "contact",
            success_content,
        ))
    }
}
