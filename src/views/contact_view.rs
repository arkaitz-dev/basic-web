use maud::{html, Markup};

pub fn render() -> Markup {
    html! {
        section class="section" {
            div class="container" {
                div class="content-card" {
                    h2 class="contact-section-title" { "Contacto" }
                    
                    div class="contact-grid" {
                        div class="contact-info" {
                            h3 { "¡Hablemos!" }
                            p class="contact-info-text" {
                                "¿Tienes un proyecto en mente? ¿Buscas un desarrollador para tu equipo? "
                                "¡Me encantaría escuchar sobre tu idea y cómo podemos trabajar juntos!"
                            }
                            
                            div class="contact-item" {
                                strong { "📧 Email:" }
                                span { "tu.email@ejemplo.com" }
                            }
                            div class="contact-item" {
                                strong { "📱 Teléfono:" }
                                span { "+34 123 456 789" }
                            }
                            div class="contact-item" {
                                strong { "📍 Ubicación:" }
                                span { "Madrid, España" }
                            }
                            div class="contact-item" {
                                strong { "💼 LinkedIn:" }
                                a href="https://linkedin.com/in/tu-perfil" class="contact-link" { "linkedin.com/in/tu-perfil" }
                            }
                            div class="contact-item" {
                                strong { "🐙 GitHub:" }
                                a href="https://github.com/tu-usuario" class="contact-link" { "github.com/tu-usuario" }
                            }
                            
                            div class="contact-info-section" {
                                h4 class="contact-info-section-title" { "¿Por qué trabajar conmigo?" }
                                h4 class="contact-info-section-title" { "¿Por qué trabajar conmigo?" }
                                ul class="contact-info-list" {
                                    li class="contact-info-list-item" { "✅ Código limpio y bien documentado" }
                                    li class="contact-info-list-item" { "✅ Comunicación clara y constante" }
                                    li class="contact-info-list-item" { "✅ Entrega puntual de proyectos" }
                                    li class="contact-list-item-custom" { "✅ Tecnologías modernas y escalables" }
                                    li class="contact-list-item-custom" { "✅ Soporte post-lanzamiento" }
                                }
                            }
                        }
                        
                        div class="contact-form" {
                            h3 class="contact-form-title" { "Envíame un mensaje" }
                            
                            div id="contact-response" {
                                // Response will be loaded here
                            }
                            
                            form hx-post="/contact" hx-target="#contact-response" hx-swap="innerHTML" {
                                div class="form-group" {
                                    label for="name" { "Nombre *" }
                                    input type="text" id="name" name="name" required="true" placeholder="Tu nombre completo" autocomplete="name";
                                }
                                
                                div class="form-group" {
                                    label for="email" { "Email *" }
                                    input type="email" id="email" name="email" required="true" placeholder="tu@email.com" autocomplete="email";
                                }
                                
                                div class="form-group" {
                                    label for="message" { "Mensaje *" }
                                    textarea id="message" name="message" required="true" placeholder="Cuéntame sobre tu proyecto o cómo puedo ayudarte..." autocomplete="on" {}
                                }
                                
                                button type="submit" class="cta-button contact-form-submit-button" {
                                    "Enviar Mensaje"
                                }
                            }
                            
                            p class="contact-form-footer-text" {
                                "Te responderé en menos de 24 horas"
                            }
                        }
                    }
                    
                    div class="availability-section" {
                        h3 class="availability-section-title" { "Disponibilidad" }
                        div class="availability-grid" {
                            div class="availability-item availability-item-available" {
                                h4 class="availability-item-title" { "🟢 Disponible" }
                                p class="availability-item-description" { "Para proyectos freelance" }
                            }
                            div class="availability-item availability-item-consultancy" {
                                h4 class="availability-item-title" { "🟡 Consultoría" }
                                p class="availability-item-description" { "Revisión de código y arquitectura" }
                            }
                            div class="availability-item availability-item-mentoring" {
                                h4 class="availability-item-title" { "🔵 Mentoring" }
                                p class="availability-item-description" { "Para desarrolladores junior" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_success(name: &str) -> Markup {
    html! {
        div class="alert alert-success" {
            h4 class="alert-title" { "¡Mensaje enviado con éxito!" }
            p { "Gracias " (name) ", he recibido tu mensaje y te responderé muy pronto." }
            p class="alert-text-mt" { 
                "Mientras tanto, puedes seguirme en "
                a href="https://linkedin.com/in/tu-perfil" class="alert-link" { "LinkedIn" }
                " o revisar mis proyectos en "
                a href="https://github.com/tu-usuario" class="contact-github-link" { "GitHub" }
                "."
            }
        }
        
        div class="alert-action-container" {
            p { "¿Quieres enviar otro mensaje?" }
            button hx-get="/contact" hx-target="#contact-response" hx-swap="innerHTML" class="cta-button alert-button" {
                "Nuevo mensaje"
            }
        }
    }
}

pub fn render_error(error_message: &str) -> Markup {
    html! {
        div class="alert alert-error" {
            h4 class="alert-title" { "Error al enviar el mensaje" }
            p { (error_message) }
        }
        
        div class="alert-action-container" {
            p { "Por favor, corrige el error e inténtalo de nuevo:" }
            button hx-get="/contact" hx-target="#contact-response" hx-swap="innerHTML" class="cta-button alert-button" {
                "Reintentar"
            }
        }
    }
}