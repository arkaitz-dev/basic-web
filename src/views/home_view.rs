use maud::{html, Markup};
use crate::routes::Route;

pub fn render() -> Markup {
    html! {
        section class="hero" {
            div class="container" {
                div class="hero-content" {
                    h1 class="hero-title" { "Hola, soy Tu Nombre" }
                    p class="hero-subtitle" { "Desarrollador Full Stack" }
                    p class="hero-description" {
                        "Especializado en crear aplicaciones web modernas y eficientes utilizando "
                        "tecnologías como Rust, JavaScript, React, y Node.js. "
                        "Apasionado por el código limpio y las mejores prácticas de desarrollo."
                    }
                    a href=(Route::Contact.path())
                      hx-get=(Route::Contact.path())
                      hx-target="main"
                      hx-push-url="true"
                      class="btn btn-primary"
                    { "Contactar" }
                }
            }
        }
    }
}