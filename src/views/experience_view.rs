use maud::{html, Markup};

pub fn render() -> Markup {
    html! {
        section class="section" {
            div class="container" {
                div class="content-card" {
                    h2 class="experience-title" { "Experiencia Profesional" }
                    
                    div class="timeline" {
                        div class="timeline-item" {
                            div class="timeline-date" { "2022 - Presente" }
                            h3 class="timeline-title" { "Senior Full Stack Developer" }
                            div class="timeline-company" { "TechCorp Solutions" }
                            p { 
                                "Lidero el desarrollo de aplicaciones web de alto rendimiento utilizando Rust y React. "
                                "Implementé microservicios que mejoraron el rendimiento del sistema en un 40%. "
                                "Mentoreo a un equipo de 3 desarrolladores junior y coordino con equipos de diseño y producto."
                            }
                            ul class="timeline-item-list" {
                                li { "Desarrollo de APIs REST con Rocket/Axum" }
                                li { "Implementación de arquitectura de microservicios" }
                                li { "Optimización de bases de datos PostgreSQL" }
                                li { "Implementación de CI/CD con GitLab" }
                            }
                        }
                        
                        div class="timeline-item" {
                            div class="timeline-date" { "2020 - 2022" }
                            h3 class="timeline-title" { "Full Stack Developer" }
                            div class="timeline-company" { "StartupXYZ" }
                            p { 
                                "Desarrollé desde cero la plataforma principal de la empresa utilizando Node.js y React. "
                                "Participé en todas las fases del desarrollo, desde el diseño de la arquitectura hasta el despliegue. "
                                "La aplicación ahora maneja más de 10,000 usuarios activos mensuales."
                            }
                            ul class="timeline-item-list" {
                                li { "Desarrollo frontend con React y TypeScript" }
                                li { "APIs backend con Node.js y Express" }
                                li { "Integración con servicios de terceros" }
                                li { "Implementación de autenticación JWT" }
                            }
                        }
                        
                        div class="timeline-item" {
                            div class="timeline-date" { "2019 - 2020" }
                            h3 class="timeline-title" { "Frontend Developer" }
                            div class="timeline-company" { "Digital Agency Pro" }
                            p { 
                                "Especializado en crear interfaces de usuario atractivas y funcionales para clientes diversos. "
                                "Trabajé en más de 15 proyectos web, mejorando significativamente la experiencia de usuario "
                                "y las métricas de conversión."
                            }
                            ul class="timeline-item-list" {
                                li { "Desarrollo de sitios web responsivos" }
                                li { "Optimización de rendimiento web" }
                                li { "Integración con CMS (WordPress, Strapi)" }
                                li { "Implementación de diseños UX/UI" }
                            }
                        }
                        
                        div class="timeline-item" {
                            div class="timeline-date" { "2018 - 2019" }
                            h3 class="timeline-title" { "Junior Web Developer" }
                            div class="timeline-company" { "WebDev Solutions" }
                            p { 
                                "Comencé mi carrera profesional desarrollando sitios web corporativos y e-commerce. "
                                "Aprendí las bases del desarrollo web moderno y las mejores prácticas de la industria. "
                                "Participé en la migración de sistemas legacy a tecnologías modernas."
                            }
                            ul class="timeline-item-list" {
                                li { "HTML5, CSS3 y JavaScript vanilla" }
                                li { "Desarrollo de themes para WordPress" }
                                li { "Mantenimiento de aplicaciones PHP" }
                                li { "Testing y debugging de aplicaciones web" }
                            }
                        }
                    }
                    
                    div class="education-section" {
                        h3 class="education-section-title" { "Educación & Certificaciones" }
                        div class="education-grid" {
                            div class="education-item" {
                                h4 class="education-item-title" { "Ingeniería en Sistemas" }
                                p class="education-item-subtitle" { "Universidad Tecnológica" }
                                p class="education-item-date" { "2014 - 2018" }
                            }
                            div class="education-item" {
                                h4 class="education-item-title" { "AWS Certified Developer" }
                                p class="education-item-subtitle" { "Amazon Web Services" }
                                p class="education-item-date" { "2023" }
                            }
                            div class="education-item" {
                                h4 class="education-item-title" { "Rust Programming" }
                                p class="education-item-subtitle" { "The Rust Foundation" }
                                p class="education-item-date" { "2022" }
                            }
                        }
                    }
                }
            }
        }
    }
}