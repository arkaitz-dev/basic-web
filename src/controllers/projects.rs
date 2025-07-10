use crate::views;
use axum::response::{Html, IntoResponse};
use axum_htmx::HxRequest;
use maud::Markup;

fn into_html_response(markup: Markup) -> Html<String> {
    Html(markup.into_string())
}

pub async fn handler(HxRequest(is_htmx): HxRequest) -> impl IntoResponse {
    if is_htmx {
        into_html_response(views::projects_view::render())
    } else {
        into_html_response(views::layout::render_page_with_content(
            "projects",
            views::projects_view::render(),
        ))
    }
}
