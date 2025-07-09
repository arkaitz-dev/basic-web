use maud::{html, Markup, DOCTYPE};
use crate::{get_website_json_ld, get_person_json_ld};

pub fn render_page(current_section: &str) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Tu Nombre - Desarrollador Full Stack" }
                meta name="description" content="Portfolio profesional de desarrollador full stack especializado en Rust, JavaScript y tecnologías web modernas";

                // Add this line
                meta name="robots" content="index, follow";

                // CSP via meta tag
                meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'; style-src 'self' 'sha256-bsV5JivYxvGywDAZ22EZJKBFip65Ng9xoJVLbBg7bdo='; object-src 'none'; base-uri 'self';";
                // Security headers are handled by Rocket Shield middleware

                // Structured Data (JSON-LD)
                script type="application/ld+json" {
                    (maud::PreEscaped(&get_website_json_ld()))
                }
                script type="application/ld+json" {
                    (maud::PreEscaped(&get_person_json_ld()))
                }

                // htmx
                // Styles
                // Preload stylesheets
                link rel="preload" href="/static/css/light.css" as="style";
                link rel="preload" href="/static/css/dark.css" as="style";

                link rel="stylesheet" href="/static/css/main.css";
                link id="theme-stylesheet" rel="stylesheet" href="/static/css/light.css";
                
                // Theme initialization script
                script src="/static/js/theme-init.js" {}
                
                // htmx
                script src="/static/js/htmx.min.js" defer {}
            }
            body {
                header {
                    nav class="container" {
                        a class="logo"
                            href="/"
                            hx-get="/"
                            hx-target="main"
                            hx-push-url="true"
                        { "Tu Nombre" }

                        div class="nav-right" {
                            ul class="nav-links" id="nav-links" {
                            li {
                                a hx-get="/about"
                                  hx-target="main"
                                  hx-push-url="true"
                                  class={ @if current_section == "about" { "active nav-link" } @else { "nav-link" } }
                                  href="/about"
                                { "Sobre Mí" }
                            }
                            li {
                                a hx-get="/experience"
                                  hx-target="main"
                                  hx-push-url="true"
                                  class={ @if current_section == "experience" { "active nav-link" } @else { "nav-link" } }
                                  href="/experience"
                                { "Experiencia" }
                            }
                            li {
                                a hx-get="/projects"
                                  hx-target="main"
                                  hx-push-url="true"
                                  class={ @if current_section == "projects" { "active nav-link" } @else { "nav-link" } }
                                  href="/projects"
                                { "Proyectos" }
                            }
                            li {
                                a hx-get="/contact"
                                  hx-target="main"
                                  hx-push-url="true"
                                  class={ @if current_section == "contact" { "active nav-link" } @else { "nav-link" } }
                                  href="/contact"
                                { "Contacto" }
                            }
                            }

                            button class="hamburger" id="hamburger-btn" aria-label="Toggle navigation menu" {
                                span class="visually-hidden" { "Toggle navigation menu" }
                                span {}
                                span {}
                                span {}
                            }

                            button class="theme-toggle-btn" id="theme-toggle-btn" aria-label="Toggle theme" {
                                span class="theme-icon sun-icon" { "☀️" }
                                span class="theme-icon moon-icon" { "🌙" }
                            }
                        }
                    }
                }

                main hx-history-elt {
                    @if current_section == "home" {
                        (render_home_section())
                    }
                }

                div class="htmx-indicator" {
                    div class="loading-spinner" {}
                }

                footer {
                    div class="container" {
                        p {
                            "Made by "
                            a href="https://arkaitz.dev" target="_arkaitzdev_website" rel="noopener noreferrer" { "ArkaitzDev" }
                        }
                    }
                }

                script src="/static/js/main.js" defer {}
            }
        }
    }
}

pub fn render_page_with_content(current_section: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Tu Nombre - Desarrollador Full Stack" }
                meta name="description" content="Portfolio profesional de desarrollador full stack especializado en Rust, JavaScript y tecnologías web modernas";

                // Add this line
                meta name="robots" content="index, follow";

                // CSP via meta tag
                meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'; style-src 'self' 'sha256-bsV5JivYxvGywDAZ22EZJKBFip65Ng9xoJVLbBg7bdo='; object-src 'none'; base-uri 'self';";

                // Security headers are handled by Rocket Shield middleware

                // Structured Data (JSON-LD)
                script type="application/ld+json" {
                    (maud::PreEscaped(&get_website_json_ld()))
                }
                script type="application/ld+json" {
                    (maud::PreEscaped(&get_person_json_ld()))
                }

                // htmx
                // Styles
                // Preload stylesheets
                link rel="preload" href="/static/css/light.css" as="style";
                link rel="preload" href="/static/css/dark.css" as="style";

                link rel="stylesheet" href="/static/css/main.css";
                link id="theme-stylesheet" rel="stylesheet" href="/static/css/light.css";
                
                // Theme initialization script
                script src="/static/js/theme-init.js" {}

                // htmx
                script src="/static/js/htmx.min.js" defer {}
            }
            body {
                header {
                    nav class="container" {
                        a class="logo"
                            href="/"
                            hx-get="/"
                            hx-target="main"
                            hx-push-url="true"
                        { "Tu Nombre" }

                        div class="nav-right" {
                            ul class="nav-links" id="nav-links" {
                            li {
                                a href="/about"
                                  hx-get="/about"
                                  hx-target="main"
                                  hx-push-url="true"
                                  class={ @if current_section == "about" { "active nav-link" } @else { "nav-link" } }
                                { "Sobre Mí" }
                            }
                            li {
                                a href="/experience"
                                  hx-get="/experience"
                                  hx-target="main"
                                  hx-push-url="true"
                                  class={ @if current_section == "experience" { "active nav-link" } @else { "nav-link" } }
                                { "Experiencia" }
                            }
                            li {
                                a href="/projects"
                                  hx-get="/projects"
                                  hx-target="main"
                                  hx-push-url="true"
                                  class={ @if current_section == "projects" { "active nav-link" } @else { "nav-link" } }
                                { "Proyectos" }
                            }
                            li {
                                a href="/contact"
                                  hx-get="/contact"
                                  hx-target="main"
                                  hx-push-url="true"
                                  class={ @if current_section == "contact" { "active nav-link" } @else { "nav-link" } }
                                { "Contacto" }
                            }
                            }

                            button class="hamburger" id="hamburger-btn" aria-label="Toggle navigation menu" {
                                span class="visually-hidden" { "Toggle navigation menu" }
                                span {}
                                span {}
                                span {}
                            }

                            button class="theme-toggle-btn" id="theme-toggle-btn" aria-label="Toggle theme" {
                                span class="theme-icon sun-icon" { "☀️" }
                                span class="theme-icon moon-icon" { "🌙" }
                            }
                        }
                    }
                }

                main hx-history-elt {
                    (content)
                }

                div class="htmx-indicator" {
                    div class="loading-spinner" {}
                }

                footer {
                    div class="container" {
                        p {
                            "Made by "
                            a href="https://arkaitz.dev" target="_arkaitzdev_website" { "ArkaitzDev" }
                        }
                    }
                }

                script src="/static/js/main.js" defer {}
            }
        }
    }
}

pub fn render_home_content() -> Markup {
    render_home_section()
}

fn render_home_section() -> Markup {
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
