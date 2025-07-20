use crate::{get_person_json_ld, get_website_json_ld, routes::Route};
use maud::{html, Markup, DOCTYPE};

pub fn render_page_with_content(current_section: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Tu Nombre - Desarrollador Full Stack" }
                meta name="description" content="Portfolio profesional de desarrollador full stack especializado en Rust, JavaScript y tecnologías web modernas";
                meta name="keywords" content="desarrollador full stack, rust, javascript, programación, portfolio, desarrollo web";
                meta name="author" content="Tu Nombre";
                meta name="robots" content="index, follow";
                meta name="language" content="es";
                meta name="revisit-after" content="7 days";

                // Open Graph tags
                meta property="og:type" content="website";
                meta property="og:title" content="Tu Nombre - Desarrollador Full Stack";
                meta property="og:description" content="Portfolio profesional de desarrollador full stack especializado en Rust, JavaScript y tecnologías web modernas";
                meta property="og:url" content="https://tudominio.com";
                meta property="og:site_name" content="Tu Nombre Portfolio";
                meta property="og:locale" content="es_ES";

                // Twitter Cards
                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:title" content="Tu Nombre - Desarrollador Full Stack";
                meta name="twitter:description" content="Portfolio profesional de desarrollador full stack especializado en Rust, JavaScript y tecnologías web modernas";
                meta name="twitter:creator" content="@tuusuario";

                // Favicon and app icons
                link rel="icon" type="image/x-icon" href="/static/favicon.ico";
                link rel="icon" type="image/png" sizes="32x32" href="/static/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/static/favicon-16x16.png";
                link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
                link rel="manifest" href="/static/site.webmanifest";
                meta name="theme-color" content="#2563eb";
                meta name="msapplication-TileColor" content="#2563eb";

                // Canonical URL
                link rel="canonical" href="https://tudominio.com";

                // CSP via meta tag (security headers now via HTTP headers)
                meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self'; font-src 'self'; object-src 'none'; media-src 'self'; frame-src 'none'; base-uri 'self'; form-action 'self'";

                // Structured Data (JSON-LD)
                script type="application/ld+json" {
                    (maud::PreEscaped(&get_website_json_ld()))
                }
                script type="application/ld+json" {
                    (maud::PreEscaped(&get_person_json_ld()))
                }

                // Preload critical resources (optimized to prevent unused warnings)
                link rel="preload" href="/static/css/electric-eclipse/main.css" as="style";
                link rel="preload" href="/static/css/electric-eclipse/light.css" as="style";
                link rel="preload" href="/static/js/htmx.min.js" as="script";
                link rel="dns-prefetch" href="//fonts.googleapis.com";

                // Stylesheets
                link rel="stylesheet" href="/static/css/electric-eclipse/main.css";
                link id="theme-stylesheet" rel="stylesheet" href="/static/css/electric-eclipse/light.css";

                // Scripts
                script src="/static/js/theme-init.js" defer {}
                script src="/static/js/htmx.min.js" defer {}
            }
            body hx-boost="true" {
                // Dedicated background layer for Chrome Android fix
                div class="background-layer" aria-hidden="true" {}
                
                header {
                    div class="container" {
                        nav {
                            a class="logo"
                                href=(Route::Home.path())
                                hx-get=(Route::Home.path())
                                hx-target="main"
                                hx-push-url="true"
                            { "Tu Nombre" }
                            
                            div class="htmx-indicator" {
                                div class="loading-spinner" {}
                            }

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
