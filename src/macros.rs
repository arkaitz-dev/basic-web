/// Macro para generar handlers de páginas con renderizado dual y logging de desarrollo
/// 
/// Genera una función handler que:
/// - Para requests htmx: devuelve solo el contenido de la vista
/// - Para requests normales: devuelve página completa con layout
/// - En modo debug: loggea información detallada del request con colores y emojis
#[macro_export]
macro_rules! generate_page_handler {
    ($handler_name:ident, $section:literal, $view_module:ident) => {
        pub async fn $handler_name(
            _uri: axum::http::Uri,
            _method: axum::http::Method,
            _headers: axum::http::HeaderMap,
            axum_htmx::HxRequest(is_htmx): axum_htmx::HxRequest,
        ) -> impl axum::response::IntoResponse {
            // Logging only in debug mode (development)
            #[cfg(debug_assertions)]
            {
                use std::time::{SystemTime, UNIX_EPOCH};
                
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                let user_agent = _headers
                    .get("user-agent")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("Unknown");
                
                let referer = _headers
                    .get("referer")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("-");
                
                let request_type = if is_htmx { "HTMX" } else { "FULL" };
                let request_source = if user_agent.contains("htmx") {
                    "🔄"
                } else if user_agent.contains("Mozilla") {
                    "🌐"
                } else {
                    "🤖"
                };
                
                // Enhanced logging format: [timestamp] method uri [type] source - referer
                println!(
                    "\x1b[36m[{}]\x1b[0m \x1b[32m{}\x1b[0m \x1b[1m{}\x1b[0m \x1b[33m[{}]\x1b[0m {} - \x1b[90m{}\x1b[0m",
                    timestamp,
                    _method,
                    _uri,
                    request_type,
                    request_source,
                    if referer == "-" { "direct" } else { referer }
                );
                
                // Additional debug info for htmx requests
                if is_htmx {
                    if let Some(target) = _headers.get("hx-target").and_then(|v| v.to_str().ok()) {
                        println!("  \x1b[90m└─ Target: {}\x1b[0m", target);
                    }
                    if let Some(trigger) = _headers.get("hx-trigger").and_then(|v| v.to_str().ok()) {
                        println!("  \x1b[90m└─ Trigger: {}\x1b[0m", trigger);
                    }
                }
            }
            
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