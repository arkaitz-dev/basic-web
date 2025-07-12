use maud::{html, Markup, DOCTYPE};
use crate::{get_website_json_ld, get_person_json_ld, routes::Route};


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
            body hx-boost="true" {
                header {
                    div class="container" {
                        nav {
                            a class="logo"
                                href=(Route::Home.path())
                                hx-get=(Route::Home.path())
                                hx-target="main"
                                hx-push-url="true"
                            { "Tu Nombre" }

                            ul class="nav-links" id="nav-links" {
                                li {
                                    a href=(Route::About.path())
                                      hx-get=(Route::About.path())
                                      hx-target="main"
                                      hx-push-url="true"
                                      class={ @if current_section == "about" { "nav-link active" } @else { "nav-link" } }
                                    { "Sobre Mí" }
                                }
                                li {
                                    a href=(Route::Experience.path())
                                      hx-get=(Route::Experience.path())
                                      hx-target="main"
                                      hx-push-url="true"
                                      class={ @if current_section == "experience" { "nav-link active" } @else { "nav-link" } }
                                    { "Experiencia" }
                                }
                                li {
                                    a href=(Route::Projects.path())
                                      hx-get=(Route::Projects.path())
                                      hx-target="main"
                                      hx-push-url="true"
                                      class={ @if current_section == "projects" { "nav-link active" } @else { "nav-link" } }
                                    { "Proyectos" }
                                }
                                li {
                                    a href=(Route::Contact.path())
                                      hx-get=(Route::Contact.path())
                                      hx-target="main"
                                      hx-push-url="true"
                                      class={ @if current_section == "contact" { "nav-link active" } @else { "nav-link" } }
                                    { "Contacto" }
                                }
                            }

                            div class="nav-controls" {
                                button class="theme-toggle" id="theme-toggle" aria-label="Toggle theme" {
                                    svg class="theme-icon-sun" width="20" height="20" fill="currentColor" viewBox="0 0 20 20" {
                                        path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd" {}
                                    }
                                    svg class="theme-icon-moon" width="20" height="20" fill="currentColor" viewBox="0 0 20 20" style="display: none;" {
                                        path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" {}
                                    }
                                }

                                button class="mobile-menu-toggle" id="mobile-menu-toggle" aria-label="Toggle navigation menu" aria-expanded="false" {
                                    span {}
                                    span {}
                                    span {}
                                }
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

