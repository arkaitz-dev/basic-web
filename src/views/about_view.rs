use maud::{html, Markup};

pub fn render() -> Markup {
    html! {
        section class="section" {
            div class="container" {
                div class="content-card" {
                    div class="about-grid" {
                        div class="about-text" {
                            h2 { "Sobre Mí" }
                            p { 
                                "Soy un desarrollador full stack con más de 5 años de experiencia "
                                "creando aplicaciones web robustas y escalables. Mi pasión por la "
                                "tecnología me lleva a estar siempre aprendiendo y adoptando las "
                                "últimas tendencias en desarrollo."
                            }
                            p {
                                "Me especializo en el desarrollo backend con Rust y Node.js, "
                                "así como en frontend con React y tecnologías web modernas. "
                                "También tengo experiencia en DevOps, bases de datos y "
                                "arquitectura de software."
                            }
                            p {
                                "Cuando no estoy programando, disfruto contribuyendo a proyectos "
                                "de código abierto, escribiendo artículos técnicos y mentoreando "
                                "a desarrolladores junior."
                            }
                        }
                        div class="about-image" {
                            div class="about-profile-placeholder" {
                                "👨‍💻"
                            }
                        }
                    }
                    
                    h3 class="skills-section-title" { "Habilidades Técnicas" }
                    div class="skills-grid" {
                        div class="skill-category" {
                            h3 { "Backend" }
                            ul class="skill-list" {
                                li { "Rust" }
                                li { "Node.js" }
                                li { "Python" }
                                li { "PostgreSQL" }
                                li { "MongoDB" }
                                li { "Redis" }
                            }
                        }
                        div class="skill-category" {
                            h3 { "Frontend" }
                            ul class="skill-list" {
                                li { "React" }
                                li { "TypeScript" }
                                li { "HTML5/CSS3" }
                                li { "Tailwind CSS" }
                                li { "htmx" }
                                li { "Vue.js" }
                            }
                        }
                        div class="skill-category" {
                            h3 { "DevOps & Tools" }
                            ul class="skill-list" {
                                li { "Docker" }
                                li { "Kubernetes" }
                                li { "AWS" }
                                li { "Git" }
                                li { "CI/CD" }
                                li { "Linux" }
                            }
                        }
                    }
                }
            }
        }
    }
}