use maud::{html, Markup};

pub fn render() -> Markup {
    html! {
        section class="section" {
            div class="container" {
                div class="content-card hero" {
                    h1 { "Hola, soy Tu Nombre" }
                    p class="subtitle" { "Desarrollador Full Stack" }
                    p {
                        "Especializado en crear aplicaciones web modernas y eficientes utilizando "
                        "tecnologías como Rust, JavaScript, React, y Node.js. "
                        "Apasionado por el código limpio y las mejores prácticas de desarrollo."
                    }
                    a href="/contact"
                      hx-get="/contact"
                      hx-target="main"
                      hx-push-url="true"
                      class="cta-button"
                    { "Contactar" }
                }
            }
        }
    }
}