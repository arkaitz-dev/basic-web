use maud::{html, Markup};

pub fn render() -> Markup {
    html! {
        section class="section" {
            div class="container" {
                div class="content-card" {
                    h2 class="projects-title" { "Proyectos Destacados" }
                    
                    div class="projects-grid" {
                        div class="project-card" {
                            div class="project-image" {
                                "🚀 E-Commerce Platform"
                            }
                            div class="project-content" {
                                h3 class="project-title" { "Plataforma E-Commerce Avanzada" }
                                p class="project-description" {
                                    "Desarrollo completo de una plataforma de comercio electrónico con "
                                    "procesamiento de pagos, gestión de inventario y panel de administración. "
                                    "Maneja más de 10,000 transacciones mensuales."
                                }
                                div class="project-tags" {
                                    span class="tag" { "Rust" }
                                    span class="tag" { "React" }
                                    span class="tag" { "PostgreSQL" }
                                    span class="tag" { "Stripe" }
                                    span class="tag" { "Docker" }
                                }
                                div class="project-links" {
                                    a href="/projects/demo/ecommerce-platform" class="project-link" { "Ver Demo" }
                                    a href="/projects/code/ecommerce-platform" class="project-link" { "Código" }
                                }
                            }
                        }
                        
                        div class="project-card" {
                            div class="project-image" {
                                "📊 Analytics Dashboard"
                            }
                            div class="project-content" {
                                h3 class="project-title" { "Dashboard de Analytics en Tiempo Real" }
                                p class="project-description" {
                                    "Sistema de análisis de datos en tiempo real con visualizaciones interactivas. "
                                    "Procesa millones de eventos por día y proporciona insights empresariales "
                                    "críticos a través de gráficos dinámicos."
                                }
                                div class="project-tags" {
                                    span class="tag" { "Node.js" }
                                    span class="tag" { "D3.js" }
                                    span class="tag" { "WebSockets" }
                                    span class="tag" { "MongoDB" }
                                    span class="tag" { "Redis" }
                                }
                                div class="project-links" {
                                    a href="/projects/demo/analytics-dashboard" class="project-link" { "Ver Demo" }
                                    a href="/projects/code/analytics-dashboard" class="project-link" { "Código" }
                                }
                            }
                        }
                        
                        div class="project-card" {
                            div class="project-image" {
                                "🎯 Task Manager"
                            }
                            div class="project-content" {
                                h3 class="project-title" { "Gestor de Tareas Colaborativo" }
                                p class="project-description" {
                                    "Aplicación de gestión de proyectos con funcionalidades de colaboración "
                                    "en tiempo real. Incluye chat integrado, notificaciones push y "
                                    "sincronización offline-first."
                                }
                                div class="project-tags" {
                                    span class="tag" { "Rust" }
                                    span class="tag" { "Yew" }
                                    span class="tag" { "WebAssembly" }
                                    span class="tag" { "PWA" }
                                    span class="tag" { "GraphQL" }
                                }
                                div class="project-links" {
                                    a href="/projects/demo/task-manager" class="project-link" { "Ver Demo" }
                                    a href="/projects/code/task-manager" class="project-link" { "Código" }
                                }
                            }
                        }
                        
                        div class="project-card" {
                            div class="project-image" {
                                "🌐 API Gateway"
                            }
                            div class="project-content" {
                                h3 class="project-title" { "Microservices API Gateway" }
                                p class="project-description" {
                                    "Gateway de alta performance para arquitectura de microservicios con "
                                    "rate limiting, autenticación, load balancing y monitoreo avanzado. "
                                    "Soporta más de 100,000 requests por segundo."
                                }
                                div class="project-tags" {
                                    span class="tag" { "Rust" }
                                    span class="tag" { "Tokio" }
                                    span class="tag" { "gRPC" }
                                    span class="tag" { "Kubernetes" }
                                    span class="tag" { "Prometheus" }
                                }
                                div class="project-links" {
                                    a href="/projects/demo/api-gateway" class="project-link" { "Ver Demo" }
                                    a href="/projects/code/api-gateway" class="project-link" { "Código" }
                                }
                            }
                        }
                        
                        div class="project-card" {
                            div class="project-image" {
                                "📱 Mobile App"
                            }
                            div class="project-content" {
                                h3 class="project-title" { "App Móvil de Fitness" }
                                p class="project-description" {
                                    "Aplicación móvil multiplataforma para tracking de ejercicios con "
                                    "integración de wearables, planes personalizados y comunidad social. "
                                    "Más de 50,000 usuarios activos."
                                }
                                div class="project-tags" {
                                    span class="tag" { "React Native" }
                                    span class="tag" { "TypeScript" }
                                    span class="tag" { "Node.js" }
                                    span class="tag" { "Firebase" }
                                    span class="tag" { "ML Kit" }
                                }
                                div class="project-links" {
                                    a href="/projects/demo/mobile-app" class="project-link" { "Ver Demo" }
                                    a href="/projects/code/mobile-app" class="project-link" { "Código" }
                                }
                            }
                        }
                        
                        div class="project-card" {
                            div class="project-image" {
                                "🤖 AI Tool"
                            }
                            div class="project-content" {
                                h3 class="project-title" { "Herramienta de IA para Code Review" }
                                p class="project-description" {
                                    "Sistema automatizado de revisión de código utilizando modelos de "
                                    "lenguaje grandes. Analiza calidad, seguridad y mejores prácticas, "
                                    "reduciendo el tiempo de review en un 60%."
                                }
                                div class="project-tags" {
                                    span class="tag" { "Python" }
                                    span class="tag" { "FastAPI" }
                                    span class="tag" { "OpenAI" }
                                    span class="tag" { "Docker" }
                                    span class="tag" { "GitHub Actions" }
                                }
                                div class="project-links" {
                                    a href="/projects/demo/ai-tool" class="project-link" { "Ver Demo" }
                                    a href="/projects/code/ai-tool" class="project-link" { "Código" }
                                }
                            }
                        }
                    }
                    
                    div class="opensource-section" {
                        h3 class="opensource-title" { "Contribuciones Open Source" }
                        div class="opensource-grid" {
                            div class="opensource-item" {
                                h4 class="opensource-item-title" { "Rocket Framework" }
                                p class="opensource-item-description" { "Contribuciones al framework web de Rust" }
                            }
                            div class="opensource-item" {
                                h4 class="opensource-item-title" { "Maud Templates" }
                                p class="opensource-item-description" { "Mejoras en el sistema de templates" }
                            }
                            div class="opensource-item" {
                                h4 class="opensource-item-title" { "htmx Library" }
                                p class="opensource-item-description" { "Documentación y ejemplos" }
                            }
                            div class="opensource-item" {
                                h4 class="opensource-item-title" { "Rust Ecosystem" }
                                p class="opensource-item-description" { "Múltiples crates y utilidades" }
                            }
                        }
                    }
                }
            }
        }
    }
}