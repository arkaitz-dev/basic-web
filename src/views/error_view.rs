use maud::{html, Markup, DOCTYPE};
use crate::routes::Route;

pub fn not_found() -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "404 - Página no encontrada | Tu Nombre" }
                meta http-equiv="X-Content-Type-Options" content="nosniff";
                meta http-equiv="X-Frame-Options" content="DENY";
                meta http-equiv="X-XSS-Protection" content="1; mode=block";
                script src="https://unpkg.com/htmx.org@2.0.3" integrity="sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq" crossorigin="anonymous" {}
                link rel="stylesheet" href="/static/css/main.css";
            }
            body {
                main class="error-main" {
                    section class="section error-section" {
                        div class="container" {
                            div class="content-card error-container" {
                                h1 { "404" }
                                h2 class="error-title" { "Página no encontrada" }
                                p { "Lo sentimos, la página que buscas no existe o ha sido movida." }
                                div class="error-button-container" {
                                    a href=(Route::Home.path()) class="cta-button" { "← Volver al inicio" }
                                }
                                div class="error-icon-container" { "🔍" }
                            }
                        }
                    }
                }
            }
        }
    }
}

