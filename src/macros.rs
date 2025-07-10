/// Macro para generar handlers de páginas con renderizado dual
/// 
/// Genera una función handler que:
/// - Para requests htmx: devuelve solo el contenido de la vista
/// - Para requests normales: devuelve página completa con layout
#[macro_export]
macro_rules! generate_page_handler {
    ($handler_name:ident, $section:literal, $view_module:ident) => {
        pub async fn $handler_name(axum_htmx::HxRequest(is_htmx): axum_htmx::HxRequest) -> impl axum::response::IntoResponse {
            fn into_html_response(markup: maud::Markup) -> axum::response::Html<String> {
                axum::response::Html(markup.into_string())
            }
            
            let content = crate::views::$view_module::render();
            
            if is_htmx {
                into_html_response(content)
            } else {
                into_html_response(crate::views::layout::render_page_with_content(
                    $section,
                    content,
                ))
            }
        }
    };
}